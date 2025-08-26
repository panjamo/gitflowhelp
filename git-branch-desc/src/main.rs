use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use git2::Repository;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
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
    /// Add or modify description for a branch (defaults to current branch)
    #[command(alias = "set")]
    Add {
        /// Target branch name (defaults to current branch)
        #[arg(short, long)]
        branch: Option<String>,
        /// Description text (if not provided, will prompt for input)
        description: Option<String>,
        /// Automatically commit the BRANCHREADME.md file after adding/modifying
        #[arg(short, long)]
        commit: bool,
        /// Automatically commit and push the BRANCHREADME.md file after adding/modifying
        #[arg(short, long)]
        push: bool,
        /// Skip confirmation prompts (force operation)
        #[arg(short, long)]
        force: bool,
    },
    /// Modify description for a branch (defaults to current branch)
    #[command(alias = "edit")]
    Modify {
        /// Target branch name (defaults to current branch)
        #[arg(short, long)]
        branch: Option<String>,
        /// New description text (if not provided, will prompt for input)
        description: Option<String>,
        /// Automatically commit the BRANCHREADME.md file after adding/modifying
        #[arg(short, long)]
        commit: bool,
        /// Automatically commit and push the BRANCHREADME.md file after adding/modifying
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
        Commands::Add {
            branch,
            description,
            commit,
            push,
            force,
        } => add_or_modify_description(branch, description, false, commit, push, force),
        Commands::Modify {
            branch,
            description,
            commit,
            push,
            force,
        } => add_or_modify_description(branch, description, true, commit, push, force),
        Commands::List { detailed, all } => list_descriptions(detailed, all),
    }
}

