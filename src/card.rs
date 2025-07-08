use crate::log;
use pcsc::{Context, Scope, Error};
use std::{sync::{Arc, Mutex}, thread, time::Duration};

pub struct CardListener {
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
    stop_flag: Arc<Mutex<bool>>,
}

impl CardListener {
    pub fn new() -> Self {
        Self {
            handle: Arc::new(Mutex::new(None)),
            stop_flag: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) {
        let mut h = self.handle.lock().unwrap();
        if h.is_some() { return }
        *self.stop_flag.lock().unwrap() = false;

        let stop = Arc::clone(&self.stop_flag);
        let handle = thread::spawn(move || {
            let ctx = match Context::establish(Scope::User) {
                Ok(c) => c,
                Err(e) => {
                    log::write_log_line(&format!("PCSC init failed: {}", e));
                    return;
                }
            };

            let mut buf = [0u8; 2048];
            loop {
                if *stop.lock().unwrap() {
                    log::write_log_line("Card listener stopped");
                    break;
                }

                match ctx.list_readers(&mut buf) {
                    Ok(mut names) => {
                        while let Some(reader_cstr) = names.next() {
                            let name = reader_cstr.to_string_lossy();
                            log::write_log_line(&format!("Reader: {}", name));
                        }
                    }
                    Err(Error::NoReadersAvailable) => {
                        log::write_log_line("No readers available");
                    }
                    Err(e) => {
                        log::write_log_line(&format!("Reader list failed: {}", e));
                    }
                }

                thread::sleep(Duration::from_secs(3));
            }
        });

        *h = Some(handle);
    }

    pub fn stop(&self) {
        *self.stop_flag.lock().unwrap() = true;
        if let Some(t) = self.handle.lock().unwrap().take() {
            let _ = t.join();
        }
    }
}
