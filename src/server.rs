// server.rs
use askama::Template;
use std::{net::SocketAddr, path::PathBuf, sync::{Arc, Mutex}, thread};
use tokio::{runtime::Runtime, sync::oneshot};
use warp::{Filter, Rejection, Reply};

#[derive(Clone)]
pub struct ServerConfig {
    pub static_dir: PathBuf,
    pub address: SocketAddr,
}

#[derive(Clone)]
pub struct ServerHandle {
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    config: ServerConfig,
}

impl ServerHandle {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            handle: Arc::new(Mutex::new(None)),
            shutdown_tx: Arc::new(Mutex::new(None)),
            config,
        }
    }

    pub fn start(&self) {
        let mut handle_guard = self.handle.lock().unwrap();
        if handle_guard.is_some() { return }

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        *self.shutdown_tx.lock().unwrap() = Some(shutdown_tx);
        let config = self.config.clone();

        let h = thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let html_route = warp::path::end().and_then(render_home);

                let static_route = warp::path("assets")
                    .and(warp::fs::dir(config.static_dir.clone()));

                let routes = html_route.or(static_route);

                let (_, server) = warp::serve(routes)
                    .bind_with_graceful_shutdown(config.address, async {
                        let _ = shutdown_rx.await;
                    });

                server.await;
            });
        });

        *handle_guard = Some(h);
    }

    pub fn stop(&self) {
        let mut handle_guard = self.handle.lock().unwrap();
        let mut shutdown_guard = self.shutdown_tx.lock().unwrap();

        if let Some(tx) = shutdown_guard.take() {
            let _ = tx.send(());
        }

        if let Some(h) = handle_guard.take() {
            let _ = h.join();
        }
    }

    pub fn is_running(&self) -> bool {
        self.handle.lock().unwrap().is_some()
    }

    pub fn address(&self) -> SocketAddr {
        self.config.address
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate<'a> {
    application_name: &'a str,
}

async fn render_home() -> Result<impl Reply, Rejection> {
    let template = HomeTemplate { application_name: "Server Tray" };
    Ok(warp::reply::html(template.render().unwrap()))
}

pub fn run_blocking(config: ServerConfig) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let html_route = warp::path::end().and_then(render_home);

        let static_route = warp::path("assets")
            .and(warp::fs::dir(config.static_dir.clone()));

        let routes = html_route.or(static_route);

        let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(
            config.address,
            async {
                let _ = tokio::signal::ctrl_c().await;
            },
        );

        server.await;
    });
}
