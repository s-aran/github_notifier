use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub const SETTINGS_VERSION: u8 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountMapping {
    github_slack: HashMap<String, String>,
}

impl Default for AccountMapping {
    fn default() -> Self {
        Self {
            github_slack: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Github {
    pub token: String,
    pub user: String,
    pub repository: String,
    pub abandoned_days: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slack {
    pub token: String,
    pub channel: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Discord {
    pub token: String,
    pub server_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "Settings::get_default_version")]
    pub version: u8,

    #[serde(default = "AccountMapping::default")]
    pub account_mapping: AccountMapping,
    
    pub github: Github,
    pub slack: Slack,
    pub discord: Discord,
}

impl Settings {
    fn get_default_version() -> u8 {
        SETTINGS_VERSION
    }
}

