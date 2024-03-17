use serde::Serialize;

use crate::github::pulls::AbandonedPullRequest;
use crate::settings::Settings;

use mustache;

trait Format {
    fn format(&self, template: impl Into<String>) -> String;
}

#[derive(Serialize)]
struct Message {
    pulls: String,
}

impl Format for Message {
    fn format(&self, template: impl Into<String>) -> String {
        let m = mustache::compile_str(template.into().as_str()).unwrap();
        m.render_to_string(self).unwrap()
    }
}

#[derive(Serialize)]
struct MessageFormat {
    pull_title: String,
    pull_url: String,
    abandoned_days: i64,
}

impl From<&AbandonedPullRequest> for MessageFormat {
    fn from(p: &AbandonedPullRequest) -> Self {
        MessageFormat {
            pull_title: p.pull.pull.title.clone().unwrap(),
            pull_url: p.pull.url.clone(),
            abandoned_days: p.delta_days,
        }
    }
}

impl Format for MessageFormat {
    fn format(&self, template: impl Into<String>) -> String {
        let m = mustache::compile_str(template.into().as_str()).unwrap();
        m.render_to_string(self).unwrap()
    }
}

fn build_message(
    message: &String,
    format_string: &String,
    abandoned_pulls: &Vec<&AbandonedPullRequest>,
) -> String {
    let formatted_strings: Vec<String> = abandoned_pulls
        .iter()
        .map(|p| MessageFormat::from(*p).format(format_string))
        .collect();

    Message {
        pulls: formatted_strings.join("\n"),
    }
    .format(message)
}

fn build_wait_for_review_message(
    settings: &Settings,
    wait_for_review: &Vec<&AbandonedPullRequest>,
) -> String {
    build_message(
        &settings.slack.wait_for_review_message,
        &settings.slack.wait_for_review_message_format,
        wait_for_review,
    )
}

fn build_wait_for_merge(
    settings: &Settings,
    wait_for_merge: &Vec<&AbandonedPullRequest>,
) -> String {
    build_message(
        &settings.slack.wait_for_merge_message,
        &settings.slack.wait_for_merge_message_format,
        wait_for_merge,
    )
}

pub fn build(
    settings: &Settings,
    wait_for_review: &Vec<&AbandonedPullRequest>,
    wait_for_merge: &Vec<&AbandonedPullRequest>,
) -> String {
    format!(
        "{}\n\n{}",
        build_wait_for_review_message(settings, wait_for_review),
        build_wait_for_merge(settings, wait_for_merge)
    )
}
