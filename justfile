# run musictl binary with cargo to merge files
musictl-input-merge:
    cargo run --bin musictl -- dev merge-files

alias mi := musictl-input-merge

# run formatter for rust code
rust-fmt:
    cargo fmt --all
    cargo clippy --tests --examples

# run formatter for rust code without formatting
rust-fmt-check:
    cargo fmt --all
    cargo clippy --tests --examples -- -D clippy::all

# run codex with pnpm
codex:
    pnpm exec codex
