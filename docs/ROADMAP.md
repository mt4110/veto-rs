# Roadmap

This is intentionally short. The goal is to ship small, then grow.

## v0.2.1: Stable Release Process (Reproducibility)
- Fix dates and versions in docs/config
- Ensure clean tarball (no `._*` files) via `git archive`
- Include release checklist

## v0.2.2: Entropy Guard Tuning (Field Ready)
- Improved tokenization (URL/Base64 handling)
- File/Line size limits (DoS prevention)
- --explain output (no raw values)

## v0.2.3: Repo Scan (EG-2)
- Repo-wide scan implementation
- Default directory ignores (`.git`, `target`, etc.)
- Output stability

## v0.2.4: Robust Allowlist
- Regex support
- Scoped allowlist
- Config validation improvements

## v0.2.5: CI/Dev UX
- `veto install-hook`
- CI mode optimization
- GitHub Actions snippets

## v0.3.0: Baseline (Stop the Bleeding)
- `veto baseline create`
- `veto scan` with baseline support
- Stable baseline file format

## v0.4+ "Trust chain & Repro"
- Release signature verification
- Nix build reproducibility check
