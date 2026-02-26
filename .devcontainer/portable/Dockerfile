FROM mcr.microsoft.com/vscode/devcontainers/typescript-node:24-bullseye

# install Taplo
#   ref: https://taplo.tamasfe.dev/cli/installation/binary.html
RUN curl -fsSL https://github.com/tamasfe/taplo/releases/latest/download/taplo-linux-x86_64.gz | gzip -d - | install -m 755 /dev/stdin /usr/local/bin/taplo

# install pnpm
#   we should match the version between devcontainer and package.json
RUN npm install -g pnpm@10.30.3
