# run musictl binary with cargo
musictl-dev *args:
    cargo run --bin musictl {{args}}

alias mctl := musictl-dev


# run artistctl binary with cargo
artistctl-dev *args:
    cargo run --bin artistctl {{args}}

alias actl := artistctl-dev


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

# create a new feature branch based off the remote main
# `branch`       : the name of the new branch to create
# `stale_branch` : an existing branch to delete first (ignored if it doesn't exist)
git-new branch stale_branch:
    git fetch -p
    git switch -d origin/main
    git branch -D {{stale_branch}} || true
    git switch -c {{branch}}
