use git2::Repository;
use regex::Regex;
use std::fs;
use std::process::Command;
use url::Url;

use crate::errors::{ErrorWithContext, Result};

const BITWARDEN_CLIENTS_URL: &str = "https://github.com/bitwarden/clients";
const BITWARDEN_SERVER_URL: &str = "https://github.com/bitwarden/server";
const BITWARDEN_SDK_URL: &str = "https://github.com/bitwarden/sdk-internal";

const PULL_REQUEST_TEMPLATE_PATH: &str = ".github/PULL_REQUEST_TEMPLATE.md";

// This is a little hacky but the template hasn't changed in over a year. I'm hardcoding it for now.
const TEMPLATE_TRACKING_HEADER: &str = "## 🎟️ Tracking\n\n<!-- Paste the link to the Jira or GitHub issue or otherwise describe / point to where this change is coming from. -->";

const BASE_JIRA_URL: &str = "https://bitwarden.atlassian.net/browse/";

/// Determines the Bitwarden repository type based on the remote URL.
fn get_bitwarden_repo(repo: &Repository) -> Option<String> {
    let remote = repo.find_remote("origin").ok()?;
    let url = remote.url()?;

    match url {
        url if url.contains("bitwarden/clients") => Some(BITWARDEN_CLIENTS_URL.to_string()),
        url if url.contains("bitwarden/server") => Some(BITWARDEN_SERVER_URL.to_string()),
        url if url.contains("bitwarden/sdk-internal") => Some(BITWARDEN_SDK_URL.to_string()),
        _ => None,
    }
}

/// Retrieves the current branch name of the repository.
fn get_branch_name(repo: &Repository) -> Result<String> {
    let head = repo.head().map_err(|e| ErrorWithContext {
        user_message: "Unable to retrieve git head".to_string(),
        technical_details: Some(e.message().to_string()),
    })?;

    let shorthand = head.shorthand().ok_or_else(|| ErrorWithContext {
        user_message: "Error getting branch shorthand name".to_string(),
        technical_details: None,
    })?;

    Ok(shorthand.to_string())
}

/// Extracts the ticket identifier from the branch name.
fn get_ticket_from_branch(branch: &str) -> Option<String> {
    let regular_expression = Regex::new(r"(\/[A-Za-z]{2}-[0-9]{1,5}\/)").ok()?;
    let matches = regular_expression.captures(branch)?;

    matches
        .get(1)
        .map(|m| m.as_str().to_string().to_uppercase().replace("/", ""))
}

/// Grab the pull request template from the repo
fn get_pull_request_template(repo: &Repository) -> String {
    let template_path = repo.workdir().unwrap().join(PULL_REQUEST_TEMPLATE_PATH);

    fs::read_to_string(template_path).unwrap()
}

fn create_pr_description(repo: &Repository, ticket: &str) -> String {
    let jira_link = format!("{}{}", BASE_JIRA_URL, ticket);
    let jira_link_markdown = format!("[{}]({})", ticket, jira_link);
    let template: String = get_pull_request_template(&repo);

    template.replace(
        TEMPLATE_TRACKING_HEADER,
        &format!("## 🎟️ Tracking\n\n{}", jira_link_markdown),
    )
}

fn construct_pull_request_url(
    repo_url: &str,
    branch: &str,
    title: &str,
    description: &str,
) -> Result<String> {
    let compare_path = format!("/compare/main...{}", branch);
    let full_url = format!("{}{}", repo_url, compare_path);

    let mut url = Url::parse(&full_url).map_err(|e| ErrorWithContext {
        user_message: format!("Invalid GitHub URL: {}", full_url),
        technical_details: Some(e.to_string()),
    })?;

    url.query_pairs_mut()
        .append_pair("quick_pull", "1")
        .append_pair("title", title)
        .append_pair("body", description);

    Ok(url.to_string())
}

pub fn run() -> Result<()> {
    let repo = Repository::discover(".").map_err(|e| ErrorWithContext {
        user_message: "GitHub repository not found".to_string(),
        technical_details: Some(e.to_string()),
    })?;

    let repo_url = get_bitwarden_repo(&repo).ok_or_else(|| ErrorWithContext {
        user_message: format!("Not a recognized Bitwarden repository"),
        technical_details: None,
    })?;

    let branch_name = get_branch_name(&repo)?;

    let ticket = get_ticket_from_branch(&branch_name).ok_or_else(|| ErrorWithContext {
        user_message: format!("No ticket found in branch name: {}", branch_name),
        technical_details: None,
    })?;

    let title = format!("[{}]", ticket);
    let description = create_pr_description(&repo, &ticket);

    let url = construct_pull_request_url(&repo_url, &branch_name, &title, &description)?;

    println!("Opening a new PR for");
    println!("  Repository: {}", repo_url);
    println!("  Branch: {}", branch_name);
    println!("  Ticket: {}", ticket);

    Command::new("open")
        .arg(&url)
        .status()
        .map_err(|e| ErrorWithContext {
            user_message: format!("Failed to open url: {}", url),
            technical_details: Some(e.to_string()),
        })?;

    Ok(())
}
