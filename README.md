# Orb

A CLI tool for automating common system maintenance tasks in my own workflow. My way of learning Rust while building something useful.

- [Orb](#orb)
  - [Development](#development)
    - [Prerequisites](#prerequisites)
    - [Running Locally](#running-locally)
  - [Commands](#commands)
    - [`backup-configs`](#backup-configs)
    - [`bitwarden-pr`](#bitwarden-pr)
    - [`clean`](#clean)
    - [`colorx`](#colorx)
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

### `backup-configs`
Moves configuration files from home directory into an Obsidian vault folder. Obsidian is automatically backed up to GitHub. The destination folder needs to be set using the `OBSIDIAN_CONFIG_FOLDER` environment variable.

**Options:** None

**Example:** `orb backup-configs`

---

### `bitwarden-pr`
Opens the GitHub pull request page for a Bitwarden repository with prefilled information. This only works when a ticket number is found within the branch name. The expected branch format should follow the [contributing guidelines](https://contributing.bitwarden.com/contributing/pull-requests/#branch): `<team>/<issue-number>/<brief-description>`. The command will fail if a repo or non-bitwarden repository is found.

**Options:** N/A

**Example:** `orb bitwarden-pr`

---

### `clean`
Remove files from Desktop and/or Downloads folders (folders are skipped, only files removed)

**Options:** `--desktop` `--downloads` (at least one required)

**Example:** `orb clean --desktop --downloads`

---

### `colorx`
Converts hex color codes to RGB and vice versa.

**Options:** N/A

**Example:** `orb colorx #FF5733`, `orb colorx 255 87 51`, `orb colorx rgb(45, 45, 45)`

## Installation

1. Copy `scripts/build-and-install.example` to `scripts/build-and-install.sh`
2. Edit `scripts/build-and-install.sh` and replace `DEST_DIR` with a directory in your `PATH` (e.g., `/usr/local/bin` or `~/.local/bin`)
3. Make script executable: `chmod +x scripts/build-and-install.sh`
4. Run script to build and install the binary: `./scripts/build-and-install.sh`

## TODO / Command Ideas

- [ ] Clear local branches
- [ ] Copy converted color code to clipboard automatically
- [ ] Support alpha values in colorx command
- Create GitHub link from last commit. This will help with addressing feedback in VSCode's GH extension.
  - From within a git repo, generate a web link to the last commit. 
  - Possibly make it interactive? Choose commit?
- [x] Better error messaging - currently rust errors are shown to the user
- [x] Create PR with title + ticket from branch
    - How generic can this be? Very specific to Bitwarden but otherwise generic to use across their repos
    - Can it support a local template? Yes
- [x] Backup .gitconfig to obsidian
- [x] Clean out download and desktop of files
