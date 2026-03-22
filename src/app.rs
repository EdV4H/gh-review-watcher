use crate::github::PullRequest;
use chrono::{Local, DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filter {
    All,
    Recent24h,
    Recent7d,
}

impl Filter {
    pub fn label(&self) -> &'static str {
        match self {
            Filter::All => "All",
            Filter::Recent24h => "24h",
            Filter::Recent7d => "7d",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Filter::All => Filter::Recent24h,
            Filter::Recent24h => Filter::Recent7d,
            Filter::Recent7d => Filter::All,
        }
    }
}

pub struct App {
    pub prs: Vec<PullRequest>,
    pub selected: usize,
    pub last_updated: String,
    pub error: Option<String>,
    pub should_quit: bool,
    pub refreshing: bool,
    pub filter: Filter,
}

impl App {
    pub fn new() -> Self {
        Self {
            prs: Vec::new(),
            selected: 0,
            last_updated: "never".to_string(),
            error: None,
            should_quit: false,
            refreshing: false,
            filter: Filter::All,
        }
    }

    pub fn update_prs(&mut self, prs: Vec<PullRequest>) {
        self.prs = prs;
        self.last_updated = Local::now().format("%H:%M:%S").to_string();
        self.error = None;
        self.refreshing = false;
        // Keep selection in bounds
        if self.selected >= self.prs.len() && !self.prs.is_empty() {
            self.selected = self.prs.len() - 1;
        }
    }

    pub fn set_error(&mut self, err: String) {
        self.error = Some(err);
        self.last_updated = Local::now().format("%H:%M:%S").to_string();
        self.refreshing = false;
    }

    pub fn filtered_prs(&self) -> Vec<&PullRequest> {
        let now = Utc::now();
        self.prs.iter().filter(|pr| {
            match self.filter {
                Filter::All => true,
                Filter::Recent24h => {
                    if let Ok(dt) = pr.updated_at.parse::<DateTime<Utc>>() {
                        now.signed_duration_since(dt).num_hours() < 24
                    } else {
                        true
                    }
                }
                Filter::Recent7d => {
                    if let Ok(dt) = pr.updated_at.parse::<DateTime<Utc>>() {
                        now.signed_duration_since(dt).num_days() < 7
                    } else {
                        true
                    }
                }
            }
        }).collect()
    }

    pub fn next(&mut self) {
        let len = self.filtered_prs().len();
        if len > 0 {
            self.selected = (self.selected + 1).min(len - 1);
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn selected_pr(&self) -> Option<&PullRequest> {
        self.filtered_prs().get(self.selected).copied()
    }

    pub fn toggle_filter(&mut self) {
        self.filter = self.filter.next();
        self.selected = 0;
    }
}
