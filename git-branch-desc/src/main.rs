use anyhow::{Context, Result};
use arboard::Clipboard;
use clap::{Parser, Subcommand};
use git2::Repository;
use regex::Regex;
use serde_json::Value;
use reqwest::blocking::Client;
use std::collections::HashSet;
use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::Path;
use std::process::Command;
use tabwriter::TabWriter;
use terminal_size::{Width, terminal_size};

#[derive(Parser)]
#[command(name = "git-branch-desc")]
#[command(about = "A tool to manage branch descriptions stored in BRANCHREADME.md files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Edit description for a branch (defaults to current branch)
    #[command(alias = "e")]
    Edit {
        /// Target branch name (defaults to current branch)
        #[arg(short, long)]
        branch: Option<String>,
        /// Description text (if not provided, will prompt for input)
        description: Option<String>,
        /// Read description from clipboard
        #[arg(long, conflicts_with_all = ["description", "stdin"])]
        clipboard: bool,
        /// Read description from stdin
        #[arg(long, conflicts_with_all = ["description", "clipboard", "issue"])]
        stdin: bool,
        /// Read description from GitLab issue (number or URL)
        #[arg(long, conflicts_with_all = ["description", "clipboard", "stdin"])]
        issue: Option<String>,
        /// Use AI to summarize content (works with --issue, --stdin, --clipboard, and Ollama running locally)
        #[arg(long)]
        ai_summarize: bool,
        /// Timeout in seconds for AI processing (default: 120)
        #[arg(long, default_value = "120")]
        ai_timeout: u64,
        /// Automatically commit the BRANCHREADME.md file after editing
        #[arg(short, long)]
        commit: bool,
        /// Automatically commit and push the BRANCHREADME.md file after editing
        #[arg(short, long)]
        push: bool,
        /// Skip confirmation prompts (force operation)
        #[arg(short, long)]
        force: bool,
    },
    /// List all local and remote branch descriptions
    #[command(alias = "ls")]
    List {
        /// Show full descriptions instead of truncated table view
        #[arg(short, long)]
        detailed: bool,
        /// Show all branches, including those without descriptions
        #[arg(short, long)]
        all: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Edit {
            branch,
            description,
            clipboard,
            stdin,
            issue,
            ai_summarize,
            ai_timeout,
            commit,
            push,
            force,
        } => edit_description(
            branch,
            description,
            clipboard,
            stdin,
            issue,
            ai_summarize,
            ai_timeout,
            commit,
            push,
            force,
        ),
        Commands::List { detailed, all } => list_descriptions(detailed, all),
    }
}

