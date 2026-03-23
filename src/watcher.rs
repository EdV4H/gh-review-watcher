use crate::action;
use crate::config::Config;
use crate::github::{self, PullRequest};
use std::collections::HashSet;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

pub enum WatcherEvent {
    Updated(Vec<PullRequest>),
    Error(String),
}

pub fn spawn_watcher(
    config: Config,
    tx: mpsc::UnboundedSender<WatcherEvent>,
) {
    tokio::spawn(async move {
        let mut known_ids: HashSet<(String, u64)> = HashSet::new();
        let mut first_run = true;

        loop {
            match tokio::task::spawn_blocking(github::fetch_review_requests).await {
                Ok(Ok(prs)) => {
                    if !first_run {
                        let current_ids: HashSet<(String, u64)> = prs
                            .iter()
                            .map(|pr| (pr.repo().to_string(), pr.number))
                            .collect();

                        let new_prs: Vec<&PullRequest> = prs.iter().filter(|pr| {
                            let key = (pr.repo().to_string(), pr.number);
                            !known_ids.contains(&key)
                        }).collect();

                        // Run actions for new PRs in a blocking context
                        if !new_prs.is_empty() {
                            let actions = config.on_new_pr.clone();
                            let new_prs_owned: Vec<PullRequest> = new_prs.into_iter().cloned().collect();
                            tokio::task::spawn_blocking(move || {
                                for pr in &new_prs_owned {
                                    for act in &actions {
                                        action::run_command(&act.command, pr);
                                    }
                                }
                            });
                        }

                        known_ids = current_ids;
                    } else {
                        known_ids = prs
                            .iter()
                            .map(|pr| (pr.repo().to_string(), pr.number))
                            .collect();
                        first_run = false;
                    }

                    let _ = tx.send(WatcherEvent::Updated(prs));
                }
                Ok(Err(e)) => {
                    let _ = tx.send(WatcherEvent::Error(e));
                }
                Err(e) => {
                    let _ = tx.send(WatcherEvent::Error(format!("Task join error: {e}")));
                }
            }

            time::sleep(Duration::from_secs(config.interval)).await;
        }
    });
}
