use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;

use crate::model::{Finding, Report};

#[derive(Debug, Clone)]
pub enum ScopeMode {
    Staged,
    Worktree,
    Repo,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub repo_root: PathBuf,
    pub scope: ScopeMode,
}

pub trait Check: Send + Sync {
    fn id(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(&self, ctx: &Context) -> Result<Vec<Finding>>;
}

pub struct Runner {
    checks: Vec<Box<dyn Check>>,
}

impl Runner {
    pub fn new() -> Self {
        Self { checks: vec![] }
    }

    pub fn with_check(mut self, check: Box<dyn Check>) -> Self {
        self.checks.push(check);
        self
    }

    pub fn run(&self, ctx: &Context) -> Result<Report> {
        let start = Instant::now();
        let mut findings = Vec::new();

        for check in &self.checks {
            let mut f = check.run(ctx)?;
            // Tag findings with the check id by default (makes filtering easier)
            for item in &mut f {
                if !item.tags.iter().any(|t| t == check.id()) {
                    item.tags.push(check.id().to_string());
                }
            }
            findings.extend(f);
        }

        Ok(Report {
            findings,
            duration_ms: start.elapsed().as_millis(),
        })
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}
