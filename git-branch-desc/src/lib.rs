use anyhow::{Context, Result};
use arboard::Clipboard;
use git2::Repository;
use regex::Regex;
use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::Path;
use std::process::Command;
use tabwriter::TabWriter;
use terminal_size::{Width, terminal_size};

#[derive(Debug, Clone)]
pub enum InputSource {
    /// Direct command line input (text argument or interactive prompt)
    CommandLine(Option<String>),
    /// Read from system clipboard
    Clipboard,
    /// Read from standard input
    Stdin,
    /// Read from GitLab issue reference
    Issue(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchDescription {
    pub branch: String,
    pub description: String,
}

pub struct GitBranchDescManager {
    repo: Repository,
}

impl GitBranchDescManager {
    pub fn new(repo_path: &str) -> Result<Self> {
        let repo = Repository::open(repo_path)
            .context("Failed to open repository. Make sure you're in a Git repository.")?;
        Ok(Self { repo })
    }

    pub fn edit_description_v2(
        &self,
        target_branch: Option<String>,
        input_source: InputSource,
        ai_summarize: bool,
        ai_timeout: u64,
        commit: bool,
        push: bool,
        force: bool,
    ) -> Result<()> {
        // Validate AI summarization usage
        if ai_summarize {
            match &input_source {
                InputSource::CommandLine(Some(_)) => {
                    anyhow::bail!("AI summarization cannot be used with direct text input. Use --input=clipboard, --input=stdin, or --input=issue instead.");
                }
                InputSource::CommandLine(None) | InputSource::Clipboard | InputSource::Stdin | InputSource::Issue(_) => {
                    // Valid combinations
                }
            }
        }

        // Determine the target branch
        let target_branch = target_branch
            .map(Ok)
            .unwrap_or_else(|| self.get_current_branch())?;

        // Validate that the branch exists
        self.validate_branch_exists(&target_branch)?;

        // Determine if we're working on the current branch
        let current_branch = self.get_current_branch()?;
        let is_current_branch = target_branch == current_branch;

        // Get existing description to determine if this is an add or modify operation
        let existing_description = if is_current_branch {
            self.read_current_branch_description()?
        } else {
            self.read_branch_description_from_git(&target_branch)?
                .unwrap_or_default()
        };

        let is_modify = !existing_description.trim().is_empty();

        // Safety check for non-current branches
        if !is_current_branch && !force {
            println!(
                "⚠️  You are about to modify branch '{}' (not current branch '{}')",
                target_branch, current_branch
            );
            print!("Continue? (y/N): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if !input.trim().to_lowercase().starts_with('y') {
                println!("Operation cancelled.");
                return Ok(());
            }
        }

        // Get the description content based on input source
        let description_content = match input_source {
            InputSource::CommandLine(Some(desc)) => desc,
            InputSource::CommandLine(None) => {
                self.get_interactive_input(&target_branch, &existing_description)?
            }
            InputSource::Clipboard => {
                let mut content = self.get_clipboard_content()?;
                if ai_summarize {
                    content = self.ai_summarize_content(&content, ai_timeout)?;
                }
                content
            }
            InputSource::Stdin => {
                let mut content = self.get_stdin_content()?;
                if ai_summarize {
                    content = self.ai_summarize_content(&content, ai_timeout)?;
                }
                content
            }
            InputSource::Issue(issue_ref) => {
                self.get_issue_content(&issue_ref, ai_summarize, ai_timeout)?
            }
        };

        // Write the description
        if is_current_branch {
            self.write_current_branch_description(&description_content)?;

            if commit {
                self.commit_current_branch_changes(&target_branch, is_modify, push)?;
            }
        } else {
            self.commit_to_branch(&target_branch, &description_content, is_modify, push)?;
        }

        let action = if is_modify { "Updated" } else { "Added" };
        println!("{action} description for branch '{target_branch}'");

        if !commit && is_current_branch {
            println!("💡 Use --commit or -c to automatically commit the change");
            if !push {
                println!("💡 Use --push or -p to automatically commit and push the change");
            }
        }

        Ok(())
    }

    pub fn edit_description(
        &self,
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
        // Convert old-style parameters to new InputSource enum for backward compatibility
        let input_source = if let Some(desc) = description {
            InputSource::CommandLine(Some(desc))
        } else if clipboard {
            InputSource::Clipboard
        } else if stdin {
            InputSource::Stdin
        } else if let Some(issue_ref) = issue {
            InputSource::Issue(issue_ref)
        } else {
            InputSource::CommandLine(None)
        };

        // Delegate to new implementation
        self.edit_description_v2(
            target_branch,
            input_source,
            ai_summarize,
            ai_timeout,
            commit,
            push,
            force,
        )
    }

    pub fn list_descriptions(&self, detailed: bool, all: bool) -> Result<()> {
        let mut descriptions = Vec::new();
        let local_branches = self.get_local_branch_list()?;
        let mut processed_branches = HashSet::new();

        // First, process remote branches
        let remotes = self.repo.remotes()?;
        for remote_name in remotes.iter() {
            if let Some(remote_name) = remote_name {
                let remote_branches = self.repo.branches(Some(git2::BranchType::Remote))?;
                for branch in remote_branches {
                    let (branch, _) = branch.context("Failed to get branch")?;
                    if let Some(name) = branch.name().context("Failed to get branch name")? {
                        if name.starts_with(&format!("{remote_name}/")) {
                            let branch_name = name
                                .strip_prefix(&format!("{remote_name}/"))
                                .unwrap_or(name);

                            if let Some(desc) = self.process_branch_description(name, true, all) {
                                descriptions.push(desc);
                                processed_branches.insert(branch_name.to_string());
                            }
                        }
                    }
                }
            }
        }

        // Then, process local branches (skip if already processed as remote)
        for branch_name in local_branches {
            if !processed_branches.contains(&branch_name) {
                if let Some(desc) = self.process_branch_description(&branch_name, false, all) {
                    descriptions.push(desc);
                }
            }
        }

        if descriptions.is_empty() {
            if all {
                println!("No branches found.");
            } else {
                println!("No branches with descriptions found.");
                println!(
                    "💡 Use --all or -a to show all branches including those without descriptions"
                );
            }
            return Ok(());
        }

        if detailed {
            self.print_detailed_descriptions(&descriptions)?;
        } else {
            self.print_table_descriptions(&descriptions)?;
        }

        Ok(())
    }

    pub fn get_current_branch(&self) -> Result<String> {
        let head = self.repo.head().context("Failed to get HEAD reference")?;
        let branch_name = head.shorthand().context("Failed to get branch name")?;
        Ok(branch_name.to_string())
    }

    pub fn read_current_branch_description(&self) -> Result<String> {
        match fs::read_to_string("BRANCHREADME.md") {
            Ok(content) => Ok(content),
            Err(_) => Ok(String::new()),
        }
    }

    pub fn write_current_branch_description(&self, description: &str) -> Result<()> {
        fs::write("BRANCHREADME.md", description)
            .context("Failed to write BRANCHREADME.md file")?;
        Ok(())
    }

    pub fn commit_current_branch_changes(
        &self,
        branch_name: &str,
        is_modify: bool,
        push: bool,
    ) -> Result<()> {
        // Stage the BRANCHREADME.md file
        let mut index = self
            .repo
            .index()
            .context("Failed to get repository index")?;
        index
            .add_path(Path::new("BRANCHREADME.md"))
            .context("Failed to stage BRANCHREADME.md")?;
        index.write().context("Failed to write index")?;

        // Create commit
        let tree_id = index.write_tree().context("Failed to write tree")?;
        let tree = self
            .repo
            .find_tree(tree_id)
            .context("Failed to find tree")?;
        let parent_commit = self.repo.head()?.peel_to_commit()?;
        let signature = self
            .repo
            .signature()
            .context("Failed to create signature")?;

        let action = if is_modify { "Update" } else { "Add" };
        let commit_message = format!("{action} branch description [skip ci]");

        self.repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                &commit_message,
                &tree,
                &[&parent_commit],
            )
            .context("Failed to create commit")?;

        println!("✅ Committed changes to branch '{branch_name}'");

        if push {
            self.push_current_branch(branch_name)?;
        }

        Ok(())
    }

    pub fn commit_to_branch(
        &self,
        branch_name: &str,
        description: &str,
        is_modify: bool,
        push: bool,
    ) -> Result<()> {
        // Get the target branch reference
        let branch_ref_name = format!("refs/heads/{branch_name}");
        let branch_ref = self
            .repo
            .find_reference(&branch_ref_name)
            .with_context(|| format!("Branch '{branch_name}' not found"))?;

        let branch_commit = branch_ref.peel_to_commit()?;
        let branch_tree = branch_commit.tree()?;

        // Create a new tree with the BRANCHREADME.md file
        let mut tree_builder = self.repo.treebuilder(Some(&branch_tree))?;

        // Create blob for the description content
        let blob_id = self.repo.blob(description.as_bytes())?;

        // Insert the file into the tree
        tree_builder.insert("BRANCHREADME.md", blob_id, git2::FileMode::Blob.into())?;
        let new_tree_id = tree_builder.write()?;
        let new_tree = self.repo.find_tree(new_tree_id)?;

        // Create commit
        let signature = self
            .repo
            .signature()
            .context("Failed to create signature")?;
        let action = if is_modify { "Update" } else { "Add" };
        let commit_message = format!("{action} branch description [skip ci]");

        let new_commit_id = self.repo.commit(
            None, // Don't update any reference yet
            &signature,
            &signature,
            &commit_message,
            &new_tree,
            &[&branch_commit],
        )?;

        // Update the branch reference to point to the new commit
        let mut branch_ref = self.repo.find_reference(&branch_ref_name)?;
        branch_ref.set_target(new_commit_id, &commit_message)?;

        println!("✅ Committed changes to branch '{branch_name}'");

        if push {
            self.push_branch(branch_name)?;
        }

        Ok(())
    }

    pub fn validate_branch_exists(&self, branch_name: &str) -> Result<()> {
        // Check local branches first
        let local_ref = format!("refs/heads/{branch_name}");
        if self.repo.find_reference(&local_ref).is_ok() {
            return Ok(());
        }

        // Check remote branches
        let remote_ref = format!("refs/remotes/origin/{branch_name}");
        if self.repo.find_reference(&remote_ref).is_ok() {
            return Ok(());
        }

        anyhow::bail!(
            "Branch '{branch_name}' not found. Available branches:\n{}",
            self.get_available_branches_list()?
        );
    }

    pub fn get_local_branch_list(&self) -> Result<Vec<String>> {
        let mut branches = Vec::new();
        let branch_iter = self.repo.branches(Some(git2::BranchType::Local))?;

        for branch in branch_iter {
            let (branch, _) = branch.context("Failed to get branch")?;
            if let Some(name) = branch.name().context("Failed to get branch name")? {
                branches.push(name.to_string());
            }
        }

        Ok(branches)
    }

    pub fn get_clipboard_content(&self) -> Result<String> {
        let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
        let content = clipboard
            .get_text()
            .context("Failed to read from clipboard")?;
        Ok(content.trim().to_string())
    }

    pub fn get_stdin_content(&self) -> Result<String> {
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

    pub fn get_interactive_input(
        &self,
        target_branch: &str,
        existing_description: &str,
    ) -> Result<String> {
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

    pub fn get_issue_content(
        &self,
        issue_ref: &str,
        ai_summarize: bool,
        ai_timeout: u64,
    ) -> Result<String> {
        // Parse the issue reference - could be a number or a URL
        let issue_number = parse_issue_reference(issue_ref)?;

        // Use glab to get issue information
        let output = Command::new("glab")
            .args(["issue", "view", &issue_number, "--output", "json"])
            .output()
            .context(
                "Failed to execute glab command. Make sure glab is installed and configured.",
            )?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("glab command failed: {error_msg}");
        }

        let json_output = String::from_utf8_lossy(&output.stdout);
        let mut content = parse_issue_json(&json_output)?;

        if ai_summarize {
            content = self.ai_summarize_content(&content, ai_timeout)?;
        }

        Ok(content)
    }

    pub fn ai_summarize_content(&self, content: &str, timeout_seconds: u64) -> Result<String> {
        // Validate content length - git diffs can be very large
        const MAX_CONTENT_LENGTH: usize = 8000; // Reasonable limit for AI processing
        let content_to_process = if content.len() > MAX_CONTENT_LENGTH {
            println!(
                "Content is large ({} chars), truncating to {} chars for AI processing...",
                content.len(),
                MAX_CONTENT_LENGTH
            );
            &content[..MAX_CONTENT_LENGTH]
        } else {
            content
        };

        // Determine if this looks like a git diff
        let is_git_diff = content_to_process.contains("diff --git")
            || content_to_process.contains("@@")
            || content_to_process
                .lines()
                .any(|line| line.starts_with("+++") || line.starts_with("---"));

        let system_prompt = if is_git_diff {
            "You are an expert software engineer. Create a concise 2-3 sentence branch description from this git diff. Focus on the main changes and their purpose. Do not include implementation details or file names."
        } else {
            "You are an expert software engineer. Create a concise 2-3 sentence branch description from this content. Focus on the main goals and requirements. Keep it professional and actionable."
        };

        let user_prompt = format!("Content to summarize:\n\n{}", content_to_process);

        let client = Client::new();
        let request_body = serde_json::json!({
            "model": "llama3.2:1b",
            "stream": false,
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ]
        });

        println!(
            "🤖 Generating AI summary (timeout: {}s)...",
            timeout_seconds
        );

        let response = client
            .post("http://localhost:11434/api/chat")
            .timeout(std::time::Duration::from_secs(timeout_seconds))
            .json(&request_body)
            .send()
            .context(
                "Failed to connect to Ollama. Make sure Ollama is running locally on port 11434.",
            )?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Ollama API request failed with status: {}",
                response.status()
            );
        }

