pub mod backup_configs;
pub mod bitwarden_pr;
pub mod clean;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "orb")]
#[clap(about = "A CLI tool for miscellaneous tasks", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Clean files and folders
    Clean(clean::CleanArgs),
    /// Move config files into obsidian markdown files to be backed up.
    BackupConfigs,
    BitwardenPr,
}
