use serde::{Deserialize, Serialize};
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
    pub webhook_url: Option<String>,
    #[serde(default = "bool::default")]
    pub enable_markdown: bool,
    pub wait_for_review_message: String,
    pub wait_for_review_message_format: String,
    pub wait_for_merge_message: String,
    pub wait_for_merge_message_format: String,
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
    pub discord: Option<Discord>,
}

impl Settings {
    fn get_default_version() -> u8 {
        SETTINGS_VERSION
    }
}
