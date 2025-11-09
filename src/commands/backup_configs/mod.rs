use std::{env, fs, path};

use crate::errors::{ErrorWithContext, Result};

/// Template for wrapping config file content in a markdown code block.
const MARKDOWN_TEMPLATE: &str = "```bash\n{content}\n```";

/// Moves the .gitconfig file into an obsidian markdown file for backup.
fn backup_gitconfig(obsidian_folder: &str) -> Result<()> {
    let home = env::var("HOME").map_err(|e| ErrorWithContext {
        user_message: "HOME environment variable not set".to_string(),
        technical_details: Some(e.to_string()),
    })?;
    let mut gitconfig_path = path::PathBuf::from(home);
    gitconfig_path.push(".gitconfig");
    let gitconfig = fs::read_to_string(&gitconfig_path).map_err(|e| ErrorWithContext {
        user_message: "Cannot read `.gitconfig` file".to_string(),
        technical_details: Some(e.to_string()),
    })?;

    let markdown = MARKDOWN_TEMPLATE.replace("{content}", &gitconfig);

    let mut output_path = path::PathBuf::from(obsidian_folder);
    output_path.push("gitconfig.md");
    fs::write(output_path, markdown).map_err(|e| ErrorWithContext {
        user_message: "Failed to write `.gitconfig` file to obsidian".to_string(),
        technical_details: Some(e.to_string()),
    })?;

    println!("✓ Backed up .gitconfig");
    Ok(())
}

/// Moves the .zshrc file into an obsidian markdown file for backup.
fn backup_zshrc(obsidian_folder: &str) -> Result<()> {
    let home = env::var("HOME").map_err(|e| ErrorWithContext {
        user_message: "HOME environment variable not set".to_string(),
        technical_details: Some(e.to_string()),
    })?;

    let mut zshrc_path = path::PathBuf::from(home);
    zshrc_path.push(".zshrc");
    let zshrc = fs::read_to_string(&zshrc_path).map_err(|e| ErrorWithContext {
        user_message: "Cannot read `.zshrc` file".to_string(),
        technical_details: Some(e.to_string()),
    })?;

    let markdown = MARKDOWN_TEMPLATE.replace("{content}", &zshrc);

    let mut output_path = path::PathBuf::from(obsidian_folder);
    output_path.push("zshrc.md");
    fs::write(output_path, markdown).map_err(|e| ErrorWithContext {
        user_message: "Failed to write `.zshrc` file to obsidian".to_string(),
        technical_details: Some(e.to_string()),
    })?;

    println!("✓ Backed up .zshrc");
    Ok(())
}

pub fn run() -> Result<()> {
    let obsidian_folder = env::var("OBSIDIAN_CONFIG_FOLDER").map_err(|e| ErrorWithContext {
        user_message: "OBSIDIAN_CONFIG_FOLDER environment variable not set".to_string(),
        technical_details: Some(e.to_string()),
    })?;

    backup_gitconfig(&obsidian_folder)?;
    backup_zshrc(&obsidian_folder)?;

    Ok(())
}
