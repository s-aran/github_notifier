use octocrab::models::pulls::MergeableState;
use octocrab::models::pulls::PullRequest;
use octocrab::Page;
use octocrab::{params, Error, Octocrab};
use tokio::runtime::Runtime;

use crate::settings::Github;

#[derive(Debug, Clone)]
pub struct PullRequestWithMergeable {
    pub pull: PullRequest,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct AbandonedPullRequest {
    pub pull: PullRequestWithMergeable,
    pub delta_days: i64,
    pub abandoned: bool,
}

fn create_octocrab(github_token: &String) -> Octocrab {
    Octocrab::builder()
        .personal_token(github_token.to_owned())
        .build()
        .unwrap()
}

fn create_rt() -> Runtime {
    tokio::runtime::Runtime::new().unwrap()
}

pub fn list_pulls(github_settings: &Github) -> Result<Page<PullRequest>, Error> {
    let rt = create_rt();
    rt.block_on(async {
        let octocrab = create_octocrab(&github_settings.token);
        octocrab
            .pulls(&github_settings.user, &github_settings.repository)
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
    })
}

pub fn get_pull(github_settings: &Github, number: u64) -> PullRequestWithMergeable {
    let rt = create_rt();
    match rt.block_on(async {
        let octocrab = create_octocrab(&github_settings.token);
        octocrab
            .pulls(&github_settings.user, &github_settings.repository)
            .get(number)
            .await
    }) {
        Ok(r) => PullRequestWithMergeable {
            url: format!(
                "https://github.com/{}/{}/pull/{}",
                github_settings.user, github_settings.repository, r.number,
            ),
            pull: r,
        },
        Err(e) => {
            eprintln!("Error: {:?}", e);
            panic!();
        }
    }
}

pub fn list_pulls_with_mergeable(
    github_settings: &Github,
    pulls: Vec<PullRequest>,
) -> Vec<PullRequestWithMergeable> {
    pulls
        .iter()
        .map(|p| {
            return get_pull(&github_settings, p.number);
        })
        .collect()
}

pub fn make_abandoned_pulls(
    github_settings: &Github,
    pulls: Vec<PullRequestWithMergeable>,
) -> Vec<AbandonedPullRequest> {
    pulls
        .iter()
        .filter_map(|p| {
            let created_at = match &p.pull.created_at {
                Some(t) => t,
                None => return None,
            };

            let delta = chrono::Utc::now() - created_at;
            let delta_days = delta.num_days();
            let abandoned = delta_days > (github_settings.abandoned_days as i64);

            let result = AbandonedPullRequest {
                pull: p.clone(),
                delta_days,
                abandoned,
            };

            if abandoned {
                Some(result)
            } else {
                None
            }
        })
        .collect()
}

pub fn filter_wait_for_review(pulls: &Vec<AbandonedPullRequest>) -> Vec<&AbandonedPullRequest> {
    pulls
        .iter()
        .filter_map(|p| {
            // NOTE: may always be true if the user is an authenticated admin
            // let mergeable

            let mergeable_state = match &p.pull.pull.mergeable_state {
                Some(s) => s.clone(),
                None => {
                    eprintln!("Warning: mergeable_state is None");
                    return None;
                }
            };

            if mergeable_state != MergeableState::Clean {
                Some(p)
            } else {
                None
            }
        })
        .collect()
}

pub fn filter_wait_for_merge(pulls: &Vec<AbandonedPullRequest>) -> Vec<&AbandonedPullRequest> {
    pulls
        .iter()
        .filter_map(|p| {
            let mergeable = match p.pull.pull.mergeable {
                Some(m) => m,
                None => {
                    eprintln!("Warning: mergeable member is None");
                    return None;
                }
            };

            let mergeable_state = match &p.pull.pull.mergeable_state {
                Some(s) => s.clone(),
                None => {
                    eprintln!("Warning: mergeable_state is None");
                    return None;
                }
            };

            if mergeable && mergeable_state == MergeableState::Clean {
                Some(p)
            } else {
                None
            }
        })
        .collect()
}
