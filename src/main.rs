#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod server;
mod log;

use tray_icon::{
    TrayIconBuilder, Icon,
    menu::{Menu, MenuItem, PredefinedMenuItem, MenuEvent},
};
use tao::event_loop::{EventLoop, ControlFlow};

fn load_icon() -> Icon {
    log::write_log_line("Loading embedded icon");
    let img = image::load_from_memory(include_bytes!("../icon.ico"))
        .unwrap()
        .to_rgba8();
    let (w, h) = img.dimensions();
    log::write_log_line("Embedded icon loaded successfully");
    Icon::from_rgba(img.into_raw(), w, h).unwrap()
}

fn main() {
    log::write_log_line("App launched");

    let event_loop = EventLoop::new();
    log::write_log_line("Event loop created");

    let menu = Menu::new();
    log::write_log_line("Menu created");

    let m_start = MenuItem::new("Start", true, None);
    let m_stop = MenuItem::new("Stop", true, None);
    let m_exit = MenuItem::new("Exit", true, None);

    log::write_log_line(&format!("Start ID: {:?}", m_start.id()));
    log::write_log_line(&format!("Stop ID: {:?}", m_stop.id()));
    log::write_log_line(&format!("Exit ID: {:?}", m_exit.id()));

    menu.append(&m_start).unwrap();
    menu.append(&m_stop).unwrap();
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    menu.append(&m_exit).unwrap();

    let _tray = TrayIconBuilder::new()
        .with_icon(load_icon())
        .with_menu(Box::new(menu.clone()))
        .with_tooltip("Rust Tray Server")
        .build()
        .unwrap();
    log::write_log_line("Tray built successfully");

    let handle = server::ServerHandle::new();

    event_loop.run(move |_, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        while let Ok(m_evt) = MenuEvent::receiver().try_recv() {
            log::write_log_line(&format!("Menu: {:?}", m_evt.id));

            if m_evt.id == m_start.id() {
                handle.start();
            } else if m_evt.id == m_stop.id() {
                handle.stop();
            } else if m_evt.id == m_exit.id() {
                log::write_log_line("Exit clicked");
                handle.stop();
                std::process::exit(0);
            }
        }
    });
}
