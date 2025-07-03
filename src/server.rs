use crate::log;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tokio::{runtime::Runtime, sync::oneshot};
use warp::Filter;

pub struct ServerHandle {
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl ServerHandle {
    pub fn new() -> Self {
        Self {
            handle: Arc::new(Mutex::new(None)),
            shutdown_tx: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self) {
        let mut handle_guard = self.handle.lock().unwrap();
        if handle_guard.is_some() { return }

        log::write_log_line("Starting server");

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        *self.shutdown_tx.lock().unwrap() = Some(shutdown_tx);

        let h = thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let route = warp::path::end().map(|| "Hello!");

                let (_addr, server) = warp::serve(route)
                    .bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), async {
                        shutdown_rx.await.ok();
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
            log::write_log_line("Shutdown signal sent");
        } else {
            log::write_log_line("Server not running");
            return;
        }

        if let Some(h) = handle_guard.take() {
            let _ = h.join();
            log::write_log_line("Server stopped");
        }
    }
}
