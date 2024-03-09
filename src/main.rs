use std::borrow::Borrow;
use std::fs;
use reqwest;

use octocrab::params;
use serde::{Deserialize, Serialize};

use serenity::all::CreateMessage;
use serenity::{async_trait, prelude::*};
use serenity::model::prelude::*;
use serenity::model::id::ChannelId;

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

        struct DiscordBot {}

        #[async_trait]
        impl EventHandler for DiscordBot
        {
            async fn message(&self, ctx: Context, msg: Message)
            {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await
                    {
                        println!("Error sending message: {:?}", why);
                    }
            }

            async fn ready(&self, _: Context, ready: Ready)
            {
                println!("{} is connected", ready.user.name);
            }
        }

// #[derive(Debug,Serialize)]
// struct SlackGetChannelListBody
// {
//     types: Option<String>,
// }

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

        // github
        //     .issues("XAMPPRocky", "octocrab")
        //     .list()
        //     .state(params::State::All)
        //     .per_page(10)
        //     .send()
        //     .await
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

        // list channels
        // {
        //     let url = "https://slack.com/api/conversations.list";
        //     
        //     let body = SlackGetChannelListBody{
        //         types: Some("public_channel,private_channel".to_owned()),
        //     };
        //     let response = match rt.block_on(async {client.post(url).header(
        //         reqwest::header::AUTHORIZATION, format!("Bearer {}", slack_token)).header( reqwest::header::CONTENT_TYPE,"application/json".to_owned()).json(&body
        //     ).send().await}){
        //         Ok(r) => r,
        //         Err(e) => {
        //             eprintln!("{}", e);
        //             return;
        //         }
        //     };

        //     
        //     #[derive(Debug, Deserialize)]
        //     struct SlackChannel {
        //         id: String,
        //         name: String,
        //         is_channel: bool,
        //     }

        //     #[derive(Debug,Deserialize)]
        //     struct             SlackGetChannelListResponse {
        //         ok: bool,
        //         error: Option<String>,
        //         channels: Vec<SlackChannel>,

        //     }

        //     println!("{:?}", response);
        //     let text:SlackGetChannelListResponse = match rt.block_on(async {response.json().await}) {
        //         Ok(r) => r,
        //         Err(e) => {
        //             eprintln!("{}", e);
        //             return;
        //         }
        //     };
        //     
        //     println!("{:#?}", text.channels);
        // }

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

    // discord
    {

        let token  = &settings.discord.token;
        let server_id = &settings.discord.server_id;

        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT| GatewayIntents::DIRECT_MESSAGES;

        // let mut client = Client::builder(token, intents).event_handler(DiscordBot).await.expected("Err creating client");

        // if let Err(why) = client.start().await {
        //     println!("Client error: {:?}", why);
        // }

        // let builder = CreateMessage::new().content("hello");
        // let channel_id = ChannelId(server_id.parse::<u64>().unwrap());
        //     match rt.block_on(async {channel_id.say(&token, &builder).await}) {
        //         Ok(_) => println!("Message sent successfully"),
        //         Err(why) => println!("Error sending message: {:?}", why),
        //     };
    }

    // println!("{:#?}", page);

}
