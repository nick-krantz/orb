use git2::Repository;
use regex::Regex;
use std::io::{Error, ErrorKind, Result};

const BITWARDEN_CLIENTS_URL: &str = "https://github.com/bitwarden/clients";
const BITWARDEN_SERVER_URL: &str = "https://github.com/bitwarden/server";
const BITWARDEN_SDK_URL: &str = "https://github.com/bitwarden/sdk-internal";

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
    let head = repo.head().map_err(|e| Error::new(ErrorKind::Other, e))?;
    let shorthand = head.shorthand().ok_or_else(|| {
        Error::new(
            ErrorKind::Other,
            "Failed to get branch name from repository head",
        )
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

pub fn run() -> Result<()> {
    let repo = Repository::discover(".").map_err(|e| Error::new(ErrorKind::Other, e))?;

    let repo_url = get_bitwarden_repo(&repo)
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Not a recognized Bitwarden repository"))?;

    println!("Repository URL: {}", repo_url);

    let branch_name = get_branch_name(&repo)?;

    println!("Current Branch: {}", branch_name);

    let ticket = get_ticket_from_branch(&branch_name)
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "No ticket found in branch name"))?;

    println!("Ticket: {}", ticket);
    // Create base title with ticket name
    // Create base description from template
    // Create PR URL with title and description from template
    // https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/using-query-parameters-to-create-a-pull-request
    Ok(())
}
