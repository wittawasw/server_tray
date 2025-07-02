mod server;

use tray_icon::{
    TrayIconBuilder, Icon,
    menu::{Menu, MenuItem, PredefinedMenuItem, MenuEvent},
};
use image::ImageReader;
use std::{thread, time::Duration};

fn load_icon(path: &str) -> Icon {
    let img = ImageReader::open(path).unwrap().decode().unwrap().to_rgba8();
    let (w, h) = img.dimensions();
    Icon::from_rgba(img.into_raw(), w, h).unwrap()
}

fn main() {
    let menu = Menu::new();
    let m_start = MenuItem::new("Start", true, None);
    let m_stop = MenuItem::new("Stop", true, None);
    menu.append(&m_start).unwrap();
    menu.append(&m_stop).unwrap();
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    let m_exit = MenuItem::new("Exit", true, None);
    menu.append(&m_exit).unwrap();

    let icon = load_icon("icon.png");
    let _tray = TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(menu.clone()))
        .with_tooltip("Rust Tray Server")
        .build()
        .unwrap();

    let handle = server::ServerHandle::new();

    // Correct ID comparison using as_ref()
    loop {
        if let Ok(m_evt) = MenuEvent::receiver().try_recv() {
            let id_str = m_evt.id.as_ref();
            if id_str == m_start.id().as_ref() {
                handle.start();
            } else if id_str == m_stop.id().as_ref() {
                handle.stop();
            } else if id_str == m_exit.id().as_ref() {
                std::process::exit(0);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
