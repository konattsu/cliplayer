FROM mcr.microsoft.com/vscode/devcontainers/typescript-node:22-bullseye

# install Taplo
#   ref: https://taplo.tamasfe.dev/cli/installation/binary.html
RUN curl -fsSL https://github.com/tamasfe/taplo/releases/latest/download/taplo-linux-x86_64.gz | gzip -d - | install -m 755 /dev/stdin /usr/local/bin/taplo
