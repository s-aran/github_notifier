use reqwest;
use tokio::runtime::Runtime;

use crate::Settings;  
use crate::slack::message::model::*;

pub fn post(settings: &Settings, message: impl Into<String>) {
        let slack_token = &settings.slack.token;
        let channel = &settings.slack.channel;

        let client = reqwest::Client::new();

        let body = SlackPostMessageBody{
            channel: channel.to_owned(),
            text: message.into(),
            username: None,
        };

    let rt = Runtime::new().unwrap();
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

pub fn post_with_webhook(settings: &Settings, message: impl Into<String>) {
    let webhook_url = match &settings.slack.webhook_url {
        Some(url) => url,
        None => {
            eprintln!("webhook url is not set");
            return;
        }
    };

    let client = reqwest::Client::new();

    let body = SlackPostMessageBody{
        channel: settings.slack.channel.to_owned(),
        text: message.into(),
        username: None,
    };

    let rt = Runtime::new().unwrap();
    let response = match rt.block_on(async {client.post(webhook_url).
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
}