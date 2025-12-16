# Roadmap

This is intentionally short. The goal is to ship small, then grow.

## v0.1 "Stop the accident"
- Check framework + JSON output
- Entropy Guard MVP (staged diff)
- Allowlist + baseline
- Pre-commit integration snippet

## v0.2 "Dependency reality check"
- Cargo.lock parsing (direct + transitive)
- Advisory integration (pluggable source: local DB / OSV)
- Output: package -> findings

## v0.3 "Trust chain"
- Signature verification (tag/commit verify)
- Policy: required signature for protected refs

## v0.4 "Repro or it didn't happen"
- Nix build reproducibility check (hash consistency)
- CI integration patterns

## later
- Optional Postgres storage for audit logs / dashboards
- Extra "small tools" as subcommands (zombie-proc, etc.) but powered by shared core
