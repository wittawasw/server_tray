use std::{fs::OpenOptions, io::Write, path::PathBuf};

pub fn write_log_line(message: &str) {
    let exe_path = std::env::current_exe().unwrap();
    let mut log_path = PathBuf::from(exe_path.parent().unwrap());
    log_path.push("server_tray.log");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .unwrap();

    let timestamp = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    writeln!(file, "{} {}", timestamp, message).unwrap();
}