fn add_or_modify_description(
    target_branch: Option<String>,
    description: Option<String>,
    is_modify: bool,
    commit: bool,
    push: bool,
    force: bool,
) -> Result<()> {
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
            println!("🚀 Force mode: Modifying branch '{target_branch}' without confirmation");
        } else {
            println!(
                "⚠️  WARNING: You are about to modify branch '{target_branch}' while on '{current_branch}'"
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

    let description = if let Some(desc) = description {
        desc
    } else {
        if is_modify {
            let existing = if is_current_branch {
                read_current_branch_description().unwrap_or_default()
            } else {
                read_branch_description_from_git(&repo, &target_branch).unwrap_or_default()
            };
            if !existing.is_empty() {
                println!("Current description for branch '{target_branch}':");
                println!("{existing}");
                println!();
            }
        }

        print!("Enter description for branch '{target_branch}': ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_string()
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

    let action = if is_modify { "Modified" } else { "Added" };
    println!("✅ {action} description for branch '{target_branch}'");

    // Handle commit and push options
    if push || commit {
        if is_current_branch {
            // Use traditional approach for current branch
            commit_current_branch_changes(&repo, &target_branch, is_modify, push)?;
        } else {
            // Use low-level Git operations for non-current branch
            commit_to_branch(&repo, &target_branch, &description, is_modify, push)?;
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
        (f32::from(w) * 0.9) as usize
    } else {
        80 // Default fallback width
    }
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();

    for line in text.lines() {
        if line.len() <= width {
            lines.push(line.to_string());
        } else {
            let mut current_line = String::new();
            let words: Vec<&str> = line.split_whitespace().collect();

            for word in words {
                if current_line.is_empty() {
                    current_line = word.to_string();
                } else if current_line.len() + 1 + word.len() <= width {
                    current_line.push(' ');
                    current_line.push_str(word);
                } else {
                    lines.push(current_line);
                    current_line = word.to_string();
                }
            }

            if !current_line.is_empty() {
                lines.push(current_line);
            }
        }
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn list_descriptions(detailed: bool, show_all: bool) -> Result<()> {
    let repo = Repository::open(".")
        .context("Failed to open repository. Make sure you're in a Git repository.")?;

    // Get all local and remote branches without checking them out
    let local_branches = repo.branches(Some(git2::BranchType::Local))?;
    let remote_branches = repo.branches(Some(git2::BranchType::Remote))?;

    let mut branch_descriptions = Vec::new();
    let mut found_any = false;
    let mut local_branch_names = HashSet::new();

    // Process local branches first
    for branch_result in local_branches {
        let (branch, _) = branch_result?;

        if let Some(branch_name) = branch.name()? {
            local_branch_names.insert(branch_name.to_string());
            if let Ok(description) = read_branch_description_from_git(&repo, branch_name) {
                if !description.is_empty() {
                    found_any = true;
                    process_branch_description(
                        branch_name,
                        &description,
                        detailed,
                        &mut branch_descriptions,
                    )?;
                } else if show_all {
                    found_any = true;
                    process_branch_description(
                        branch_name,
                        "-",
                        detailed,
                        &mut branch_descriptions,
                    )?;
                }
            } else if show_all {
                found_any = true;
                process_branch_description(branch_name, "-", detailed, &mut branch_descriptions)?;
            }
        }
    }

    // Process remote branches, but skip if local branch with same name exists
    for branch_result in remote_branches {
        let (branch, _) = branch_result?;

        if let Some(branch_name) = branch.name()? {
            // Skip remote HEAD references
            if branch_name.ends_with("/HEAD") {
                continue;
            }

            // Extract the branch name without remote prefix to check for local conflicts
            let local_equivalent = if let Some(slash_pos) = branch_name.find('/') {
                &branch_name[slash_pos + 1..]
            } else {
                branch_name
            };

            // Skip this remote branch if a local branch with the same name exists
            if local_branch_names.contains(local_equivalent) {
                continue;
            }

            if let Ok(description) = read_branch_description_from_git(&repo, branch_name) {
                if !description.is_empty() {
                    found_any = true;
                    process_branch_description(
                        branch_name,
                        &description,
                        detailed,
                        &mut branch_descriptions,
                    )?;
                } else if show_all {
                    found_any = true;
                    process_branch_description(
                        branch_name,
                        "-",
                        detailed,
                        &mut branch_descriptions,
                    )?;
                }
            } else if show_all {
                found_any = true;
                process_branch_description(branch_name, "-", detailed, &mut branch_descriptions)?;
            }
        }
    }

    if !found_any {
        if show_all {
            println!("No branches found.");
        } else {
            println!("No branch descriptions found.");
        }
    } else if !detailed {
        // Create tabwriter with proper padding and alignment
        let mut tw = TabWriter::new(vec![]).padding(2);

        // Write header
        writeln!(tw, "Branch\tDescription").unwrap();
        writeln!(tw, "------\t-----------").unwrap();

        // Write data rows
        for desc in branch_descriptions {
            writeln!(tw, "{}\t{}", desc.branch, desc.description).unwrap();
        }

        // Flush and print
        let written = tw.into_inner().unwrap();
        print!("{}", String::from_utf8(written).unwrap());
        if show_all {
            println!("\nUse --detailed (-d) flag to see full descriptions.");
        } else {
            println!(
                "\nUse --detailed (-d) flag to see full descriptions, or --all (-a) to show all branches."
            );
        }
    }

    Ok(())
}

fn process_branch_description(
    branch_name: &str,
    description: &str,
    detailed: bool,
    branch_descriptions: &mut Vec<BranchDescription>,
) -> Result<()> {
    if detailed {
        // For detailed view, show full descriptions wrapped at 90% terminal width
        let terminal_width = get_terminal_width();
        let box_width = terminal_width.saturating_sub(4).max(20); // Leave space for box chars
        let header_width = box_width.saturating_sub(branch_name.len() + 3).max(0);

        println!("\n┌─ {} {}", branch_name, "─".repeat(header_width));

        let wrapped_lines = wrap_text(description, box_width.saturating_sub(2));
        for line in wrapped_lines {
            println!("│ {line}");
        }

        println!("└{}", "─".repeat(box_width));
    } else {
        // Truncate long descriptions for table display
        let truncated_description = if description.len() > 80 {
            format!("{}...", &description[..77])
        } else {
            description.to_string()
        };

        branch_descriptions.push(BranchDescription {
            branch: branch_name.to_string(),
            description: truncated_description,
        });
    }

    Ok(())
}

fn get_current_branch(repo: &Repository) -> Result<String> {
    let head = repo.head().context("Failed to get HEAD reference")?;

    if let Some(branch_name) = head.shorthand() {
        Ok(branch_name.to_string())
    } else {
        anyhow::bail!("Could not determine current branch name")
    }
}

fn read_current_branch_description() -> Result<String> {
    let file_path = "BRANCHREADME.md";

    if !Path::new(file_path).exists() {
        return Ok(String::new());
    }

    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read description file: {file_path}"))?;

    Ok(content.trim().to_string())
}

fn write_current_branch_description(description: &str) -> Result<()> {
    let file_path = "BRANCHREADME.md";

    fs::write(file_path, description)
        .with_context(|| format!("Failed to write description file: {file_path}"))?;

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
    let branch_ref_name = format!("refs/heads/{branch_name}");

    if repo.find_reference(&branch_ref_name).is_ok() {
        Ok(())
    } else {
        // Check if it's a remote branch pattern
        if branch_name.contains('/') {
            anyhow::bail!(
                "Branch '{}' not found. Note: Use --branch for local branches only. \
                To create a description for a remote branch, create a local tracking branch first:\n\
                git checkout -b {} origin/{}",
                branch_name,
                branch_name.split('/').next_back().unwrap_or(branch_name),
                branch_name
            );
        }
        anyhow::bail!(
            "Local branch '{}' not found. Available local branches:\n{}",
            branch_name,
            get_local_branch_list(repo)?
        );
    }
}

fn get_local_branch_list(repo: &Repository) -> Result<String> {
    let local_branches = repo.branches(Some(git2::BranchType::Local))?;
    let mut branch_names = Vec::new();

    for branch_result in local_branches {
        let (branch, _) = branch_result?;
        if let Some(branch_name) = branch.name()? {
            branch_names.push(format!("  - {branch_name}"));
        }
    }

    if branch_names.is_empty() {
        Ok("  (no local branches found)".to_string())
    } else {
        Ok(branch_names.join("\n"))
    }
}

fn read_branch_description_from_git(repo: &Repository, branch_name: &str) -> Result<String> {
    // Get the branch reference - handle both local and remote branches
    let branch_ref_name = if branch_name.starts_with("refs/") {
        // Already a full reference
        branch_name.to_string()
    } else if branch_name.starts_with("origin/")
        || branch_name.starts_with("upstream/")
        || (branch_name.contains('/')
            && repo
                .find_reference(&format!("refs/remotes/{branch_name}"))
                .is_ok())
    {
        // This is a remote branch like "origin/main"
        format!("refs/remotes/{branch_name}")
    } else {
        // Local branch (including those with slashes like "feature/auth")
        format!("refs/heads/{branch_name}")
    };

    let branch_ref = repo
        .find_reference(&branch_ref_name)
        .with_context(|| format!("Failed to find branch: {branch_name}"))?;

    // Get the commit that the branch points to
    let commit_oid = branch_ref.target().context("Failed to get target commit")?;

    let commit = repo
        .find_commit(commit_oid)
        .context("Failed to find commit")?;

    // Get the tree from the commit
    let tree = commit.tree().context("Failed to get tree from commit")?;

    // Try to find BRANCHREADME.md in the tree
    match tree.get_name("BRANCHREADME.md") {
        Some(entry) => {
            // Get the blob
            let blob = repo
                .find_blob(entry.id())
                .context("Failed to find blob for BRANCHREADME.md")?;

            // Convert blob content to string
            let content = std::str::from_utf8(blob.content())
                .context("Failed to convert blob content to UTF-8")?;

            Ok(content.trim().to_string())
        }
        None => {
            // File doesn't exist in this branch
            Ok(String::new())
        }
    }
}
