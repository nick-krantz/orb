use clap::Parser;

mod commands;
mod errors;

use commands::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Clean(args) => commands::clean::run(&args),
        Commands::BackupConfigs => commands::backup_configs::run(),
        Commands::BitwardenPr => commands::bitwarden_pr::run(),
        Commands::ColorX => commands::colorx::run(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e.user_message);

        if cli.verbose {
            if let Some(details) = &e.technical_details {
                eprintln!("\nTechnical details:\n{}", details);
            }
        }

        std::process::exit(1);
    }
}
