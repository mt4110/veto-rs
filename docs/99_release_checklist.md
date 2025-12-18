# 99 Release Checklist

Purpose: Ensure 100% reproducible releases.

## Pre-Release
- [ ] **Cleanliness**: Ensure no `._*` or `__MACOSX` files exist in the repo.
  - `find . -name '._*' -delete`
  - `git ls-files '._*' '**/._*'` should return nothing.
- [ ] **Tests**: Run full workspace tests.
  - `cargo test --workspace`
- [ ] **Docs**: Update `Last verified` dates in `docs/*.md` to today (YYYY-MM-DD).
- [ ] **Version**: Bump version in:
  - `flake.nix`
  - `crates/*/Cargo.toml`

## Release
- [ ] **Tag**: Create a signed tag.
  - `git tag -s vX.Y.Z -m "Release vX.Y.Z"`
  - `git verify-tag vX.Y.Z`
- [ ] **Archive**: Create the distribution tarball using `git archive` (NOT manual zip).
  - `git archive --format=tar.gz --prefix=veto-rs-vX.Y.Z/ -o veto-rs-vX.Y.Z.tar.gz vX.Y.Z`
- [ ] **Verify Archive**: Check the content of the tarball.
  - `tar -tf veto-rs-vX.Y.Z.tar.gz | grep '._'` -> Should be empty.
  - `tar -tf veto-rs-vX.Y.Z.tar.gz | grep 'target/'` -> Should be empty.

## Post-Release
- [ ] **Push**: `git push origin main --tags`
