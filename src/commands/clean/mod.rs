use clap::Args;
use dirs;
use std::fs;

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

fn clear_desktop() -> std::io::Result<()> {
    let desktop_path = dirs::desktop_dir().unwrap();

    let desktop_contents = fs::read_dir(desktop_path)?;

    for entry in desktop_contents {
        let path = entry?.path();

        if path.is_file() {
            fs::remove_file(path)?;
        } else {
            println!("Skipping folder on desktop: {:?}", path);
        }
    }

    Ok(())
}

fn clear_downloads() -> std::io::Result<()> {
    let downloads_path = dirs::download_dir().unwrap();

    let downloads_contents = fs::read_dir(downloads_path)?;

    for entry in downloads_contents {
        let path = entry?.path();

        if path.is_file() {
            fs::remove_file(path)?;
        } else {
            println!("Skipping folder in downloads: {:?}", path);
        }
    }
    Ok(())
}

pub fn run(args: &CleanArgs) -> std::io::Result<()> {
    if args.desktop {
        clear_desktop()?;
    }

    if args.downloads {
        clear_downloads()?;
    }

    Ok(())
}
