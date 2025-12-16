use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use veri_core::{Context, Runner, Severity};
use veri_core::runner::ScopeMode;
use veri_config::Config;

#[derive(Parser, Debug)]
#[command(name = "veri", version, about = "Local verification gates (fast, safe output).")]
struct Cli {
    /// Repo root (default: current dir)
    #[arg(long)]
    repo: Option<PathBuf>,

    /// Config file path (default: veri.toml if present)
    #[arg(long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run checks
    Scan {
        /// Output format: text|json (overrides config)
        #[arg(long)]
        format: Option<String>,

        /// Scope: staged|worktree|repo (overrides config)
        #[arg(long)]
        scope: Option<String>,
    },

    /// Print environment & basic diagnostics
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let repo_root = cli.repo.unwrap_or(std::env::current_dir()?);

    // Load config (optional)
    let cfg = load_config(cli.config.as_deref()).unwrap_or_default();

    match cli.cmd {
        Command::Doctor => {
            println!("veri doctor");
            println!("- repo_root: {}", repo_root.display());
            println!("- config: {}", if cli.config.is_some() { "custom" } else { "default/none" });
            println!("- rust: {}", env!("CARGO_PKG_RUST_VERSION"));
            Ok(())
        }
        Command::Scan { format, scope } => {
            let format = format.or_else(|| Some(cfg.output.format.clone())).unwrap_or_else(|| "text".into());
            let scope = scope.or_else(|| Some(cfg.scope.mode.clone())).unwrap_or_else(|| "staged".into());

            let ctx = Context {
                repo_root,
                scope: parse_scope(&scope),
            };

            // Runner with a placeholder check (replace with real checks later)
            let runner = Runner::new().with_check(Box::new(DummyCheck));

            let report = runner.run(&ctx)?;

            let exit_code = exit_code_from(&cfg, report.worst_severity());
            match format.as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&report)?);
                }
                _ => {
                    print_text(&report);
                }
            }

            std::process::exit(exit_code);
        }
    }
}

fn load_config(override_path: Option<&std::path::Path>) -> Result<Config> {
    let path = if let Some(p) = override_path {
        Some(p.to_path_buf())
    } else {
        let default = std::path::PathBuf::from("veri.toml");
        default.exists().then_some(default)
    };

    if let Some(p) = path {
        Ok(veri_config::load_from(p)?)
    } else {
        Ok(Config::default())
    }
}

fn parse_scope(s: &str) -> ScopeMode {
    match s {
        "repo" => ScopeMode::Repo,
        "worktree" => ScopeMode::Worktree,
        _ => ScopeMode::Staged,
    }
}

fn exit_code_from(cfg: &Config, worst: Option<Severity>) -> i32 {
    let threshold = match cfg.output.fail_on.as_str() {
        "critical" => Severity::Critical,
        "high" => Severity::High,
        "medium" => Severity::Medium,
        _ => Severity::Low,
    };

    match worst {
        None => 0,
        Some(w) if w >= threshold => 1,
        Some(_) => 0,
    }
}

fn print_text(report: &veri_core::Report) {
    if report.findings.is_empty() {
        println!("OK (no findings) — {}ms", report.duration_ms);
        return;
    }

    println!("Found {} issue(s) — {}ms", report.findings.len(), report.duration_ms);
    for f in &report.findings {
        let loc = f.location.as_ref().map(|l| format!("{}:{}", l.file, l.line.unwrap_or(0))).unwrap_or_else(|| "-".into());
        println!("- [{}] {} @ {}", format!("{:?}", f.severity).to_uppercase(), f.title, loc);
        println!("  {}", f.message);
    }
}

/// Placeholder check — replace with real modules (entropy guard, deps check, etc.)
struct DummyCheck;

impl veri_core::Check for DummyCheck {
    fn id(&self) -> &'static str { "CK-000" }
    fn description(&self) -> &'static str { "Example placeholder check (always passes)." }
    fn run(&self, _ctx: &Context) -> Result<Vec<veri_core::Finding>> {
        Ok(vec![])
    }
}
