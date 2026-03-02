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
