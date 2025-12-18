# veto-rs
**Local verification gates** for developers and CI: stop accidents *before* they land.

## What it is
`veto` is a Rust CLI that runs a set of **checks** against your repo (or staged diff) and returns:
- human-readable output (safe by default)
- machine-readable JSON
- non-zero exit codes to block commits/CI when needed

This repo is structured as a small workspace:
- `veto-core`  : check framework + report model (no CLI, no IO assumptions)
- `veto-config`: config parsing + defaults
- `veto-cli`   : user-facing CLI (subcommands, formats, exit codes)
- `xtask`      : release/dev helpers (optional)

## Prerequisites

To ensure reproducibility across platforms, please use **Nix**.

```bash
# Recommended (with direnv)
direnv allow

# Or explicitly
nix develop
```

Please ensure you are inside the Nix environment before running `cargo`.

## Quick start
```bash
cargo run -p veto-cli -- scan
```

JSON output:
```bash
cargo run -p veto-cli -- scan --format json
```

## Config
Copy:
- `config/veto.toml.example` → `veto.toml`

## Optional infra
- `infra/postgres/docker-compose.yml` (only if you later want audit logs / shared state)

## Roadmap (high level)
- Entropy Guard (staged diff secret blocker)
- Dependency checks (Cargo.lock / npm lock / OSV)
- Signature verification (tags / commits)
- Build reproducibility checks (Nix-oriented)

See: `docs/ROADMAP.md`
