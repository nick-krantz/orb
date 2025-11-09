use clap::Args;
use dirs;
use std::fs;

use crate::errors::{ErrorWithContext, Result};

#[derive(Args)]
#[group(required = true, multiple = true)]
pub struct CleanArgs {
    /// Remove all files and folders on the desktop
    #[clap(long)]
    desktop: bool,

    /// Remove all files and folders in the downloads folder
    #[clap(long)]
    downloads: bool,
}

fn clear_desktop() -> Result<()> {
    let desktop_path = dirs::desktop_dir()
        .ok_or_else(|| ErrorWithContext::new("Unable to find desktop directory", None::<String>))?;

    let desktop_contents = fs::read_dir(&desktop_path).map_err(|e| {
        ErrorWithContext::new("Failed to read desktop directory", Some(e.to_string()))
    })?;

    for entry in desktop_contents {
        let path = entry
            .map_err(|e| {
                ErrorWithContext::new("Failed to read desktop entry", Some(e.to_string()))
            })?
            .path();

        if path.is_file() {
            fs::remove_file(&path).map_err(|e| {
                ErrorWithContext::new(
                    format!("Failed to remove file: {:?}", path),
                    Some(e.to_string()),
                )
            })?;
        } else {
            println!("Skipping folder on desktop: {:?}", path);
        }
    }

    Ok(())
}

fn clear_downloads() -> Result<()> {
    let downloads_path = dirs::download_dir().ok_or_else(|| {
        ErrorWithContext::new("Unable to find downloads directory", None::<String>)
    })?;

    let downloads_contents = fs::read_dir(&downloads_path).map_err(|e| {
        ErrorWithContext::new("Failed to read downloads directory", Some(e.to_string()))
    })?;

    for entry in downloads_contents {
        let path = entry
            .map_err(|e| {
                ErrorWithContext::new("Failed to read downloads entry", Some(e.to_string()))
            })?
            .path();

        if path.is_file() {
            fs::remove_file(&path).map_err(|e| {
                ErrorWithContext::new(
                    format!("Failed to remove file: {:?}", path),
                    Some(e.to_string()),
                )
            })?;
        } else {
            println!("Skipping folder in downloads: {:?}", path);
        }
    }
    Ok(())
}

pub fn run(args: &CleanArgs) -> Result<()> {
    if args.desktop {
        clear_desktop()?;
    }

    if args.downloads {
        clear_downloads()?;
    }

    Ok(())
}
