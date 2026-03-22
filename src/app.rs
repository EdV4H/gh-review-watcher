use crate::github::PullRequest;
use chrono::Local;

pub struct App {
    pub prs: Vec<PullRequest>,
    pub selected: usize,
    pub last_updated: String,
    pub error: Option<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            prs: Vec::new(),
            selected: 0,
            last_updated: "never".to_string(),
            error: None,
            should_quit: false,
        }
    }

    pub fn update_prs(&mut self, prs: Vec<PullRequest>) {
        self.prs = prs;
        self.last_updated = Local::now().format("%H:%M:%S").to_string();
        self.error = None;
        // Keep selection in bounds
        if self.selected >= self.prs.len() && !self.prs.is_empty() {
            self.selected = self.prs.len() - 1;
        }
    }

    pub fn set_error(&mut self, err: String) {
        self.error = Some(err);
        self.last_updated = Local::now().format("%H:%M:%S").to_string();
    }

    pub fn next(&mut self) {
        if !self.prs.is_empty() {
            self.selected = (self.selected + 1).min(self.prs.len() - 1);
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn selected_pr(&self) -> Option<&PullRequest> {
        self.prs.get(self.selected)
    }
}
