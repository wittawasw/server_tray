use crate::log;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tokio::runtime::Runtime;
use warp::Filter;

pub struct ServerHandle(Arc<Mutex<Option<thread::JoinHandle<()>>>>);

impl ServerHandle {
    pub fn new() -> Self { ServerHandle(Arc::new(Mutex::new(None))) }

    pub fn start(&self) {
        let mut opt = self.0.lock().unwrap();
        if opt.is_some() { return }
        log::write_log_line("Starting server");
        let h = thread::spawn(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                warp::serve(warp::path::end().map(|| "Hello!"))
                    .run(([127,0,0,1], 8080)).await;
            });
        });
        *opt = Some(h);
    }

    pub fn stop(&self) {
        let mut opt = self.0.lock().unwrap();
        if opt.take().is_some() {
            log::write_log_line("Server stopped");
        } else {
            log::write_log_line("Server stopped (was not running)");
        }
    }
}
