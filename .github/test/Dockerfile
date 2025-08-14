FROM ubuntu:latest

# @wsl (host)
#   ~/cliplayer$ docker build -t my-act-container-image -f ./.devcontainer/Dockerfile.act .
#   ~/cliplayer$ act -P ubuntu-latest=my-act-container-image --pull=false -j (任意)


# 必要なパッケージのインストール
RUN apt-get update && \
    apt-get install -y \
        curl \
        git \
        sudo \
        ca-certificates \
        build-essential \
        nodejs \
        npm

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
RUN git --version && gh --version && cargo --version && node --version && npm --version

CMD ["/bin/bash"]