        let response_json: Value = response
            .json()
            .context("Failed to parse Ollama response as JSON")?;

        let summary = response_json["message"]["content"]
            .as_str()
            .context("Failed to extract content from Ollama response")?;

        let cleaned_summary = clean_ai_preamble(summary);

        if cleaned_summary.trim().is_empty() {
            anyhow::bail!(
                "AI generated empty summary. Please try again or provide description manually."
            );
        }

        Ok(cleaned_summary)
    }

    fn process_branch_description(
        &self,
        full_branch_name: &str,
        _is_remote: bool,
        include_all: bool,
    ) -> Option<BranchDescription> {
        let branch_name = full_branch_name;

        if let Ok(Some(description)) = self.read_branch_description_from_git(full_branch_name) {
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

    fn get_available_branches_list(&self) -> Result<String> {
        let mut branches = Vec::new();

        // Add local branches
        for branch in self.get_local_branch_list()? {
            branches.push(format!("  {branch}"));
        }

        // Add remote branches
        let remote_branches = self.repo.branches(Some(git2::BranchType::Remote))?;
        for branch in remote_branches {
            let (branch, _) = branch.context("Failed to get branch")?;
            if let Some(name) = branch.name().context("Failed to get branch name")? {
                if let Some(short_name) = name.strip_prefix("origin/") {
                    branches.push(format!("  {short_name} (remote)"));
                }
            }
        }

        Ok(branches.join("\n"))
    }

    fn push_current_branch(&self, branch_name: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["push", "origin", branch_name])
            .output()
            .context("Failed to execute git push command")?;

        if output.status.success() {
            println!("✅ Pushed changes to remote branch '{branch_name}'");
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to push branch: {error_msg}");
        }

        Ok(())
    }

    fn push_branch(&self, branch_name: &str) -> Result<()> {
        let output = Command::new("git")
            .args([
                "push",
                "origin",
                &format!("refs/heads/{branch_name}:refs/heads/{branch_name}"),
            ])
            .output()
            .context("Failed to execute git push command")?;

        if output.status.success() {
            println!("✅ Pushed changes to remote branch '{branch_name}'");
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to push branch: {error_msg}");
        }

        Ok(())
    }

    fn print_detailed_descriptions(&self, descriptions: &[BranchDescription]) -> Result<()> {
        for desc in descriptions {
            println!("Branch: {}", desc.branch);
            println!("Description:");
            let wrapped = wrap_text(&desc.description, get_terminal_width());
            for line in wrapped.lines() {
                println!("  {line}");
            }
            println!();
        }
        Ok(())
    }

    fn print_table_descriptions(&self, descriptions: &[BranchDescription]) -> Result<()> {
        let mut tw = TabWriter::new(io::stdout());
        writeln!(tw, "BRANCH\tDESCRIPTION")?;
        writeln!(tw, "------\t-----------")?;

        for desc in descriptions {
            let wrapped_desc = wrap_text(
                &desc.description,
                get_terminal_width() - desc.branch.len() - 10,
            );
            let first_line = wrapped_desc.lines().next().unwrap_or("");
            writeln!(tw, "{}\t{}", desc.branch, first_line)?;
        }

        tw.flush()?;
        Ok(())
    }

    pub fn read_branch_description_from_git(&self, branch_name: &str) -> Result<Option<String>> {
        // Try to find the branch reference
        let branch_ref = if let Ok(branch_ref) = self
            .repo
            .find_reference(&format!("refs/heads/{branch_name}"))
        {
            branch_ref
        } else if let Ok(branch_ref) = self
            .repo
            .find_reference(&format!("refs/remotes/{branch_name}"))
        {
            branch_ref
        } else {
            return Ok(None);
        };

        let commit = branch_ref.peel_to_commit()?;
        let tree = commit.tree()?;

        match tree.get_name("BRANCHREADME.md") {
            Some(entry) => {
                let blob = self.repo.find_blob(entry.id())?;
                let content = String::from_utf8_lossy(blob.content());
                Ok(Some(content.to_string()))
            }
            None => Ok(None),
        }
    }
}

