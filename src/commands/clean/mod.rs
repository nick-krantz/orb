use clap::Args;

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
    // Implementation to clear desktop files
    Ok(())
}

fn clear_downloads() -> std::io::Result<()> {
    // Implementation to clear downloads files
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
