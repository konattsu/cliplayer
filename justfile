# run musictl binary with cargo
musictl-dev *args:
    cargo run --bin musictl {{args}}

alias mctl := musictl-dev


# run artistctl binary with cargo
artistctl-dev *args:
    cargo run --bin artistctl {{args}}

alias actl := artistctl-dev
