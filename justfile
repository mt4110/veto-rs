set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

fmt:
  cargo fmt --all

clippy:
  cargo clippy --workspace --all-targets -- -D warnings

test:
  cargo test --workspace

run:
  cargo run -p veto-cli -- scan

ci: fmt clippy test
