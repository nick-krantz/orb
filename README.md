# Orb

A CLI tool for automating common system maintenance tasks.

- [Orb](#orb)
  - [Development](#development)
    - [Prerequisites](#prerequisites)
    - [Running Locally](#running-locally)
  - [Commands](#commands)
  - [Installation](#installation)
  - [TODO / Command Ideas](#todo--command-ideas)

## Development

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))

### Running Locally

1. Install Rust toolchain (see Prerequisites above)
2. Run command with `cargo run -- [command] [options]`
   - Example: `cargo run -- clean --help`

## Commands

Below are the current set of commands. Run with `--help` to see available options.

| Command        | Description                                                                                                                                                                                                                 | Options                                           | Example                           |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- | --------------------------------- |
| `clean`        | Remove files from Desktop and/or Downloads folders (folders are skipped, only files removed)                                                                                                                                | `--desktop` `--downloads` (at least one required) | `orb clean --desktop --downloads` |
| backup-configs | Moves configuration files from home directory into an Obsidian vault folder. Obsidian is automatically backed up to GitHub. The destination folder needs to be set using the `OBSIDIAN_CONFIG_FOLDER` environment variable. | None                                              | `orb backup-configs`              |

## Installation

1. Copy `scripts/build-and-install.example` to `scripts/build-and-install.sh`
2. Edit `scripts/build-and-install.sh` and replace `DEST_DIR` with a directory in your `PATH` (e.g., `/usr/local/bin` or `~/.local/bin`)
3. Make script executable: `chmod +x scripts/build-and-install.sh`
4. Run script to build and install the binary: `./scripts/build-and-install.sh`

## TODO / Command Ideas

- [ ] Clear local branches
- [ ] Better error messaging - currently rust errors are shown to the user
- Create GitHub link from last commit. This will help with addressing feedback in VSCode's GH extension.
  - From within a git repo, generate a web link to the last commit. 
  - Possibly make it interactive? Choose commit?
- [x] Create PR with title + ticket from branch
    - How generic can this be? Very specific to Bitwarden but otherwise generic to use across their repos
    - Can it support a local template? Yes
- [x] Backup .gitconfig to obsidian
- [x] Clean out download and desktop of files
