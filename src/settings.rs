use serde::{Serialize, Deserialize};

pub const SETTINGS_VERSION: u8 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsGithub {
    pub token: String,
    pub user: String,
    pub repository: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsSlack {
    pub token: String,
    pub channel: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingDiscord {
    pub token: String,
    pub server_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "Settings::get_default_version")]
    pub version: u8,
    pub github: SettingsGithub,
    pub slack: SettingsSlack,
    pub discord: SettingDiscord,
}

impl Settings {
    fn get_default_version() -> u8 {
        SETTINGS_VERSION
    }
}

