fmt-clippy:
  cargo fmt --all -- --check
  cargo clippy --all-targets --all-features -- -D warnings
