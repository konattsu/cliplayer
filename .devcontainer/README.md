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

1. Copy the contents of your preferred configuration to `.devcontainer/` at the repository root
2. Open the repository in VS Code
3. Run "Reopen in Container" from the command palette

Or, VS Code should allow you to select which devcontainer to use directly if it detects multiple configurations.

## Changes from Original

Both configurations include the following improvements:
- Renamed service from `ts_react_dev` to `cliplayer-dev` for clarity
- Removed unused extensions: `mtxr.sqltools` and `ritwickdey.liveserver`
- Fixed English comment formatting in the rust extensions section
