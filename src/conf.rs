use anyhow::Result;
use config::{Config as ConfigBuilder, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

const CONFIG_FILE_NAME: &str = "minibin_rs.toml";

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleClickAction {
    Open,
    Empty,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub update_config_speed: u64,
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

        let config = ConfigBuilder::builder()
            .add_source(File::from(file).format(FileFormat::Toml))
            .build()?;

        *self = config.try_deserialize::<Config>()?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update_config_speed: 1,
            translate: Translate::default(),
            trash: TrashSettings::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrashSettings {
    pub max_fill_size_mb: u64,
    pub non_adaptive_fill_size: u8,
    pub double_click_actions: DoubleClickAction,
    pub icons_two_states: bool,
    pub recycle_no_progress: bool,
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
            double_click_actions: DoubleClickAction::Open,
            non_adaptive_fill_size: 1,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
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
            open: "Open".to_string(),
            empty: "Empty".to_string(),
            configure: "Configure".to_string(),
            configure_double_click: "Icon Double-Click Action".to_string(),
            configure_system: "System Integration".to_string(),
            configure_system_confirm: "Confirm Recycling".to_string(),
            configure_system_sound: "Allow Sound".to_string(),
            configure_system_progress: "Allow Progress Window".to_string(),
            configure_icons: "Change Icons".to_string(),
            configure_icons_two_state: "Only use empty/full icons".to_string(),
            configure_icons_empty: "Empty".to_string(),
            configure_icons_25: "25%".to_string(),
            configure_icons_50: "50%".to_string(),
            configure_icons_75: "75%".to_string(),
            configure_icons_full: "Full".to_string(),
            configure_icons_reset: "Reset Icons".to_string(),
            about: "About".to_string(),
            exit: "Exit".to_string(),
        }
    }
}
