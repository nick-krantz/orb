use std::{env, fs, io, path};

/// Template for wrapping config file content in a markdown code block.
const MARKDOWN_TEMPLATE: &str = "```bash\n{content}\n```";

/// Moves the .gitconfig file into an obsidian markdown file for backup.
fn backup_gitconfig(obsidian_folder: &str) -> io::Result<()> {
    let home = env::var("HOME").map_err(|_| {
        io::Error::new(io::ErrorKind::NotFound, "HOME environment variable not set")
    })?;
    let mut gitconfig_path = path::PathBuf::from(home);
    gitconfig_path.push(".gitconfig");

    let gitconfig = fs::read_to_string(&gitconfig_path)?;

    let markdown = MARKDOWN_TEMPLATE.replace("{content}", &gitconfig);

    let mut output_path = path::PathBuf::from(obsidian_folder);
    output_path.push("gitconfig.md");
    fs::write(output_path, markdown)?;

    println!("✓ Backed up .gitconfig");
    Ok(())
}

/// Moves the .zshrc file into an obsidian markdown file for backup.
fn backup_zshrc(obsidian_folder: &str) -> io::Result<()> {
    let home = env::var("HOME").map_err(|_| {
        io::Error::new(io::ErrorKind::NotFound, "HOME environment variable not set")
    })?;
    let mut zshrc_path = path::PathBuf::from(home);
    zshrc_path.push(".zshrc");
    let zshrc = fs::read_to_string(&zshrc_path)?;

    let markdown = MARKDOWN_TEMPLATE.replace("{content}", &zshrc);

    let mut output_path = path::PathBuf::from(obsidian_folder);
    output_path.push("zshrc.md");
    fs::write(output_path, markdown)?;

    println!("✓ Backed up .zshrc");
    Ok(())
}

pub fn run() -> io::Result<()> {
    let obsidian_folder = env::var("OBSIDIAN_CONFIG_FOLDER").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "OBSIDIAN_CONFIG_FOLDER environment variable not set",
        )
    })?;

    backup_gitconfig(&obsidian_folder)?;
    backup_zshrc(&obsidian_folder)?;

    Ok(())
}
