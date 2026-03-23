use crate::github::PullRequest;
use std::process::Command;

fn shell_escape(s: &str) -> String {
    // Single-quote escaping: replace ' with '\''
    format!("'{}'", s.replace('\'', "'\\''"))
}

pub fn expand_template(template: &str, pr: &PullRequest) -> String {
    template
        .replace("{repo}", pr.repo())
        .replace("{number}", &pr.number.to_string())
        .replace("{title}", &shell_escape(&pr.title))
        .replace("{author}", pr.author())
        .replace("{url}", &pr.url)
}

pub fn run_command(command: &str, pr: &PullRequest) {
    let expanded = expand_template(command, pr);
    match Command::new("sh")
        .arg("-c")
        .arg(&expanded)
        .spawn()
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to run on_new_pr command: {e}\nCommand: {expanded}");
        }
    }
}
