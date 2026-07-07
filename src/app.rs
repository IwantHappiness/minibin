use crate::{
    Config,
    conf::DoubleClickAction,
    icon::{load_icon_bytes, load_menu_icon_bytes},
    trash::{clear_trash, open_trash},
};
use anyhow::Result;
use tray_icon::{
    MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{
        AboutMetadata, CheckMenuItem, IconMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu,
    },
};
use winit::application::ApplicationHandler;

const TOOLTIP: &str = "Minibin 0.1.0";

const EXIT_ITEM_ID: &str = "1";
const EMPTY_ITEM_ID: &str = "2";
const OPEN_ITEM_ID: &str = "3";
const RESET_ITEM_ID: &str = "4";
const SYSTEM_RECYCLE_ID: &str = "5";
const SYSTEM_PROGRES_ID: &str = "6";
const SYSTEM_SOUND_ID: &str = "7";
const CLICK_EMPTY_ID: &str = "8";
const CLICK_OPEN_ID: &str = "9";

pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
    UpdateTray(u64, u64),
    UpdateConfig,
}

pub struct App {
    // Config
    pub conf: Config,
    // Current index in default_icons
    current_index: usize,
    // App icon
    tray_icon: Option<TrayIcon>,
    // Default icons in app
    default_icons: [&'static [u8]; 5],
}

impl App {
    pub fn new(default_icons: [&'static [u8]; 5], conf: Config) -> App {
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
            version: Some("0.1.0".into()),
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
        let icon = load_icon_bytes(self.default_icons[0]);
        let app = TrayIconBuilder::new()
            .with_menu(Box::new(self.new_tray_menu()))
            .with_tooltip(TOOLTIP)
            .with_icon(icon)
            .build()
            .ok()?;

        app.set_show_menu_on_left_click(false);

        Some(app)
    }

    fn main_menu_item(
        &self,
        exti_id: &str,
        empty_id: &str,
        open_id: &str,
    ) -> (MenuItem, MenuItem, MenuItem) {
        let exit = MenuItem::with_id(exti_id, &self.conf.translate.exit, true, None);
        let empty = MenuItem::with_id(empty_id, &self.conf.translate.empty, true, None);
        let open = MenuItem::with_id(open_id, &self.conf.translate.open, true, None);

        (exit, empty, open)
    }

    fn configure_icon_items(&self, reset_item_id: &str) -> Result<Submenu> {
        let sep = PredefinedMenuItem::separator();

        let two_states = MenuItem::new(&self.conf.translate.configure_icons_two_state, true, None);

        let reset_icons = MenuItem::with_id(
            reset_item_id,
            &self.conf.translate.configure_icons_reset,
            true,
            None,
        );

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

        Ok(Submenu::with_items(
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
        )?)
    }

    fn click_configure_items(
        &self,
        click_empty_id: &str,
        click_open_id: &str,
    ) -> (CheckMenuItem, CheckMenuItem) {
        let click_configure_empty = CheckMenuItem::with_id(
            click_empty_id,
            &self.conf.translate.empty,
            true,
            self.conf.trash.double_click_actions == DoubleClickAction::Empty,
            None,
        );
        let click_configure_open = CheckMenuItem::with_id(
            click_open_id,
            &self.conf.translate.open,
            true,
            self.conf.trash.double_click_actions == DoubleClickAction::Open,
            None,
        );

        (click_configure_empty, click_configure_open)
    }

    fn system_intergrations_items(
        &self,
        recycle_id: &str,
        progress_id: &str,
        sound_id: &str,
    ) -> (CheckMenuItem, CheckMenuItem, CheckMenuItem) {
        let system_recycle = CheckMenuItem::with_id(
            recycle_id,
            &self.conf.translate.configure_system_confirm,
            true,
            self.conf.trash.recycle_no_confirm,
            None,
        );

        let system_progress = CheckMenuItem::with_id(
            progress_id,
            &self.conf.translate.configure_system_progress,
            true,
            self.conf.trash.recycle_no_progress,
            None,
        );

        let system_sound = CheckMenuItem::with_id(
            sound_id,
            &self.conf.translate.configure_system_sound,
            true,
            self.conf.trash.recycle_no_sound,
            None,
        );

        (system_recycle, system_sound, system_progress)
    }

    fn new_tray_menu(&self) -> Menu {
        let sep = PredefinedMenuItem::separator();

        let (exit, empty, open) = self.main_menu_item(EXIT_ITEM_ID, EMPTY_ITEM_ID, OPEN_ITEM_ID);

        let about = PredefinedMenuItem::about(
            Some(&self.conf.translate.about),
            Some(App::create_metadata()),
        );

        let (click_configure_empty, click_configure_open) =
            self.click_configure_items(CLICK_EMPTY_ID, CLICK_OPEN_ID);
        let click_configure = Submenu::with_items(
            &self.conf.translate.configure_double_click,
            true,
            &[&click_configure_empty, &click_configure_open],
        )
        .unwrap();

        let (system_recycle, system_sound, system_progress) =
            self.system_intergrations_items(SYSTEM_RECYCLE_ID, SYSTEM_PROGRES_ID, SYSTEM_SOUND_ID);
        let system_integration = Submenu::with_items(
            &self.conf.translate.configure_system,
            true,
            &[&system_recycle, &system_sound, &system_progress],
        )
        .unwrap();

        let configure_icons = self.configure_icon_items(RESET_ITEM_ID).unwrap();

        let configure = Submenu::with_items(
            &self.conf.translate.configure,
            true,
            &[
                &click_configure,
                &system_integration,
                &configure_icons,
                &sep,
                &about,
            ],
        )
        .unwrap();

        Menu::with_items(&[&open, &empty, &sep, &configure, &sep, &exit]).unwrap()
    }

    fn update_tray_icon(&mut self, size: u64, items: u64) {
        let tray = self.tray_icon.as_mut().unwrap();

        let (format, comfort_size) = match size {
            0..=1000 => ("Byte", size),
            1001..=1_000_000 => ("KB", size / 1024),
            1_000_001..=1_000_000_000 => ("MB", size / 1024 / 1024),
            _ => ("GB", size / 1024 / 1024 / 1024),
        };

        let tooltip = format!("{TOOLTIP}\n\n{comfort_size} {format} {items} files");
        tray.set_tooltip(Some(tooltip)).unwrap();

        let index = get_index_by_percent(size, self.conf.trash.max_fill_size_mb * 1024 * 1024, 5);

        if self.current_index != index {
            tray.set_icon(Some(load_icon_bytes(self.default_icons[index])))
                .unwrap();
        }
    }

    fn parse_flags_trash(&self) -> u32 {
        (self.conf.trash.recycle_no_confirm as u32) << 2
            | (self.conf.trash.recycle_no_progress as u32) << 1
            | (self.conf.trash.recycle_no_sound as u32)
    }
}

impl ApplicationHandler<UserEvent> for App {
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
        match event {
            UserEvent::TrayIconEvent(tray_icon_event @ TrayIconEvent::DoubleClick { .. }) if matches!(tray_icon_event, TrayIconEvent::DoubleClick { button, .. } if button == MouseButton::Left) => {
                if let TrayIconEvent::DoubleClick { button: _, .. } = tray_icon_event {
                    if self.conf.trash.double_click_actions == DoubleClickAction::Open {
                        open_trash();
                    } else {
                        clear_trash(self.parse_flags_trash());
                    }
                }
            }
            UserEvent::MenuEvent(menu_event) => match menu_event.id.as_ref() {
                OPEN_ITEM_ID => open_trash(),
                EMPTY_ITEM_ID => clear_trash(self.parse_flags_trash()),
                EXIT_ITEM_ID => {
                    self.conf.write().expect("Failed to write to config.");
                    event_loop.exit();
                }
                SYSTEM_RECYCLE_ID => {
                    self.conf.trash.recycle_no_confirm = !self.conf.trash.recycle_no_confirm
                }
                SYSTEM_PROGRES_ID => {
                    self.conf.trash.recycle_no_progress = !self.conf.trash.recycle_no_progress
                }
                SYSTEM_SOUND_ID => {
                    self.conf.trash.recycle_no_sound = !self.conf.trash.recycle_no_sound
                }
                CLICK_EMPTY_ID => self.conf.trash.double_click_actions = DoubleClickAction::Empty,
                CLICK_OPEN_ID => self.conf.trash.double_click_actions = DoubleClickAction::Open,
                _ => {}
            },
            UserEvent::UpdateTray(size, items) => self.update_tray_icon(size, items),
            UserEvent::UpdateConfig => self.conf.read().unwrap_or_else(|e| eprintln!("{e}")),
            _ => {}
        }
    }
}

fn get_index_by_percent(size: u64, max_size: u64, levels: usize) -> usize {
    if size == 0 || max_size == 0 {
        return 0;
    }

    // let ratio = size.min(max_size) * levels as u64 / max_size;
    // ratio.min((levels - 1) as u64) as usize

    let mut index = ((size as f64 / max_size as f64) * (levels as f64)).floor() as usize;
    if index >= levels {
        index = levels - 1;
    }
    index
}
