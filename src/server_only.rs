#![cfg_attr(target_os = "windows", windows_subsystem = "console")]

mod server;
mod log;

use std::{net::SocketAddr, path::PathBuf};

fn main() {
    log::write_log_line("Server-only binary starting");

    let addr = std::env::var("BIND_ADDR")
        .ok()
        .and_then(|s| s.parse::<SocketAddr>().ok())
        .unwrap_or_else(|| SocketAddr::from(([0, 0, 0, 0], 8080)));

    let static_dir = std::env::var("STATIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("assets"));

    let cfg = server::ServerConfig {
        static_dir,
        address: addr,
    };

    println!("Listening on http://{}", cfg.address);
    server::run_blocking(cfg);
}
