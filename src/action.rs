use crate::github::PullRequest;
use std::process::Command;

pub fn expand_template(template: &str, pr: &PullRequest) -> String {
    template
        .replace("{repo}", pr.repo())
        .replace("{number}", &pr.number.to_string())
        .replace("{title}", &pr.title)
        .replace("{author}", pr.author())
        .replace("{url}", &pr.url)
}

pub fn run_command(command: &str, pr: &PullRequest) {
    let expanded = expand_template(command, pr);
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&expanded)
        .spawn();
}
