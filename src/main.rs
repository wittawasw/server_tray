#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod server;
mod thaiid;
mod card;
mod log;

use tray_icon::{
    TrayIconBuilder, Icon, TrayIcon,
    menu::{Menu, MenuItem, PredefinedMenuItem, MenuEvent, MenuId},
};
use tao::event_loop::{EventLoop, ControlFlow};
use std::{net::SocketAddr, path::PathBuf, sync::{Arc, Mutex}};
use crate::server::{ServerHandle, ServerConfig};

fn load_icon() -> Icon {
    log::write_log_line("Loading embedded icon");
    let img = image::load_from_memory(include_bytes!("../icon.ico"))
        .unwrap()
        .to_rgba8();
    let (w, h) = img.dimensions();
    log::write_log_line("Embedded icon loaded successfully");
    Icon::from_rgba(img.into_raw(), w, h).unwrap()
}

#[derive(Clone, Default)]
struct MenuIds {
    start: Option<MenuId>,
    stop: Option<MenuId>,
    open: Option<MenuId>,
    exit: Option<MenuId>,
}

#[derive(Default)]
struct MenuState {
    ids: MenuIds,
    menu: Option<Menu>,
}

fn build_menu(running: bool, addr: SocketAddr, ids: &mut MenuIds) -> Menu {
    let menu = Menu::new();

    if running {
        let host_label = format!("http://{}", addr);
        let m_host = MenuItem::new(&host_label, false, None);
        let m_open = MenuItem::new("Open in browser", true, None);
        let m_stop = MenuItem::new("Stop", true, None);
        let m_exit = MenuItem::new("Exit", true, None);

        ids.start = None;
        ids.open = Some(m_open.id().clone());
        ids.stop = Some(m_stop.id().clone());
        ids.exit = Some(m_exit.id().clone());

        menu.append(&m_host).unwrap();
        menu.append(&m_open).unwrap();
        menu.append(&m_stop).unwrap();
        menu.append(&PredefinedMenuItem::separator()).unwrap();
        menu.append(&m_exit).unwrap();

        log::write_log_line(&format!(
            "Menu(running) open:{:?} stop:{:?} exit:{:?}",
            m_open.id(), m_stop.id(), m_exit.id()
        ));
    } else {
        let m_start = MenuItem::new("Start", true, None);
        let m_exit = MenuItem::new("Exit", true, None);

        ids.start = Some(m_start.id().clone());
        ids.open = None;
        ids.stop = None;
        ids.exit = Some(m_exit.id().clone());

        menu.append(&m_start).unwrap();
        menu.append(&PredefinedMenuItem::separator()).unwrap();
        menu.append(&m_exit).unwrap();

        log::write_log_line(&format!(
            "Menu(stopped) start:{:?} exit:{:?}",
            m_start.id(), m_exit.id()
        ));
    }

    menu
}

fn refresh_menu(tray: &TrayIcon, running: bool, addr: SocketAddr, state: &Arc<Mutex<MenuState>>) {
    let mut st = state.lock().unwrap();
    let mut ids = MenuIds::default();
    let menu = build_menu(running, addr, &mut ids);
    tray.set_menu(Box::new(menu.clone())).unwrap(); // keep our own clone alive
    st.ids = ids;
    st.menu = Some(menu);
    log::write_log_line("Tray menu refreshed");
}

fn main() {
    log::write_log_line("App launched");

    let event_loop = EventLoop::new();
    log::write_log_line("Event loop created");

    let config = ServerConfig {
        static_dir: PathBuf::from("assets"),
        address: SocketAddr::from(([127, 0, 0, 1], 8080)),
    };

    let handle = ServerHandle::new(config);
    let card_listener = card::CardListener::new();

    let tray = TrayIconBuilder::new()
        .with_icon(load_icon())
        .with_tooltip("Rust Tray Server")
        .build()
        .unwrap();
    log::write_log_line("Tray built successfully");

    handle.start();
    card_listener.start();

    let state = Arc::new(Mutex::new(MenuState::default()));
    refresh_menu(&tray, handle.is_running(), handle.address(), &state);

    event_loop.run({
        let tray = tray.clone();
        let state = state.clone();
        move |_, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            // inside event_loop.run
        while let Ok(m_evt) = MenuEvent::receiver().try_recv() {
            log::write_log_line(&format!("Menu: {:?}", m_evt.id));

            let ids = { state.lock().unwrap().ids.clone() };

            let evt_id = &m_evt.id; // borrow, no move

            if Some(evt_id) == ids.start.as_ref() {
                handle.start();
                card_listener.start();
                refresh_menu(&tray, true, handle.address(), &state);
            } else if Some(evt_id) == ids.stop.as_ref() {
                card_listener.stop();
                handle.stop();
                refresh_menu(&tray, false, handle.address(), &state);
            } else if Some(evt_id) == ids.open.as_ref() {
                let url = format!("http://{}", handle.address());
                let _ = open::that(url);
            } else if Some(evt_id) == ids.exit.as_ref() {
                log::write_log_line("Exit clicked");
                card_listener.stop();
                handle.stop();
                std::process::exit(0);
            }
        }

        }
    });
}