fn edit_description(
    target_branch: Option<String>,
    description: Option<String>,
    clipboard: bool,
    stdin: bool,
    issue: Option<String>,
    ai_summarize: bool,
    ai_timeout: u64,
    commit: bool,
    push: bool,
    force: bool,
) -> Result<()> {
    // Validate AI summarize usage
    if ai_summarize && description.is_none() && !clipboard && !stdin && issue.is_none() {
        anyhow::bail!(
            "The --ai-summarize flag requires an input method. \
            Use with --issue, --clipboard, or --stdin."
        );
    }

    let repo = Repository::open(".")
        .context("Failed to open repository. Make sure you're in a Git repository.")?;

    let current_branch = get_current_branch(&repo)?;
    let target_branch = target_branch.unwrap_or(current_branch.clone());
    let is_current_branch = target_branch == current_branch;

    // Validate that the target branch exists if it's different from current
    if !is_current_branch {
        validate_branch_exists(&repo, &target_branch)?;

        // Warn user about modifying a different branch (unless --force is used)
        if force {
            println!("🚀 Force mode: Editing branch '{target_branch}' without confirmation");
        } else {
            println!(
                "⚠️  WARNING: You are about to edit branch '{target_branch}' while on '{current_branch}'"
            );

            if commit || push {
                println!("This will create a commit directly on the target branch.");
            }

            print!("Continue? [y/N]: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let confirmation = input.trim().to_lowercase();

            if confirmation != "y" && confirmation != "yes" {
                println!("Operation cancelled.");
                return Ok(());
            }
        }
    }

    // Check if description already exists
    let existing_description = if is_current_branch {
        read_current_branch_description().unwrap_or_default()
    } else {
        read_branch_description_from_git(&repo, &target_branch)
            .unwrap_or(None)
            .unwrap_or_default()
    };

    let is_new = existing_description.is_empty();

    let description = if let Some(desc) = description {
        if ai_summarize {
            ai_summarize_content(&desc, ai_timeout)
                .context("Failed to summarize content using AI")?
        } else {
            desc
        }
    } else if clipboard {
        let content = get_clipboard_content()?;
        if ai_summarize {
            ai_summarize_content(&content, ai_timeout)
                .context("Failed to summarize clipboard content using AI")?
        } else {
            content
        }
    } else if stdin {
        let content = get_stdin_content()?;
        if ai_summarize {
            ai_summarize_content(&content, ai_timeout)
                .context("Failed to summarize stdin content using AI")?
        } else {
            content
        }
    } else if let Some(issue_ref) = issue {
        get_issue_content(&issue_ref, ai_summarize, ai_timeout)?
    } else {
        get_interactive_input(&target_branch, &existing_description)?
    };

    if description.is_empty() {
        println!("No description provided. Operation cancelled.");
        return Ok(());
    }

    if is_current_branch {
        write_current_branch_description(&description)?;
    } else {
        // For non-current branches, we only prepare the tree but don't commit yet
        // The actual commit happens in the commit_to_branch function
        if !commit && !push {
            println!(
                "⚠️  WARNING: Description prepared for branch '{target_branch}' but not committed."
            );
            println!("Use --commit or --push to save the changes to the branch.");
            println!("Without committing, the description exists only temporarily.");
            return Ok(());
        }
    }

    let action = if is_new { "Added" } else { "Updated" };
    println!("✅ {action} description for branch '{target_branch}'");

    // Handle commit and push options
    if push || commit {
        if is_current_branch {
            // Use traditional approach for current branch
            commit_current_branch_changes(&repo, &target_branch, !is_new, push)?;
        } else {
            // Use low-level Git operations for non-current branch
            commit_to_branch(&repo, &target_branch, &description, !is_new, push)?;
        }
    }

    Ok(())
}

struct BranchDescription {
    branch: String,
    description: String,
}

fn get_terminal_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        (w as usize * 90) / 100 // Use 90% of terminal width
    } else {
        80 // Default width
    }
}

fn wrap_text(text: &str, max_width: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        // If adding this word would exceed the max width, start a new line
        if !current_line.is_empty() && current_line.len() + 1 + word.len() > max_width {
            lines.push(current_line.clone());
            current_line = word.to_string();
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }

    // Don't forget the last line
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines.join("\n")
}

