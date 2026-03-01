FROM ubuntu:latest

# This container is intended for local testing of our GitHub Actions
# using the `act` runner (https://github.com/nektos/act).  It mimics the
# `ubuntu-latest` environment that the workflows expect.  It is **not** used
# in the official CI; only developers invoke it manually on their machines.

# install basic packages
RUN apt-get update && \
    apt-get install -y \
        curl \
        git \
        sudo \
        ca-certificates \
        build-essential \
        nodejs \
        npm

# install pnpm so that front-end workflows can run without additional setup
# the version is kept in sync with package.json / devcontainer configurations
RUN npm install -g pnpm@10.30.3

# gh (GitHub CLI) のインストール
RUN type -p curl >/dev/null || apt-get install -y curl && \
    curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | \
        dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg && \
    chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) \
        signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] \
        https://cli.github.com/packages stable main" | \
        tee /etc/apt/sources.list.d/github-cli.list > /dev/null && \
    apt-get update && \
    apt-get install -y gh

# Rust (cargo含む) のインストール
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# RustのPATHを通す
ENV PATH="/root/.cargo/bin:${PATH}"

# バージョン確認コマンド
RUN git --version && gh --version && cargo --version && node --version && pnpm --version

CMD ["/bin/bash"]


# 使い方
# 1. build the helper image (one‑time; rebuild whenever you change the Dockerfile)
# docker build -t cliplayer-act -f .github/test/Dockerfile .
#
# # 2. invoke `act`, telling it to use that image instead of downloading one
# #    and to execute the `build` job defined in rust_tools_test.yml.
# #    (omit -j to run the default job, if any.)
# act \
#   -P ubuntu-latest=cliplayer-act \      # use our pre‑built image
#   --pull=false \                        # don’t try to pull a fresh ubuntu image
#   -W .github/workflows/rust_tools_test.yml \  # explicitly load this workflow file
#   -j build                              # run the “build” job
#
# # you can also run the workflow without specifying -j; act will then pick
# # the first job in the file (which happens to be “build” anyway):
# act -P ubuntu-latest=cliplayer-act --pull=false \
#     -W .github/workflows/rust_tools_test.yml
#
