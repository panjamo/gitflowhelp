use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use git_branch_desc::{GitBranchDescManager, InputSource};

#[derive(Parser)]
#[command(name = "git-branch-desc")]
#[command(about = "A tool to manage branch descriptions stored in BRANCHREADME.md files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum InputMethod {
    /// Read from command line argument or interactive prompt (default)
    #[value(name = "cli")]
    CommandLine,
    /// Read from system clipboard
    #[value(name = "clipboard")]
    Clipboard,
    /// Read from standard input
    #[value(name = "stdin")]
    Stdin,
    /// Read from GitLab issue
    #[value(name = "issue")]
    Issue,
}

impl Default for InputMethod {
    fn default() -> Self {
        Self::CommandLine
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Edit description for a branch (defaults to current branch)
    #[command(alias = "e")]
    Edit {
        /// Target branch name (defaults to current branch)
        #[arg(short, long)]
        branch: Option<String>,
        
        /// Description text (for cli input method only)
        description: Option<String>,
        
        /// Input source method
        #[arg(long, value_enum, default_value = "cli")]
        input: InputMethod,
        
        /// GitLab issue reference (number or URL) - required when input=issue
        #[arg(long, required_if_eq("input", "issue"))]
        issue_ref: Option<String>,
        
        /// Use AI to summarize content (works with all input methods except direct cli text)
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
            input,
            issue_ref,
            ai_summarize,
            ai_timeout,
            commit,
            push,
            force,
        } => {
            let input_source = match input {
                InputMethod::CommandLine => InputSource::CommandLine(description),
                InputMethod::Clipboard => InputSource::Clipboard,
                InputMethod::Stdin => InputSource::Stdin,
                InputMethod::Issue => InputSource::Issue(issue_ref.unwrap()),
            };
            
            manager.edit_description_v2(
                branch,
                input_source,
                ai_summarize,
                ai_timeout,
                commit,
                push,
                force,
            )
        }
        Commands::List { detailed, all } => manager.list_descriptions(detailed, all),
    }
}