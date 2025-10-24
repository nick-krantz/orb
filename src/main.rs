use clap::Parser;

mod commands;

use commands::{Cli, Commands};

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Clean(args) => commands::clean::run(&args)?,
    }

    Ok(())
}
