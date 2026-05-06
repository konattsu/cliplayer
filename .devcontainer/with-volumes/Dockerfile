FROM mcr.microsoft.com/vscode/devcontainers/typescript-node:24-bullseye

# install Taplo
# ref: https://taplo.tamasfe.dev/cli/installation/binary.html
RUN curl -fsSL https://github.com/tamasfe/taplo/releases/latest/download/taplo-linux-x86_64.gz \
  | gzip -d - | install -m 755 /dev/stdin /usr/local/bin/taplo

# install pnpm
# we should match the version between devcontainer and package.json
RUN npm install -g pnpm@10.33.3

# install just
# ref: https://github.com/casey/just#pre-built-binaries
RUN curl -fsSL https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# install Codex and Bubblewrap for sandboxing
RUN pnpm install @openai/codex && apt-get update && apt-get install bubblewrap -y

# install mold
RUN curl -fsSL -L https://github.com/rui314/mold/releases/download/v2.41.0/mold-2.41.0-x86_64-linux.tar.gz \
  | tar -xzf - -C /usr/local/bin --strip-components=2 mold-2.41.0-x86_64-linux/bin/mold

RUN apt-get install clang -y
