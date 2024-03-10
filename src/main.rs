use std::{ fs};

use octocrab::{models::pulls::PullRequest, params};

use chrono::TimeDelta;

mod settings;
use settings::Settings;

mod slack;


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
    

    let protected_branch = protected_branches.items.get(0).unwrap();
    
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
            // .per_page(100)
            .send()
            .await
    
    }) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let abandoned_pulls: Vec<&PullRequest> = page.items.iter().filter(|p| {
        let created_at = match &p.created_at {
            Some(t) => t,
            None => return false,
        };
        let delta = chrono::Utc::now() - created_at;
        let delta_days = delta.num_days();
        println!("{:?}", &delta_days);
        delta_days > (settings.github.abandoned_days as i64)
    }).collect();

    println!("--------------------------------------------------------------------------------");
    for pull in abandoned_pulls.iter() {
        println!("{:?}", pull);
    }
    println!("--------------------------------------------------------------------------------");

    {
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
        let first_page = abandoned_pulls.get(0).unwrap();
         // println!("{:?}", first_page);
        let pr_num = &first_page.number;
        let url = &first_page.url;
        let default_title = "(none)".to_owned();
        let title = first_page.title.as_ref().unwrap_or(&default_title);
        let created_at = &first_page.created_at;
        let updated_at = &first_page.updated_at;
        let closed_at = &first_page.closed_at;
        let merged_at = &first_page.merged_at;
        let default_request_reviewers = Vec::new();
        let requested_reviewers = first_page.requested_reviewers.as_ref().unwrap_or(&default_request_reviewers);
        let mergeable = &first_page.mergeable;
        let mergeable_state = &first_page.mergeable_state;

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
            println!("{:?}", requested_reviewer.email.as_ref().unwrap_or(&default_email));
        }

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

    slack::message::send::post(&settings, "test message".to_owned());

}
