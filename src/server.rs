use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle}
};
use tokio::runtime::Runtime;
use warp::Filter;

pub struct ServerHandle(Arc<Mutex<Option<JoinHandle<()>>>>);

impl ServerHandle {
    pub fn new() -> Self {
        ServerHandle(Arc::new(Mutex::new(None)))
    }

    pub fn start(&self) {
        let mut opt = self.0.lock().unwrap();
        if opt.is_some() {
            println!("Already running");
            return;
        }
        let handle = thread::spawn(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                println!("Starting server on http://127.0.0.1:8080");
                let route = warp::path::end().map(|| "Hello!");
                warp::serve(route).run(([127,0,0,1],8080)).await;
            });
        });
        *opt = Some(handle);
    }

    pub fn stop(&self) {
        let mut opt = self.0.lock().unwrap();
        if opt.take().is_some() {
            println!("Stop requested (thread detached)");
        } else {
            println!("Server not running");
        }
    }
}