// Utility functions
pub fn get_terminal_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        (w as usize * 90) / 100 // Use 90% of terminal width
    } else {
        80 // Default width
    }
}

pub fn wrap_text(text: &str, max_width: usize) -> String {
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

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines.join("\n")
}

pub fn clean_ai_preamble(text: &str) -> String {
    let text = text.trim();

    // Filter out lines with thinking tags and empty lines
    let filtered_lines: Vec<&str> = text
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.contains("<think>") && !trimmed.contains("</think>")
        })
        .collect();

    let mut result = filtered_lines.join("\n");

    // Remove common AI preambles
    let preambles_to_remove = [
        "Here's a concise branch description:",
        "Here is a concise branch description:",
        "Here's a branch description:",
        "Here is a branch description:",
        "Here's the branch description:",
        "Here is the branch description:",
        "A concise branch description:",
        "Branch description:",
        "Description:",
        "Summary:",
        "Here's a summary:",
        "Here is a summary:",
        "Based on the content:",
        "Based on this content:",
        "Description for this branch:",
        "A brief description:",
        "Brief description:",
    ];

    for preamble in &preambles_to_remove {
        if result
            .trim_start()
            .to_lowercase()
            .starts_with(&preamble.to_lowercase())
        {
            result = result
                .trim_start()
                .strip_prefix(preamble)
                .unwrap_or(&result)
                .trim_start()
                .to_string();
        }
    }

    // Remove surrounding quotes if present
    let result = result.trim();
    if (result.starts_with('"') && result.ends_with('"'))
        || (result.starts_with('\'') && result.ends_with('\''))
    {
        result[1..result.len() - 1].trim().to_string()
    } else {
        result.to_string()
    }
}

