use anyhow::Result;
use clap::{Parser, Subcommand};
use git_branch_desc::GitBranchDescManager;

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
    let manager = GitBranchDescManager::new(".")?;

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
        } => manager.edit_description(
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
        Commands::List { detailed, all } => manager.list_descriptions(detailed, all),
    }
}
