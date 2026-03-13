use crate::{
    Config,
    icon::{load_icon_bytes, load_menu_icon_bytes},
    trash::{clear_trash, open_trash},
};
use tray_icon::{
    MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{AboutMetadata, IconMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
};
use winit::application::ApplicationHandler;

const TOOLTIP: &str = "Minibin 1.0.0";

pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
    UpdateTray(i64, i64),
}

pub struct App<'a> {
    // Config
    conf: Config,
    // Current index in default_icons
    current_index: usize,
    // App icon
    tray_icon: Option<TrayIcon>,
    // Default icons in app
    default_icons: [&'a [u8]; 5],
}

impl<'a> App<'a> {
    pub fn new(default_icons: [&'a [u8]; 5], conf: Config) -> App<'a> {
        App {
            conf,
            default_icons,
            current_index: 0,
            tray_icon: None,
        }
    }

    fn create_metadata() -> AboutMetadata {
        AboutMetadata {
            name: Some("Minibin".into()),
            version: Some("1.0.0".into()),
            short_version: None,
            authors: Some(vec!["IwantHappiness".into()]),
            comments: None,
            copyright: None,
            license: Some("MIT".into()),
            website: Some("site-4suj.onrender.com".into()),
            website_label: None,
            credits: None,
            icon: None,
        }
    }

    fn new_tray_icon(&self) -> Option<TrayIcon> {
        let icon = load_icon_bytes(&self.default_icons[0]);
        let app = TrayIconBuilder::new()
            .with_menu(Box::new(self.new_tray_menu()))
            .with_tooltip(TOOLTIP)
            .with_icon(icon)
            .build()
            .ok()?;

        app.set_show_menu_on_left_click(false);

        Some(app)
    }

    fn new_tray_menu(&self) -> Menu {
        let sep = PredefinedMenuItem::separator();

        let open = MenuItem::new(&self.conf.translate.open, true, None);
        let empty = MenuItem::new(&self.conf.translate.empty, true, None);
        let exit = MenuItem::new(&self.conf.translate.exit, true, None);
        let about = PredefinedMenuItem::about(
            Some(&self.conf.translate.about),
            Some(App::create_metadata()),
        );

        let reset_icons = MenuItem::new(&self.conf.translate.configure_icons_reset, true, None);
        let empty_icons = IconMenuItem::new(
            &self.conf.translate.empty,
            true,
            Some(load_menu_icon_bytes(self.default_icons[0])),
            None,
        );

        let quarter = IconMenuItem::new(
            &self.conf.translate.configure_icons_25,
            true,
            Some(load_menu_icon_bytes(self.default_icons[1])),
            None,
        );
        let half = IconMenuItem::new(
            &self.conf.translate.configure_icons_50,
            true,
            Some(load_menu_icon_bytes(self.default_icons[2])),
            None,
        );
        let three_quartes = IconMenuItem::new(
            &self.conf.translate.configure_icons_75,
            true,
            Some(load_menu_icon_bytes(self.default_icons[3])),
            None,
        );
        let full = IconMenuItem::new(
            &self.conf.translate.configure_icons_full,
            true,
            Some(load_menu_icon_bytes(self.default_icons[4])),
            None,
        );
        let two_states = MenuItem::new(&self.conf.translate.configure_icons_two_state, true, None);

        let configure_icons = Submenu::with_items(
            &self.conf.translate.configure_icons,
            true,
            &[
                &two_states,
                &sep,
                &empty_icons,
                &quarter,
                &half,
                &three_quartes,
                &full,
                &sep,
                &reset_icons,
            ],
        )
        .unwrap();

        let configure = Submenu::with_items(
            &self.conf.translate.configure,
            true,
            &[&configure_icons, &sep, &about],
        )
        .unwrap();

        Menu::with_items(&[&open, &empty, &sep, &configure, &sep, &exit]).unwrap()
    }

    fn update_tray_icon(&mut self, size: i64, items: i64) {
        let tray = self.tray_icon.as_mut().unwrap();

        let (format, comfort_size) = match size {
            0..=1000 => ("Byte", size),
            1001..=1_000_000 => ("KB", size / 1024),
            1_000_001..=1_000_000_000 => ("MB", size / 1024 / 1024),
            _ => ("GB", size / 1024 / 1024 / 1024),
        };

        let tooltip = format!("{}\n\n{}{} {} files", TOOLTIP, comfort_size, format, items);
        tray.set_tooltip(Some(tooltip)).unwrap();

        let index = get_index_by_percent(size, self.conf.trash.max_fill_size_mb * 1024 * 1024, 5);

        if self.current_index != index {
            tray.set_icon(Some(load_icon_bytes(self.default_icons[index])))
                .unwrap();
        }
    }
}

impl ApplicationHandler<UserEvent> for App<'_> {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        if winit::event::StartCause::Init == cause {
            self.tray_icon = self.new_tray_icon();
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        if let UserEvent::TrayIconEvent(input) = &event
            && let TrayIconEvent::DoubleClick { button, .. } = input
            && MouseButton::Left == *button
        {
            clear_trash();
        };

        if let UserEvent::UpdateTray(size, items) = event {
            self.update_tray_icon(size, items);
        }

        if let UserEvent::MenuEvent(event) = event {
            if event.id == "1001" {
                open_trash();
            }

            if event.id == "1003" {
                event_loop.exit();
            }

            if event.id == "1002" {
                clear_trash();
            }

            dbg!(event.id);
        }
    }
}

fn get_index_by_percent(size: i64, max_size: u64, levels: usize) -> usize {
    if size == 0 || max_size == 0 {
        return 0;
    }

    let mut index = ((size as f64 / max_size as f64) * (levels as f64)).floor() as usize;
    if index >= levels {
        index = levels - 1;
    }
    index
}
