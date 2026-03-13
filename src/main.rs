#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #![allow(unused)]
use anyhow::Result;
use app::{App, UserEvent};
use conf::Config;
use std::time::Duration;
use tray_icon::{TrayIconEvent, menu::MenuEvent};
use winit::event_loop::{EventLoop, EventLoopProxy};

mod app;
mod conf;
mod icon;
mod trash;

// Embedding icons
static DEFAULT_ICONS: [&[u8]; 5] = [
    include_bytes!("../icons/0.ico"),
    include_bytes!("../icons/25.ico"),
    include_bytes!("../icons/50.ico"),
    include_bytes!("../icons/75.ico"),
    include_bytes!("../icons/100.ico"),
];

fn main() -> Result<()> {
    // Handle event
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;

    // Handle touch icon event
    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::TrayIconEvent(event));
    }));

    // Handle touch menu event
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    // Handle recycle bin size
    let proxy = event_loop.create_proxy();
    start_tray_updater(proxy);

    let _menu_channel = MenuEvent::receiver();
    let _tray_channel = TrayIconEvent::receiver();

    let mut conf = Config::default();
    conf.read()?;

    let mut app = App::new(DEFAULT_ICONS, conf);

    if let Err(err) = event_loop.run_app(&mut app) {
        println!("Error: {err:?}");
    }

    Ok(())
}

fn start_tray_updater(proxy: EventLoopProxy<UserEvent>) {
    std::thread::spawn(move || -> ! {
        loop {
            std::thread::sleep(Duration::from_secs_f32(2.5));

            let (size, items) = trash::recyle_bin_size().unwrap_or_default();
            proxy.send_event(UserEvent::UpdateTray(size, items)).ok();
        }
    });
}
