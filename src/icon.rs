use image::load_from_memory;
// use std::path::Path;
use tray_icon::{Icon, menu::Icon as MenuIcon};
// use std::path::Path;

// pub fn load_icon(path: &Path) -> Icon {
//     let (icon_rgba, icon_width, icon_height) = {
//         let image = image::open(path)
//             .expect("Failed to open icon path")
//             .into_rgba8();
//         let (width, height) = image.dimensions();
//         let rgba = image.into_raw();
//         (rgba, width, height)
//     };
//     Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
// }

pub fn load_icon_bytes(bytes: &[u8]) -> Icon {
    let image = load_from_memory(bytes).unwrap().into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Icon::from_rgba(rgba, width, height).unwrap()
}

pub fn load_menu_icon_bytes(bytes: &[u8]) -> MenuIcon {
    let image = load_from_memory(bytes).unwrap().into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    MenuIcon::from_rgba(rgba, width, height).unwrap()
}
