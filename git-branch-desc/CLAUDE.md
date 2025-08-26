# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Git Branch Description Manager (`git-branch-desc`) is a Rust CLI tool that manages branch descriptions by storing them in `BRANCHREADME.md` files within each branch. The tool supports operations on both local and remote branches without requiring checkout, using low-level Git operations.

## Key Commands

### Development
```bash
# Build the project
cargo build --release

# Install globally
cargo install --path .

# Format code
cargo fmt

# Run clippy lints
cargo clippy --fix
```

### Testing the tool
```bash
# Basic usage examples
./target/release/git-branch-desc edit "Test description"
./target/release/git-branch-desc e "Test description"  # Short alias

# Alternative input methods
./target/release/git-branch-desc edit --clipboard      # From clipboard
echo "Description" | ./target/release/git-branch-desc edit --stdin  # From stdin
./target/release/git-branch-desc edit --issue 123      # From GitLab issue number
./target/release/git-branch-desc edit --issue "https://gitlab.com/owner/repo/-/issues/456"  # From GitLab issue URL

# AI-powered summarization
./target/release/git-branch-desc edit --issue 123 --ai-summarize     # AI summary of issue
./target/release/git-branch-desc edit --issue "https://gitlab.com/owner/repo/-/issues/456" --ai-summarize
./target/release/git-branch-desc edit --clipboard --ai-summarize     # AI summary of clipboard
./target/release/git-branch-desc edit --stdin --ai-summarize         # AI summary of stdin

./target/release/git-branch-desc list
```

## Architecture

### Core Components
- **main.rs**: Single-file implementation containing all functionality
- **CLI Structure**: Uses `clap` for command-line parsing with two main commands:
  - `edit`/`e`: Edit branch descriptions (unified add/modify functionality) with multiple input methods
  - `list`/`ls`: List all branch descriptions

### Key Dependencies
- `git2`: Low-level Git operations for reading/writing to branches without checkout
- `clap`: Command-line argument parsing with derive macros
- `tabwriter`: Table formatting for list output
- `terminal_size`: Terminal width detection for text wrapping
- `anyhow`: Error handling
- `arboard`: Clipboard access for reading descriptions from system clipboard
- `regex`: URL parsing for GitLab issue links
- `serde_json`: Robust JSON parsing for GitLab issue data
- `reqwest`: HTTP client for Ollama API communication

### Git Integration
- **Current Branch Operations**: Traditional Git workflow (write file → stage → commit)
- **Remote Branch Operations**: Low-level Git object manipulation to commit directly to target branches
- **Safety Features**: Confirmation prompts when modifying non-current branches, branch validation
- **Storage**: Each branch maintains its own `BRANCHREADME.md` file in the branch root

### Key Functions
- `edit_description()`: Main logic for editing descriptions with intelligent add/update behavior
- `get_clipboard_content()`: Reads description text from system clipboard
- `get_stdin_content()`: Reads description text from stdin with terminal detection
- `get_interactive_input()`: Handles interactive description input with existing content display
- `get_issue_content()`: Fetches GitLab issue content using configured `glab.exe` with optional AI summarization
- `ai_summarize_content()`: Uses Ollama API to create concise branch descriptions from verbose issue content
- `parse_issue_reference()`: Parses GitLab issue numbers and URLs
- `parse_issue_json()`: Robustly extracts title and description from glab JSON output using serde_json
- `list_descriptions()`: Reads descriptions from all branches using Git objects
- `commit_to_branch()`: Low-level Git operations for committing to non-current branches
- `read_branch_description_from_git()`: Reads description files directly from Git trees
- `validate_branch_exists()`: Ensures target branch exists with helpful error messages

## Development Notes

- The tool supports both local and remote branch operations without checkout
- All commits include `[skip ci]` to prevent CI pipeline triggers
- Text wrapping adapts to terminal width (90% of available width)
- Remote branches are prioritized over local branches with same names in listings
- Safety warnings and confirmations prevent accidental branch modifications
- The unified `edit` command intelligently detects existing descriptions and shows them for editing
- Commit messages automatically reflect whether content was "Added" or "Updated"
- Multiple input methods supported: direct argument, interactive prompt, clipboard, stdin, and GitLab issues
- All input options (clipboard, stdin, issue) are mutually exclusive with each other and direct text input
- Stdin input includes terminal detection to prevent hanging when no input is available
- GitLab issue support uses configured `glab.exe` and accepts both issue numbers and full GitLab URLs
- Issue content is formatted as "Title" followed by the issue description (no markdown heading prefix)
- JSON parsing uses `serde_json` for robust handling of complex GitLab API responses
- AI summarization requires Ollama running locally with llama3.2:1b model (or compatible)
- AI integration uses reqwest for HTTP communication with Ollama's API
- AI prompts are optimized to create concise 2-3 sentence branch descriptions focused on main goals
- AI functionality works with --issue, --stdin, and --clipboard input methods
- AI summarization includes validation to ensure it's used with an appropriate input method
- AI functionality gracefully handles cases where Ollama is not available with helpful error messages