pub fn parse_issue_reference(issue_ref: &str) -> Result<String> {
    // Check if it's a GitLab issue URL (supports nested groups)
    let url_regex =
        Regex::new(r"https?://[^/]+/.+/-/issues/(\d+)").context("Failed to compile URL regex")?;

    if let Some(captures) = url_regex.captures(issue_ref) {
        if let Some(number) = captures.get(1) {
            return Ok(number.as_str().to_string());
        }
    }

    // Check if it's just a number
    if !issue_ref.is_empty() && issue_ref.chars().all(|c| c.is_ascii_digit()) {
        return Ok(issue_ref.to_string());
    }

    anyhow::bail!(
        "Invalid issue reference: '{}'. Expected issue number or GitLab issue URL.",
        issue_ref
    );
}

pub fn parse_issue_json(json: &str) -> Result<String> {
    // Parse JSON using serde_json for robust parsing
    let parsed: Value =
        serde_json::from_str(json).context("Failed to parse JSON output from glab")?;

    // Extract title
    let title = parsed["title"]
        .as_str()
        .context("Could not extract issue title from glab output")?;

    // Extract description (handle null and empty cases)
    let description = parsed["description"].as_str().unwrap_or("").trim();

    // Format the result
    let result = if description.is_empty() {
        title.to_string()
    } else {
        format!("{}\n\n{}", title, description)
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_repo() -> (TempDir, Repository) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo = Repository::init(temp_dir.path()).expect("Failed to init repository");

        // Configure user for commits
        let mut config = repo.config().expect("Failed to get config");
        config
            .set_str("user.name", "Test User")
            .expect("Failed to set user.name");
        config
            .set_str("user.email", "test@example.com")
            .expect("Failed to set user.email");

        // Create initial commit
        fs::write(temp_dir.path().join("README.md"), "# Test Repository")
            .expect("Failed to create README");

        {
            let mut index = repo.index().expect("Failed to get index");
            index
                .add_path(Path::new("README.md"))
                .expect("Failed to add README");
            index.write().expect("Failed to write index");

            let tree_id = index.write_tree().expect("Failed to write tree");
            let tree = repo.find_tree(tree_id).expect("Failed to find tree");
            let signature = repo.signature().expect("Failed to create signature");

            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                "Initial commit",
                &tree,
                &[],
            )
            .expect("Failed to create initial commit");
        }

        (temp_dir, repo)
    }

    #[test]
    fn test_get_terminal_width() {
        let width = get_terminal_width();
        assert!(width > 0);
        assert!(width <= 1000); // Reasonable upper bound
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a long line that should be wrapped at a certain width";
        let wrapped = wrap_text(text, 20);

        for line in wrapped.lines() {
            assert!(line.len() <= 20);
        }

        // Test that words are preserved
        assert!(wrapped.contains("This"));
        assert!(wrapped.contains("wrapped"));
    }

    #[test]
    fn test_clean_ai_preamble() {
        let input = "Here's a concise branch description: Implement user authentication feature";
        let result = clean_ai_preamble(input);
        assert_eq!(result, "Implement user authentication feature");

        let input_with_thinking = "
        <think>This is thinking</think>
        Here's a branch description:
        
        Implement user authentication feature
        ";
        let result = clean_ai_preamble(input_with_thinking);
        // The function should strip "Here's a branch description:" and keep the content
        assert_eq!(result, "Implement user authentication feature");
    }

    #[test]
    fn test_parse_issue_reference() {
        // Test issue number
        assert_eq!(parse_issue_reference("123").unwrap(), "123");

        // Test GitLab URL
        let url = "https://gitlab.com/owner/repo/-/issues/456";
        assert_eq!(parse_issue_reference(url).unwrap(), "456");

        // Test invalid input
        assert!(parse_issue_reference("invalid").is_err());
    }

    #[test]
    fn test_parse_issue_json() {
        let json = r#"{"title": "Fix login bug", "description": "The login form is not working properly"}"#;
        let result = parse_issue_json(json).unwrap();
        assert_eq!(
            result,
            "Fix login bug\n\nThe login form is not working properly"
        );

        // Test with empty description
        let json_no_desc = r#"{"title": "Fix login bug", "description": ""}"#;
        let result = parse_issue_json(json_no_desc).unwrap();
        assert_eq!(result, "Fix login bug");

        // Test with null description
        let json_null_desc = r#"{"title": "Fix login bug", "description": null}"#;
        let result = parse_issue_json(json_null_desc).unwrap();
        assert_eq!(result, "Fix login bug");
    }

    #[test]
    fn test_git_branch_desc_manager_new() {
        let (temp_dir, _repo) = create_test_repo();
        let manager = GitBranchDescManager::new(temp_dir.path().to_str().unwrap());
        assert!(manager.is_ok());

        // Test with invalid path
        let invalid_manager = GitBranchDescManager::new("/invalid/path");
        assert!(invalid_manager.is_err());
    }

    #[test]
    fn test_read_write_current_branch_description() {
        let (temp_dir, _repo) = create_test_repo();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let manager = GitBranchDescManager::new(".").unwrap();

        // Test reading non-existent file
        let description = manager.read_current_branch_description().unwrap();
        assert!(description.is_empty());

        // Test writing and reading
        let test_desc = "Test description";
        manager.write_current_branch_description(test_desc).unwrap();
        let read_desc = manager.read_current_branch_description().unwrap();
        assert_eq!(read_desc, test_desc);

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_get_local_branch_list() {
        let (_temp_dir, _repo) = create_test_repo();
        let manager = GitBranchDescManager::new(_temp_dir.path().to_str().unwrap()).unwrap();

        let branches = manager.get_local_branch_list().unwrap();
        assert!(!branches.is_empty());
        // The default branch name depends on Git configuration, could be main or master
        assert!(!branches.is_empty());
    }

    #[test]
    fn test_validate_branch_exists() {
        let (_temp_dir, _repo) = create_test_repo();
        let manager = GitBranchDescManager::new(_temp_dir.path().to_str().unwrap()).unwrap();

        // Test existing branch
        let current_branch = manager.get_current_branch().unwrap();
        assert!(manager.validate_branch_exists(&current_branch).is_ok());

        // Test non-existent branch
        assert!(
            manager
                .validate_branch_exists("non-existent-branch")
                .is_err()
        );
    }

    #[test]
    fn test_branch_description_struct() {
        let desc = BranchDescription {
            branch: "feature/test".to_string(),
            description: "Test description".to_string(),
        };

        assert_eq!(desc.branch, "feature/test");
        assert_eq!(desc.description, "Test description");

        // Test Clone and PartialEq
        let desc2 = desc.clone();
        assert_eq!(desc, desc2);
    }
}
