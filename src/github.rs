use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct PullRequest {
    pub repository: RepoInfo,
    pub number: u64,
    pub title: String,
    pub author: AuthorInfo,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct RepoInfo {
    #[serde(rename = "nameWithOwner")]
    pub name_with_owner: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthorInfo {
    pub login: String,
}

impl PullRequest {
    pub fn repo(&self) -> &str {
        &self.repository.name_with_owner
    }

    pub fn author(&self) -> &str {
        &self.author.login
    }

    /// Format updatedAt for display (truncate to minutes)
    pub fn updated_short(&self) -> &str {
        if self.updated_at.len() >= 16 {
            &self.updated_at[..16]
        } else {
            &self.updated_at
        }
    }
}

pub fn fetch_review_requests() -> Result<Vec<PullRequest>, String> {
    let output = Command::new("gh")
        .args([
            "search",
            "prs",
            "--review-requested=@me",
            "--state=open",
            "--json",
            "repository,number,title,author,updatedAt,url",
            "--limit",
            "100",
        ])
        .output()
        .map_err(|e| format!("Failed to run gh: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("gh command failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse JSON: {e}"))
}
