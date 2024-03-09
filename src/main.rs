use std::fs;
use reqwest;

use octocrab::params;
use serde::{Deserialize, Serialize};

use chrono::TimeDelta;

const SETTINGS_VERSION: u8 = 1;

#[derive(Debug, Serialize, Deserialize)]
struct SettingsGithub {
    token: String,
    user: String,
    repository: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SettingsSlack {
    token: String,
    channel: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SettingDiscord {
    token: String,
    server_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    #[serde(default = "Settings::get_default_version")]
    version: u8,
    github: SettingsGithub,
    slack: SettingsSlack,
    discord: SettingDiscord,
}

impl Settings {
    fn get_default_version() -> u8 {
        SETTINGS_VERSION
    }
}

#[derive(Debug,Serialize)]
struct SlackPostMessageBody
{
channel: String,
text: String,
}

#[derive(Debug, Deserialize)]
struct SlackPostMessageResponse
{
    ok : bool,
    error: Option<String>,
}



fn main() {
    println!("Hello, world!");

    let file_path = ".settings.toml";
    let settings_contents = fs::read_to_string(file_path).unwrap();
    let settings: Settings = match toml::from_str(&settings_contents) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    println!("token: {}", settings.github.token);
    println!("user: {}", settings.github.user);
    println!("repository: {}", settings.github.repository);
    println!("{:#?}", settings);

    let rt = tokio::runtime::Runtime::new().unwrap();

    let protected_branches = match rt.block_on(async {
        let github = octocrab::instance();

        github
            .repos(&settings.github.user, &settings.github.repository)
            .list_branches()
            .protected(true)
            .send()
            .await
    }) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let protected_branch = &protected_branches.items.get(0).unwrap();

    // println!("{:#?}", protected_branches);
    let page = match rt.block_on(async {
        let github = octocrab::instance();

        github
            .pulls(&settings.github.user, &settings.github.repository)
            .list()
            // Optional Parameters
            .state(params::State::Open)
            //.head("main")
            //.base("branch")
            .sort(params::pulls::Sort::Popularity)
            .direction(params::Direction::Ascending)
            .per_page(100)
            .send()
            .await

    }) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    {
        let first_page = page.items.get(0).unwrap();
        // println!("{:?}", tmp);
        let url = &first_page.url;
        let default_title = "(none)".to_owned();
        let title = first_page.title.as_ref().unwrap_or(&default_title);
        let created_at = &first_page.created_at;
        let updated_at = &first_page.updated_at;
        let closed_at = &first_page.closed_at;
        let merged_at = &first_page.merged_at;
        let default_request_reviewers = Vec::new();
        let requested_reviewers = first_page.requested_reviewers.as_ref().unwrap_or(&default_request_reviewers);

        println!("{:?}", url);
        println!("{:?}", title);
        println!("{:?}", created_at);
        println!("{:?}", updated_at);
        println!("{:?}", closed_at);
        println!("{:?}", merged_at);
        println!("{:?}", requested_reviewers);

        let delta = match TimeDelta::try_days(90) {
            Some(d) => d,
            None => {
                eprintln!("TimeDelta failed.");
                return;
            }
        };

        let created = created_at.unwrap();
        println!("{:?}", created);
        println!("{:?}", created - delta);
        let now = chrono::Utc::now();
        println!("{:?}",now-created);
        let diff = now -created;
        println!("days: {}", diff.num_days());
    }


    // send slack
    {
        let slack_token = &settings.slack.token;
        let channel = &settings.slack.channel;

        let client = reqwest::Client::new();

        let body = SlackPostMessageBody{
            channel: channel.to_owned(),
            text: "hello from bot".to_owned(),
        };

        let url = "https://slack.com/api/chat.postMessage";
        let response = match rt.block_on(async {client.post(url).header(reqwest::header::AUTHORIZATION, format!("Bearer {}", slack_token)).
            json(&body).send().await}) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
        // println!("--------------------------------------------------------------------------------");
        // println!("{:?}", response);
        let post_response: SlackPostMessageResponse = match rt.block_on(async {response.json().await}) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        // println!("{:#?}", post_response);
    }

}