fn list_descriptions(detailed: bool, all: bool) -> Result<()> {
    let repo = Repository::open(".")
        .context("Failed to open repository. Make sure you're in a Git repository.")?;

    let mut descriptions = Vec::new();
    let local_branches = get_local_branch_list(&repo)?;
    let mut processed_branches = HashSet::new();

    // First, process remote branches
    let remotes = repo.remotes()?;
    for remote_name in remotes.iter() {
        if let Some(remote_name) = remote_name {
            let remote_refs = repo.references_glob(&format!("refs/remotes/{remote_name}/*"))?;
            for reference in remote_refs {
                if let Ok(reference) = reference {
                    if let Some(branch_name) = reference.shorthand() {
                        if let Some(branch_name) =
                            branch_name.strip_prefix(&format!("{remote_name}/"))
                        {
                            if branch_name == "HEAD" {
                                continue;
                            }

                            if let Some(desc) = process_branch_description(
                                &repo,
                                &format!("{remote_name}/{branch_name}"),
                                true,
                                all,
                            ) {
                                descriptions.push(desc);
                                processed_branches.insert(branch_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // Then process local branches that haven't been processed as remotes
    for branch_name in local_branches {
        if !processed_branches.contains(&branch_name) {
            if let Some(desc) = process_branch_description(&repo, &branch_name, false, all) {
                descriptions.push(desc);
            }
        }
    }

    if descriptions.is_empty() && !all {
        println!("No branch descriptions found. Use --all to see all branches.");
        return Ok(());
    }

    if detailed {
        for desc in descriptions {
            println!("Branch: {}", desc.branch);
            println!("Description:");
            println!("{}", desc.description);
            println!("{}", "-".repeat(50));
        }
    } else {
        let terminal_width = get_terminal_width();
        let max_desc_width = if terminal_width > 30 {
            terminal_width - 30
        } else {
            50
        };

        let mut tw = TabWriter::new(io::stdout());
        writeln!(tw, "BRANCH\tDESCRIPTION")?;
        writeln!(tw, "------\t-----------")?;

        for desc in descriptions {
            let wrapped_desc = if desc.description.len() > max_desc_width {
                let truncated = &desc.description[..max_desc_width.saturating_sub(3)];
                format!("{}...", truncated)
            } else {
                wrap_text(&desc.description, max_desc_width)
            };

            writeln!(tw, "{}\t{}", desc.branch, wrapped_desc.replace('\n', " "))?;
        }

        tw.flush()?;
    }

    Ok(())
}

fn process_branch_description(
    repo: &Repository,
    full_branch_name: &str,
    _is_remote: bool,
    include_all: bool,
) -> Option<BranchDescription> {
    let branch_name = full_branch_name;

    if let Ok(Some(description)) = read_branch_description_from_git(repo, full_branch_name) {
        if !description.trim().is_empty() {
            return Some(BranchDescription {
                branch: branch_name.to_string(),
                description: description.trim().to_string(),
            });
        }
    }

    if include_all {
        Some(BranchDescription {
            branch: branch_name.to_string(),
            description: "(no description)".to_string(),
        })
    } else {
        None
    }
}

fn get_current_branch(repo: &Repository) -> Result<String> {
    let head = repo.head().context("Failed to get HEAD reference")?;
    let branch_name = head.shorthand().context("Failed to get branch name")?;
    Ok(branch_name.to_string())
}

fn read_current_branch_description() -> Result<String> {
    match fs::read_to_string("BRANCHREADME.md") {
        Ok(content) => Ok(content),
        Err(_) => Ok(String::new()),
    }
}

fn write_current_branch_description(description: &str) -> Result<()> {
    fs::write("BRANCHREADME.md", description).context("Failed to write BRANCHREADME.md file")?;
    Ok(())
}

fn commit_current_branch_changes(
    repo: &Repository,
    branch_name: &str,
    is_modify: bool,
    push: bool,
) -> Result<()> {
    // Stage the BRANCHREADME.md file
    let mut index = repo.index().context("Failed to get repository index")?;
    index
        .add_path(Path::new("BRANCHREADME.md"))
        .context("Failed to stage BRANCHREADME.md")?;
    index.write().context("Failed to write index")?;

    // Create commit
    let signature = repo.signature().context("Failed to get git signature")?;
    let tree_id = index.write_tree().context("Failed to write tree")?;
    let tree = repo.find_tree(tree_id).context("Failed to find tree")?;

    let parent_commit = repo
        .head()
        .and_then(|head| head.peel_to_commit())
        .context("Failed to get parent commit")?;

    let commit_message = format!(
        "{} branch description [skip ci]",
        if is_modify { "Update" } else { "Add" }
    );

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &commit_message,
        &tree,
        &[&parent_commit],
    )
    .context("Failed to create commit")?;

    println!("Committed BRANCHREADME.md");

    // Push if requested
    if push {
        // Find the remote and push
        // Use system git command for pushing since git2 has SSH issues
        let output = std::process::Command::new("git")
            .args(["push", "origin", branch_name])
            .output()
            .context("Failed to execute git push command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Git push failed: {}", stderr));
        }

        println!("Pushed to origin/{branch_name}");
    }

    Ok(())
}

fn commit_to_branch(
    repo: &Repository,
    branch_name: &str,
    description: &str,
    is_modify: bool,
    push: bool,
) -> Result<()> {
    // Get the target branch reference
    let branch_ref_name = format!("refs/heads/{branch_name}");
    let branch_ref = repo
        .find_reference(&branch_ref_name)
        .with_context(|| format!("Failed to find branch: {branch_name}"))?;

    // Check if there are any uncommitted changes that might conflict
    if repo.state() != git2::RepositoryState::Clean {
        println!(
            "⚠️  WARNING: Repository has uncommitted changes. These will not interfere with the remote branch operation."
        );
    }

    // Get the current commit
    let target_commit_oid = branch_ref.target().context("Failed to get target commit")?;
    let target_commit = repo
        .find_commit(target_commit_oid)
        .context("Failed to find commit")?;

    // Get the current tree
    let current_tree = target_commit
        .tree()
        .context("Failed to get tree from commit")?;

    // Create a new blob with the description content
    let blob_oid = repo
        .blob(description.as_bytes())
        .context("Failed to create blob for description")?;

    // Create a new tree with the BRANCHREADME.md file
    let mut tree_builder = repo
        .treebuilder(Some(&current_tree))
        .context("Failed to create tree builder")?;
    tree_builder
        .insert("BRANCHREADME.md", blob_oid, 0o100644)
        .context("Failed to insert BRANCHREADME.md into tree")?;
    let new_tree_oid = tree_builder.write().context("Failed to write new tree")?;
    let new_tree = repo
        .find_tree(new_tree_oid)
        .context("Failed to find new tree")?;

    // Create a new commit
    let signature = repo.signature().context("Failed to get git signature")?;
    let commit_message = format!(
        "{} branch description [skip ci]",
        if is_modify { "Update" } else { "Add" }
    );

    let new_commit_oid = repo
        .commit(
            Some(&branch_ref_name), // Update the branch reference directly
            &signature,
            &signature,
            &commit_message,
            &new_tree,
            &[&target_commit], // Parent commit
        )
        .context("Failed to create commit")?;

    println!("✅ Committed BRANCHREADME.md to branch '{branch_name}' ({new_commit_oid})");

    // Push if requested
    if push {
        println!("🚀 Pushing to remote...");

        // Check if remote branch exists
        let remote_ref = format!("refs/remotes/origin/{branch_name}");
        let remote_exists = repo.find_reference(&remote_ref).is_ok();

        if !remote_exists {
            println!("📡 Creating new remote branch origin/{branch_name}");
        }

        // Use system git command for pushing since git2 has SSH issues
        let output = std::process::Command::new("git")
            .args(["push", "origin", branch_name])
            .output()
            .context("Failed to execute git push command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Git push failed: {}", stderr));
        }

        println!("✅ Pushed to origin/{branch_name}");
    }

    Ok(())
}

fn validate_branch_exists(repo: &Repository, branch_name: &str) -> Result<()> {
    // Check local branches first
    let local_ref = format!("refs/heads/{branch_name}");
    if repo.find_reference(&local_ref).is_ok() {
        return Ok(());
    }

    // Check remote branches
    let remote_ref = format!("refs/remotes/origin/{branch_name}");
    if repo.find_reference(&remote_ref).is_ok() {
        return Ok(());
    }

    Err(anyhow::anyhow!(
        "Branch '{branch_name}' not found. Available branches:\n{}",
        get_local_branch_list(repo)?
            .iter()
            .map(|b| format!("  {b}"))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}

fn get_local_branch_list(repo: &Repository) -> Result<Vec<String>> {
    let mut branches = Vec::new();
    let branch_iter = repo.branches(Some(git2::BranchType::Local))?;

    for branch in branch_iter {
        let (branch, _) = branch.context("Failed to get branch")?;
        if let Some(name) = branch.name().context("Failed to get branch name")? {
            branches.push(name.to_string());
        }
    }

    Ok(branches)
}

fn get_clipboard_content() -> Result<String> {
    let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
    let content = clipboard
        .get_text()
        .context("Failed to read from clipboard")?;
    Ok(content.trim().to_string())
}

fn get_stdin_content() -> Result<String> {
    if io::stdin().is_terminal() {
        anyhow::bail!(
            "No input detected on stdin. Use --clipboard or provide description as argument instead."
        );
    }

    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .context("Failed to read from stdin")?;
    Ok(content.trim().to_string())
}

fn get_interactive_input(target_branch: &str, existing_description: &str) -> Result<String> {
    // Always show existing description if it exists (unified behavior)
    if !existing_description.is_empty() {
        println!("Current description for branch '{target_branch}':");
        println!("{existing_description}");
        println!();
    }

    print!("Enter description for branch '{target_branch}': ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_issue_content(issue_ref: &str, ai_summarize: bool, ai_timeout: u64) -> Result<String> {
    // Parse the issue reference - could be a number or a URL
    let issue_number = parse_issue_reference(issue_ref)?;

    // Use glab to get issue information
    let output = Command::new("glab")
        .args(["issue", "view", &issue_number, "--output", "json"])
        .output()
        .context("Failed to execute glab command. Make sure glab is installed and configured.")?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("glab command failed: {}", error_msg);
    }

    let json_output =
        String::from_utf8(output.stdout).context("Failed to parse glab output as UTF-8")?;

    // Parse the JSON output to extract title and description
    let raw_content = parse_issue_json(&json_output)?;
    
    // Apply AI summarization if requested
    if ai_summarize {
        ai_summarize_content(&raw_content, ai_timeout)
            .context("Failed to summarize content using AI")
    } else {
        Ok(raw_content)
    }
}

fn ai_summarize_content(content: &str, timeout_seconds: u64) -> Result<String> {
    // Validate content length - git diffs can be very large
    const MAX_CONTENT_LENGTH: usize = 8000; // Reasonable limit for AI processing
    let content_to_process = if content.len() > MAX_CONTENT_LENGTH {
        println!("Content is large ({} chars), truncating to {} chars for AI processing...", 
                content.len(), MAX_CONTENT_LENGTH);
        &content[..MAX_CONTENT_LENGTH]
    } else {
        content
    };

    // Create HTTP client with configurable timeout
    let timeout_duration = std::time::Duration::from_secs(timeout_seconds);
    let client = Client::builder()
        .timeout(timeout_duration)
        .build()
        .context("Failed to create HTTP client")?;
    
    // Check if Ollama is running with shorter timeout for connection test
    let test_client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .context("Failed to create test HTTP client")?;
        
    match test_client.get("http://localhost:11434/api/tags").send() {
        Ok(response) => {
            if !response.status().is_success() {
                anyhow::bail!(
                    "Ollama is running but returned error {}. Try: ollama run llama3.2:1b", 
                    response.status()
                );
            }
        }
        Err(e) => {
            anyhow::bail!(
                "Cannot connect to Ollama: {}\n\
                Please ensure Ollama is running:\n\
                1. Install Ollama from https://ollama.ai\n\
                2. Run: ollama run llama3.2:1b\n\
                3. Keep Ollama running and try again", e
            );
        }
    }

    // Adjust prompt based on content type
    let prompt = if content_to_process.contains("diff --git") || content_to_process.contains("@@") {
        format!(
            "Create a concise branch description (2-3 sentences max) for this git diff. \
            Focus on what changes are being made and why, not technical implementation details. \
            Respond with ONLY the branch description, no preamble or explanation:\n\n{}",
            content_to_process
        )
    } else {
        format!(
            "Create a concise branch description (2-3 sentences max) for this content. \
            Focus on the main task/goal, not implementation details. \
            Respond with ONLY the branch description, no preamble or explanation:\n\n{}",
            content_to_process
        )
    };

    let request_body = serde_json::json!({
        "model": "llama3.2:1b",
        "prompt": prompt,
        "stream": false,
        "options": {
            "temperature": 0.2,
            "top_p": 0.8,
            "num_predict": 100
        }
    });

    println!("Sending content to AI for summarization (timeout: {}s)...", timeout_seconds);
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .with_context(|| format!("Failed to send request to Ollama within {} seconds", timeout_seconds))?;

    if !response.status().is_success() {
        anyhow::bail!("Ollama API returned error: {}", response.status());
    }

    let response_json: Value = response
        .json()
        .context("Failed to parse Ollama response")?;

    let raw_summary = response_json["response"]
        .as_str()
        .context("Failed to extract summary from Ollama response")?
        .trim();

    if raw_summary.is_empty() {
        anyhow::bail!("Ollama returned empty summary");
    }

    // Clean up common AI preamble patterns
    let summary = clean_ai_preamble(raw_summary);

    Ok(summary.to_string())
}

fn clean_ai_preamble(text: &str) -> &str {
    let text = text.trim();
    
    // Common preamble patterns to remove
    let preambles = [
        "Here is a concise branch description for the GitLab issue:",
        "Here is a concise branch description:",
        "Here's a concise branch description:",
        "A concise branch description:",
        "Branch description:",
        "Here is the branch description:",
        "Here's the branch description:",
    ];
    
    for preamble in &preambles {
        if let Some(stripped) = text.strip_prefix(preamble) {
            return stripped.trim().trim_matches('"').trim();
        }
    }
    
    // Also check for patterns that end with colon and newline
    if let Some(colon_pos) = text.find(':') {
        let before_colon = &text[..colon_pos];
        if before_colon.to_lowercase().contains("description") || 
           before_colon.to_lowercase().contains("branch") {
            let after_colon = &text[colon_pos + 1..];
            return after_colon.trim().trim_matches('"').trim();
        }
    }
    
    text.trim_matches('"').trim()
}


fn parse_issue_reference(issue_ref: &str) -> Result<String> {
    // Check if it's a GitLab issue URL
    let url_regex = Regex::new(r"https?://[^/]+/[^/]+/[^/]+/-/issues/(\d+)")
        .context("Failed to compile URL regex")?;

    if let Some(captures) = url_regex.captures(issue_ref) {
        if let Some(number) = captures.get(1) {
            return Ok(number.as_str().to_string());
        }
    }

    // Check if it's just a number
    if issue_ref.chars().all(|c| c.is_ascii_digit()) {
        return Ok(issue_ref.to_string());
    }

    anyhow::bail!(
        "Invalid issue reference: '{}'. Expected either an issue number (e.g., '123') or a GitLab issue URL.",
        issue_ref
    );
}

fn parse_issue_json(json: &str) -> Result<String> {
    // Parse JSON using serde_json for robust parsing
    let parsed: Value = serde_json::from_str(json)
        .context("Failed to parse JSON output from glab")?;

    // Extract title
    let title = parsed["title"]
        .as_str()
        .context("Could not extract issue title from glab output")?;

    // Extract description (handle null and empty cases)
    let description = parsed["description"]
        .as_str()
        .unwrap_or("")
        .trim();

    // Format the content as title + description
    let mut content = format!("{}", title);
    if !description.is_empty() {
        content.push_str(&format!("\n\n{}", description));
    }

    Ok(content)
}

fn read_branch_description_from_git(
    repo: &Repository,
    branch_name: &str,
) -> Result<Option<String>> {
    // Try to find the branch reference
    let branch_ref =
        if let Ok(branch_ref) = repo.find_reference(&format!("refs/heads/{branch_name}")) {
            branch_ref
        } else if let Ok(branch_ref) = repo.find_reference(&format!("refs/remotes/{branch_name}")) {
            branch_ref
        } else {
            return Ok(None);
        };

    // Get the commit that the branch points to
    let commit_oid = branch_ref.target().context("Failed to get target commit")?;
    let commit = repo
        .find_commit(commit_oid)
        .context("Failed to find commit")?;

    // Get the tree from the commit
    let tree = commit.tree().context("Failed to get tree from commit")?;

    // Try to find the BRANCHREADME.md file in the tree
    match tree.get_name("BRANCHREADME.md") {
        Some(entry) => {
            let blob = repo
                .find_blob(entry.id())
                .context("Failed to find blob for BRANCHREADME.md")?;
            let content = String::from_utf8(blob.content().to_vec())
                .context("Failed to convert blob content to UTF-8")?;
            Ok(Some(content))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_issue_reference() {
        // Test issue number
        assert_eq!(parse_issue_reference("123").unwrap(), "123");
        
        // Test GitLab URL
        assert_eq!(
            parse_issue_reference("https://gitlab.com/owner/repo/-/issues/456").unwrap(),
            "456"
        );
        
        // Test GitLab URL with different domain
        assert_eq!(
            parse_issue_reference("https://gitlab.example.com/group/project/-/issues/789").unwrap(),
            "789"
        );
        
        // Test invalid input
        assert!(parse_issue_reference("invalid").is_err());
        assert!(parse_issue_reference("https://github.com/owner/repo/issues/123").is_err());
    }

    #[test]
    fn test_parse_issue_json() {
        // Test with actual failing JSON sample from the error report
        let actual_sample = r#"{"id":172148901,"iid":15,"external_id":"","state":"opened","description":"Currently the project builds on Jenkins using the @jenkinsfile. We need to migrate this to a GitLab CI/CD pipeline.\n\nThe Jenkins pipeline currently:\n\n- Builds using VS2017 with MsBuild\n- Runs serverbuild.cmd and serverbuild_msm.cmd\n- Generates TPTrackSvc_x64.msm and TPTrackSvc_x86.msm files\n- Deploys artifacts using Maven\n- Creates git tags\n\nReference: Jenkinsfile in repository root\n\nrelates to #16+s","health_status":"","title":"add a build pipeline for build and generating the msm, former build on jenkins","created_at":"2025-08-20T12:20:31.835Z"}"#;
        
        let actual_result = parse_issue_json(actual_sample).unwrap();
        assert!(actual_result.starts_with("add a build pipeline for build and generating the msm, former build on jenkins"));
        assert!(actual_result.contains("Currently the project builds on Jenkins"));
        assert!(actual_result.contains("We need to migrate this to a GitLab CI/CD pipeline"));
        
        // Test with real glab JSON output format
        let json = r#"{"id":172148901,"iid":15,"title":"add a build pipeline for build and generating the msm, former build on jenkins","description":"Currently the project builds on Jenkins using the @jenkinsfile. We need to migrate this to a GitLab CI/CD pipeline.\n\nThe Jenkins pipeline currently:\n\n- Builds using VS2017 with MsBuild\n- Runs serverbuild.cmd and serverbuild_msm.cmd\n- Generates TPTrackSvc_x64.msm and TPTrackSvc_x86.msm files\n- Deploys artifacts using Maven\n- Creates git tags\n\nReference: Jenkinsfile in repository root\n\nrelates to #16+s","state":"opened"}"#;
        
        let result = parse_issue_json(json).unwrap();
        assert!(result.starts_with("add a build pipeline for build and generating the msm, former build on jenkins"));
        assert!(result.contains("Currently the project builds on Jenkins"));
        assert!(result.contains("We need to migrate this to a GitLab CI/CD pipeline"));
        
        // Test with simple JSON
        let simple_json = r#"{"title":"Simple Title","description":"Simple description"}"#;
        let simple_result = parse_issue_json(simple_json).unwrap();
        assert_eq!(simple_result, "Simple Title\n\nSimple description");
        
        // Test with null description
        let null_desc_json = r#"{"title":"Title Only","description":null}"#;
        let null_result = parse_issue_json(null_desc_json).unwrap();
        assert_eq!(null_result, "Title Only");
        
        // Test with empty description
        let empty_desc_json = r#"{"title":"Title Only","description":""}"#;
        let empty_result = parse_issue_json(empty_desc_json).unwrap();
        assert_eq!(empty_result, "Title Only");
        
        // Test with escaped characters
        let escaped_json = r#"{"title":"Title with \"quotes\"","description":"Description with\nnewlines and \"quotes\""}"#;
        let escaped_result = parse_issue_json(escaped_json).unwrap();
        assert!(escaped_result.contains("Title with \"quotes\""));
        assert!(escaped_result.contains("Description with\nnewlines and \"quotes\""));
        
        // Test with complex nested JSON (like real glab output)
        let complex_json = r#"{"id":123,"title":"Complex Issue","description":"Multi-line\ndescription","author":{"name":"Test User"},"labels":["bug","priority::high"]}"#;
        let complex_result = parse_issue_json(complex_json).unwrap();
        assert_eq!(complex_result, "Complex Issue\n\nMulti-line\ndescription");
        
        // Test with missing description field
        let no_desc_json = r#"{"id":123,"title":"Title Only","state":"open"}"#;
        let no_desc_result = parse_issue_json(no_desc_json).unwrap();
        assert_eq!(no_desc_result, "Title Only");
        
        // Test error case - missing title
        let no_title_json = r#"{"id":123,"description":"Description only"}"#;
        assert!(parse_issue_json(no_title_json).is_err());
        
        // Test error case - invalid JSON
        let invalid_json = r#"{"title":"Invalid JSON" missing bracket"#;
        assert!(parse_issue_json(invalid_json).is_err());
    }

    #[test]
    fn test_clean_ai_preamble() {
        // Test removing various preamble patterns
        assert_eq!(
            clean_ai_preamble("Here is a concise branch description for the GitLab issue: Update Jenkins pipeline"),
            "Update Jenkins pipeline"
        );
        
        assert_eq!(
            clean_ai_preamble("Here's a concise branch description: Fix authentication bug"),
            "Fix authentication bug"
        );
        
        assert_eq!(
            clean_ai_preamble("Branch description: Implement OAuth2 system"),
            "Implement OAuth2 system"
        );
        
        // Test with quotes
        assert_eq!(
            clean_ai_preamble("Here is a concise branch description: \"Update build pipeline\""),
            "Update build pipeline"
        );
        
        // Test with colon patterns
        assert_eq!(
            clean_ai_preamble("A brief description for this branch: Migrate to GitLab CI"),
            "Migrate to GitLab CI"
        );
        
        // Test text without preamble (should remain unchanged)
        assert_eq!(
            clean_ai_preamble("Update Jenkins pipeline to use GitLab CI/CD"),
            "Update Jenkins pipeline to use GitLab CI/CD"
        );
        
        // Test text with extra whitespace
        assert_eq!(
            clean_ai_preamble("  Here is the branch description:   Fix critical bug   "),
            "Fix critical bug"
        );
    }

    #[test]
    fn test_ai_summarize_validation() {
        // Test that AI summarize validation works correctly
        
        // Should fail when ai_summarize is true but no input method is provided
        let result = std::panic::catch_unwind(|| {
            edit_description(
                None,          // target_branch
                None,          // description
                false,         // clipboard
                false,         // stdin
                None,          // issue
                true,          // ai_summarize
                120,           // ai_timeout
                false,         // commit
                false,         // push
                false,         // force
            )
        });
        
        // The function should fail before reaching repository operations
        assert!(result.is_err() || result.unwrap().is_err());
    }

    #[test]
    fn test_ai_summarize_content() {
        // This test requires Ollama to be running locally
        // It's more of an integration test
        let content = "Fix authentication bug\n\nUsers are experiencing login failures due to expired tokens not being properly refreshed. Need to implement automatic token refresh mechanism.";
        
        // This test will only pass if Ollama is running
        match ai_summarize_content(content, 30) {
            Ok(summary) => {
                assert!(!summary.is_empty());
                // Don't assume AI summary is shorter - it might expand short input
                println!("AI Summary: {}", summary);
            }
            Err(e) => {
                // Expected if Ollama isn't running
                println!("AI test skipped (Ollama not available): {}", e);
            }
        }
    }
}
