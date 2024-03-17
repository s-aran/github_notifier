use reqwest;
use serde::Serialize;
use tokio::runtime::Runtime;

use crate::slack::message::model::*;
use crate::Settings;

pub fn post(settings: &Settings, message: impl Into<String>) {
    let slack_token = &settings.slack.token;
    let channel = &settings.slack.channel;
    let mrkdwn = &settings.slack.enable_markdown;

    let client = reqwest::Client::new();

    let block = SlackPostMessageBlocks {
        block_type: "section".to_owned(),
        text: SlackPostMessageBlocksText {
            text_type: "mrkdwn".to_owned(),
            text: message.into(),
        },
    };

    let body = SlackPostMessageBody {
        channel: channel.to_owned(),
        // text: message.into(),
        blocks: vec![block],
        username: None,
        // mrkdwn: mrkdwn.to_owned(),
    };

    println!("body: {:?}", body);

    let rt = Runtime::new().unwrap();
    let url = "https://slack.com/api/chat.postMessage";
    let response = match rt.block_on(async {
        client
            .post(url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", slack_token),
            )
            .json(&body)
            .send()
            .await
    }) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    // println!("--------------------------------------------------------------------------------");
    // println!("{:?}", response);
    let post_response: SlackPostMessageResponse = match rt.block_on(async { response.json().await })
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    // println!("{:#?}", post_response);
}

pub fn post_with_webhook(settings: &Settings, message: impl Into<String>) {
    // let webhook_url = match &settings.slack.webhook_url {
    //     Some(url) => url,
    //     None => {
    //         eprintln!("webhook url is not set");
    //         return;
    //     }
    // };

    // let client = reqwest::Client::new();

    // let body = SlackPostMessageBody {
    //     channel: settings.slack.channel.to_owned(),
    //     text: message.into(),
    //     username: None,
    //     mrkdwn: settings.slack.enable_markdown,
    // };

    // let rt = Runtime::new().unwrap();
    // let response = match rt.block_on(async { client.post(webhook_url).json(&body).send().await }) {
    //     Ok(r) => r,
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         return;
    //     }
    // };
    // // println!("--------------------------------------------------------------------------------");
    // // println!("{:?}", response);
    // let post_response: SlackPostMessageResponse = match rt.block_on(async { response.json().await })
    // {
    //     Ok(r) => r,
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         return;
    //     }
    // };
}
