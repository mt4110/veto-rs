use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use anyhow::{Context as _, Result};

use crate::runner::ScopeMode;
use crate::{Check, Context, Finding, Severity};

pub struct EntropyGuard {
    pub enabled: bool,
    pub min_length: usize,
    pub threshold: f64,
    pub ignore_extensions: Vec<String>,
    pub allowlist: Vec<String>,
}

impl Check for EntropyGuard {
    fn id(&self) -> &'static str {
        "EG-001"
    }

    fn description(&self) -> &'static str {
        "Detects high-entropy strings that may be secrets"
    }

    fn run(&self, ctx: &crate::Context) -> Result<Vec<Finding>> {
        if !self.enabled {
            return Ok(vec![]);
        }

        let files = get_target_files(ctx)?;
        let mut findings = vec![];

        for (path_string, content) in files {
            // Check ignore extensions
            if let Some(ext) = Path::new(&path_string).extension() {
                if let Some(ext_str) = ext.to_str() {
                    if self
                        .ignore_extensions
                        .iter()
                        .any(|e| e.eq_ignore_ascii_case(ext_str))
                    {
                        continue;
                    }
                }
            }

            // Allowlist check (naive substring match for file path, maybe?)
            // Usually allowlist is for token content, but file path ignore is also useful.
            // For now, let's implement token-based allowlist as requested.

            let tokens = tokenize(&content);
            for (line_num, token) in tokens {
                if token.len() < self.min_length {
                    continue;
                }

                if self.allowlist.iter().any(|pattern| token.contains(pattern)) {
                    continue;
                }

                let entropy = shannon_entropy(token);
                if entropy > self.threshold {
                    findings.push(Finding {
                        id: self.id().to_string(),
                        title: "High-entropy token detected".to_string(),
                        severity: Severity::High,
                        message: format!(
                            "Possible secret detected (entropy: {:.2}, len: {}). Content: {}",
                            entropy,
                            token.len(),
                            mask_token(token)
                        ),
                        location: Some(crate::model::Location {
                            file: path_string.clone(),
                            line: Some(line_num as u32),
                        }),
                        tags: vec!["entropy".to_string()],
                    });
                }
            }
        }

        Ok(findings)
    }
}

fn get_target_files(ctx: &Context) -> Result<Vec<(String, String)>> {
    match ctx.scope {
        ScopeMode::Staged => {
            // git diff --cached --name-only --diff-filter=AM
            let output = Command::new("git")
                .arg("diff")
                .arg("--cached")
                .arg("--name-only")
                .arg("--diff-filter=AM")
                .current_dir(&ctx.repo_root)
                .output()
                .context("git diff --cached failed")?;

            if !output.status.success() {
                // If not a git repo or no commits yet, might fail.
                // Return empty if basic git command fails to avoid hard panic?
                // Or error out. Let's error out for now.
                return Err(anyhow::anyhow!(
                    "git diff failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }

            let paths = String::from_utf8(output.stdout)?;
            let mut results = vec![];

            for path in paths.lines() {
                // git show :path
                let show_out = Command::new("git")
                    .arg("show")
                    .arg(format!(":{}", path))
                    .current_dir(&ctx.repo_root)
                    .output();

                if let Ok(out) = show_out {
                    if out.status.success() {
                        // Check binary
                        if is_binary(&out.stdout) {
                            continue;
                        }
                        if let Ok(s) = String::from_utf8(out.stdout) {
                            results.push((path.to_string(), s));
                        }
                    }
                }
            }
            Ok(results)
        }
        ScopeMode::Worktree => {
            // git diff --name-only --diff-filter=AM
            // (Note: this only captures *changed* files. If we want ALL files in worktree, we need `git ls-files`)
            // But usually 'worktree' scan implies "scanning my current changes" not "audit entire repo".
            // Let's assume "changed files in worktree" for now (aligned with user request "git diff --name-only").

            let output = Command::new("git")
                .arg("diff")
                .arg("--name-only")
                .arg("--diff-filter=AM")
                .current_dir(&ctx.repo_root)
                .output()
                .context("git diff failed")?;

            if !output.status.success() {
                return Err(anyhow::anyhow!(
                    "git diff failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }

            let paths = String::from_utf8(output.stdout)?;
            let mut results = vec![];

            for p in paths.lines() {
                let full_path = ctx.repo_root.join(p);
                if full_path.exists() && full_path.is_file() {
                    // Simple binary check
                    // Read file
                    let mut f = fs::File::open(&full_path)?;
                    let mut buffer = Vec::new();
                    f.read_to_end(&mut buffer)?;

                    if is_binary(&buffer) {
                        continue;
                    }

                    if let Ok(s) = String::from_utf8(buffer) {
                        results.push((p.to_string(), s));
                    }
                }
            }
            Ok(results)
        }
        ScopeMode::Repo => {
            // TODO: Phase 2
            Ok(vec![])
        }
    }
}

// Simple heuristic for binary content
fn is_binary(data: &[u8]) -> bool {
    // Check first 1024 bytes for null byte
    data.iter().take(1024).any(|&b| b == 0)
}

fn shannon_entropy(s: &str) -> f64 {
    let mut map = HashMap::new();
    let len = s.len() as f64;
    for c in s.chars() {
        *map.entry(c).or_insert(0.0) += 1.0;
    }

    let mut entropy = 0.0;
    for count in map.values() {
        let p = count / len;
        entropy -= p * p.log2();
    }
    entropy
}

// Tokenize: split by whitespace and common delimiters
// Returns (line_number, token_str)
fn tokenize(content: &str) -> Vec<(usize, &str)> {
    let mut tokens = vec![];
    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1;
        // Naive split: whitespace, quotes, etc.
        // We want to capture "raw" tokens.
        // Let's try splitting by whitespace first, then trimming quotes.

        for part in line.split_whitespace() {
            // Trim widely used quotes
            let trimmed = part.trim_matches(|c| {
                c == '"' || c == '\'' || c == '`' || c == ',' || c == ';' || c == '='
            });
            if !trimmed.is_empty() {
                tokens.push((line_num, trimmed));
            }
        }
    }
    tokens
}

fn mask_token(token: &str) -> String {
    if token.len() <= 8 {
        return "***".to_string();
    }
    let start = &token[..4];
    let end = &token[token.len() - 4..];
    format!("{}...{}", start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy() {
        let low = "wvwwwvwvvwwv"; // Repetitive
        let high = "7Fz2X9kL1mN4pQ3r"; // Randomish

        let e_low = shannon_entropy(low);
        let e_high = shannon_entropy(high);

        assert!(e_high > e_low);
        assert!(e_high > 3.0); // usually > 3.5 for this length
    }

    #[test]
    fn test_mask() {
        assert_eq!(mask_token("secret"), "***");
        assert_eq!(mask_token("1234567890"), "1234...7890");
    }
}
