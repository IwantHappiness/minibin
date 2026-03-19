use anyhow::Result;
use config::{Config as ConfigBuilder, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

const CONFIG_FILE_NAME: &str = "minibin_rs.toml";

#[derive(Deserialize, Serialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Config<'a> {
    pub config_update_speed: u64,
    pub trash: TrashSettings,
    pub translate: Translate<'a>,
}

impl<'a> Config<'a> {
    pub fn write(&self) -> Result<()> {
        let conf = toml::to_string_pretty(&self)?;
        fs::write(CONFIG_FILE_NAME, conf)?;

        Ok(())
    }

    pub fn read(&mut self) -> Result<()> {
        let file = PathBuf::from(CONFIG_FILE_NAME);

        if !file.exists() {
            self.write()?;
            return Ok(());
        }

        let builder = ConfigBuilder::builder()
            .add_source(File::from(file).format(FileFormat::Toml).required(false))
            .build()?;

        if let Ok(conf) = builder.try_deserialize::<Config>() {
            *self = Config { ..conf };
        }
        Ok(())
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            config_update_speed: 1,
            translate: Translate::default(),
            trash: TrashSettings::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TrashSettings {
    pub max_fill_size_mb: u64,
    pub non_adaptive_fill_size: u8,
    pub double_click_open: bool,
    pub recycle_no_progress: bool,
    pub icons_two_states: bool,
    pub recycle_no_confirm: bool,
    pub recycle_no_sound: bool,
}

impl Default for TrashSettings {
    fn default() -> Self {
        Self {
            recycle_no_sound: false,
            recycle_no_confirm: false,
            recycle_no_progress: false,
            icons_two_states: false,
            max_fill_size_mb: 10240,
            double_click_open: false,
            non_adaptive_fill_size: 1,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Translate<'a> {
    pub open: &'a str,
    pub exit: &'a str,
    pub empty: &'a str,
    pub about: &'a str,
    pub configure: &'a str,
    pub configure_icons: &'a str,
    pub configure_system: &'a str,
    pub configure_icons_25: &'a str,
    pub configure_icons_50: &'a str,
    pub configure_icons_75: &'a str,
    pub configure_icons_full: &'a str,
    pub configure_icons_empty: &'a str,
    pub configure_icons_reset: &'a str,
    pub configure_system_sound: &'a str,
    pub configure_double_click: &'a str,
    pub configure_system_confirm: &'a str,
    pub configure_system_progress: &'a str,
    pub configure_icons_two_state: &'a str,
}

impl<'a> Default for Translate<'a> {
    fn default() -> Self {
        Self {
            open: "Open",
            empty: "Empty",
            configure: "Configure",
            configure_double_click: "Icon Double-Click Action",
            configure_system: "System Integration",
            configure_system_confirm: "Confirm Recycling",
            configure_system_sound: "Allow Sound",
            configure_system_progress: "Allow Progress Window",
            configure_icons: "Change Icons",
            configure_icons_two_state: "Only use empty/full icons",
            configure_icons_empty: "Empty",
            configure_icons_25: "25%",
            configure_icons_50: "50%",
            configure_icons_75: "75%",
            configure_icons_full: "Full",
            configure_icons_reset: "Reset Icons",
            about: "About",
            exit: "Exit",
        }
    }
}
