# Devcontainer Configurations

This repository provides two devcontainer configurations to suit different development environments:

## 1. `with-volumes/` - Environment-specific setup

This configuration is optimized for specific host environments with custom volume mounts:
- Maps `~/cliplayer` to `/workspaces` in the container
- Persists npm cache via `~/.npm` volume mount
- Includes SSH agent socket forwarding for git operations

**Use this if:**
- You have the repository cloned in `~/cliplayer` on your host machine
- You want npm package caching across container rebuilds
- You need SSH agent forwarding for git operations

## 2. `portable/` - General-purpose setup

This configuration works in any environment without hardcoded paths:
- Uses generic volume mounts that work from any directory
- No npm cache persistence (relies on container's npm cache)
- No SSH agent socket forwarding

**Use this if:**
- You've cloned the repository in any directory
- You're working in a different environment where the specific paths don't exist
- You want a simpler, more portable setup

## How to Use

When you open this repository in VS Code with the Dev Containers extension installed, VS Code will detect multiple devcontainer configurations and prompt you to select which one to use.

Alternatively, you can:
1. Use the Command Palette (Ctrl+Shift+P / Cmd+Shift+P)
2. Type "Dev Containers: Reopen in Container"
3. Select your preferred configuration from the list

**Note:** If you want to set a default configuration, you can create a symlink or move one of the configurations to `.devcontainer/devcontainer.json` at the repository root, though this is not recommended as it may cause conflicts.

## Changes from Original

Both configurations include the following improvements:
- Renamed service from `ts_react_dev` to `cliplayer-dev` for clarity
- Removed unused extensions: `mtxr.sqltools` and `ritwickdey.liveserver`
- Fixed English comment formatting in the rust extensions section
