use std::fs;

mod settings;
use settings::Settings;

use crate::github::pulls;

mod github;
mod slack;

fn main() {
    println!("Hello, world!");

    let file_path = ".settings.toml";
    let settings_contents = fs::read_to_string(file_path).unwrap();
    let settings: Settings = match toml::from_str(&settings_contents) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    println!("token: {}", settings.github.token);
    println!("user: {}", settings.github.user);
    println!("repository: {}", settings.github.repository);
    println!("{:#?}", settings);

    let pulls_page = pulls::list_pulls(&settings.github).unwrap();
    let pulls_with_mergeable = pulls::list_pulls_with_mergeable(&settings.github, pulls_page.items);
    let abandoned_pulls = pulls::make_abandoned_pulls(&settings.github, pulls_with_mergeable);

    for pr in abandoned_pulls.iter() {
        let p = &pr.pull.pull;
        // let first_page = match rt.block_on(async {
        // let github = octocrab::instance();
        // github.pulls(&settings.github.user, &settings.github.repository).get(3).await})
        // {
        //     Ok(p) => p,
        //     Err(e) => {
        //         eprintln!("{}", e);
        //         return;
        //     }
        // };

        // let first_page = page.items.get(0).unwrap();
        // println!("{:?}", first_page);
        let pr_num = &p.number;
        let url = &p.url;
        let default_title = "(none)".to_owned();
        let title = p.title.as_ref().unwrap_or(&default_title);
        let created_at = &p.created_at;
        let updated_at = &p.updated_at;
        let closed_at = &p.closed_at;
        let merged_at = &p.merged_at;
        let default_request_reviewers = Vec::new();
        let requested_reviewers = p
            .requested_reviewers
            .as_ref()
            .unwrap_or(&default_request_reviewers);
        let mergeable = &p.mergeable;
        let mergeable_state = &p.mergeable_state;

        println!("{:?}", pr_num);
        println!("{:?}", url);
        println!("{:?}", title);
        println!("{:?}", created_at);
        println!("{:?}", updated_at);
        println!("{:?}", closed_at);
        println!("{:?}", merged_at);
        println!("{:?}", requested_reviewers);
        println!("{:?}", mergeable);
        println!("{:?}", mergeable_state);

        if requested_reviewers.len() > 0 {
            let requested_reviewer = requested_reviewers.get(0).unwrap();
            println!("{:?}", requested_reviewer.login);
            println!("{:?}", requested_reviewer.id);
            let default_email = "(none)".to_owned();
            println!(
                "{:?}",
                requested_reviewer.email.as_ref().unwrap_or(&default_email)
            );
        }

        let created = created_at.unwrap();
        // println!("{:?}", created);
        // println!("{:?}", created - delta);
        let now = chrono::Utc::now();
        // println!("{:?}",now-created);
        let diff = now - created;
        println!("days: {}", diff.num_days());

        println!("----------------------------------------------------------------------------------------------------");
    }

    let wait_for_review_pulls = pulls::filter_wait_for_review(&abandoned_pulls);
    println!("wait for review:");
    println!("====================================================================================================");
    for r in wait_for_review_pulls.iter() {
        println!("{:?}", r.pull.pull.number);
        println!("{:?}", r.pull.pull.requested_reviewers);
        println!("{:?}", r.pull.pull.review_comments);
        println!("--------------------------------------------------");
    }

    let wait_for_merge = pulls::filter_wait_for_merge(&abandoned_pulls);
    println!("wait for merge:");
    println!("====================================================================================================");
    for r in wait_for_merge.iter() {
        println!("{:?}", r.pull.pull.number);
        println!("{:?}", r.pull.pull.requested_reviewers);
        println!("{:?}", r.pull.pull.review_comments);
        println!("--------------------------------------------------");
    }

    {
        let mut message_list: Vec<String> = Vec::new();
        for r in wait_for_review_pulls.iter() {
            let p = &r.pull.pull;
            let num = p.number;
            let links = p.links.unwrap();
            let title = p
                .title
                .clone()
                .unwrap_or(format!("(no title, #{}", num));
            let pr_url = match links.pull_request_link {
                Some(u) => Into::<String>::into(u.href),
                None => "".to_owned(),
            };
            message_list.push(format!("[{}]({})", title, pr_url));
        }

        let pulls = message_list.join("\n");
        let message = &settings
            .slack
            .wait_for_review_message
            .replace("{pulls}", &pulls);

        slack::message::send::post(&settings, message);
    }
    // slack::message::send::post_with_webhook(&settings, "test message".to_owned());
}
