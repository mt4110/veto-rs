# Architecture

## Design goals
- **Fast**: most checks should run on staged diff or small scope by default.
- **Safe output**: never print raw secrets; prefer masked snippets.
- **Composable**: checks are pluggable modules; same engine can power CLI & CI.
- **Mono-core / multi-tool**: keep one core engine, allow multiple thin front-ends.

## Workspace crates
- `veri-core`
  - Domain model: `Finding`, `Report`, `Severity`
  - Check framework: `Check` trait, `Runner`
  - Context: repo root, execution scope (staged diff / full tree / etc.)
- `veri-config`
  - `veri.toml` parsing + defaults (keeps policy outside code)
- `veri-cli`
  - CLI UX: `scan`, `doctor`, formats, exit codes
- `xtask`
  - Optional developer tooling (release notes, version bump, etc.)

## Check lifecycle
1. CLI builds `Context` and loads config
2. Runner executes selected checks
3. Report is printed as text or JSON
4. Exit code is derived from findings (configurable threshold)

## Future: storage/DB
For audit logs or multi-repo dashboards, add a storage layer:
- `Storage` trait (append report, query summaries)
- `postgres` adapter behind a feature flag
