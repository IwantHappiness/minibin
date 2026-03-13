use anyhow::Result;
use config::{Config as ConfigBuilder, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

const CONFIG_FILE_NAME: &str = "minibin_rs.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub config_update_speed: u64,
    pub trash: TrashSettings,
    pub translate: Translate,
}

impl Config {
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

impl Default for Config {
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
    pub recycle_progress: bool,
    pub icons_two_states: bool,
    pub recycle_confirm: bool,
    pub recycle_sond: bool,
}

impl Default for TrashSettings {
    fn default() -> Self {
        Self {
            recycle_sond: true,
            recycle_confirm: true,
            recycle_progress: true,
            icons_two_states: false,
            max_fill_size_mb: 10240,
            double_click_open: false,
            non_adaptive_fill_size: 1,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Translate {
    pub open: String,
    pub exit: String,
    pub empty: String,
    pub about: String,
    pub configure: String,
    pub configure_icons: String,
    pub configure_system: String,
    pub configure_icons_25: String,
    pub configure_icons_50: String,
    pub configure_icons_75: String,
    pub configure_icons_full: String,
    pub configure_icons_empty: String,
    pub configure_icons_reset: String,
    pub configure_system_sound: String,
    pub configure_double_click: String,
    pub configure_system_confirm: String,
    pub configure_system_progress: String,
    pub configure_icons_two_state: String,
}

impl Default for Translate {
    fn default() -> Self {
        Self {
            open: "Open".into(),
            empty: "Empty".into(),
            configure: "Configure".into(),
            configure_double_click: "Icon Double-Click Action".into(),
            configure_system: "System Integration".into(),
            configure_system_confirm: "Confirm Recycling".into(),
            configure_system_sound: "Allow Sound".into(),
            configure_system_progress: "Allow Progress Window".into(),
            configure_icons: "Change Icons".into(),
            configure_icons_two_state: "Only use empty/full icons".into(),
            configure_icons_empty: "Empty".into(),
            configure_icons_25: "25%".into(),
            configure_icons_50: "50%".into(),
            configure_icons_75: "75%".into(),
            configure_icons_full: "Full".into(),
            configure_icons_reset: "Reset Icons".into(),
            about: "About".into(),
            exit: "Exit".into(),
        }
    }
}
