use mini_bin_rus::{Message, clear_trash, open_trash};
use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

const NAME: &str = "MiniBinRus";

fn main() {
    let mut tray = TrayItem::new(NAME, IconSource::Resource("white-empty")).unwrap();
    let (tx, rx) = mpsc::sync_channel(1);

    let open_tx = tx.clone();
    tray.add_menu_item("Open", move || {
        open_tx.send(Message::Open).unwrap();
    })
    .unwrap();

    let clear_tx = tx.clone();
    tray.add_menu_item("Empty", move || {
        clear_tx.send(Message::Empty).unwrap();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    tray.add_menu_item("Settings", || {
        println!("Settings!");
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Exit", move || {
        quit_tx.send(Message::Exit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Exit) => {
                break;
            }

            Ok(Message::Open) => {
                open_trash();
            }

            Ok(Message::Empty) => {
                clear_trash();
            }

            _ => {}
        }
    }
}
