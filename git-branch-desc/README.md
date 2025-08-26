# Git Branch Description Manager

A Rust CLI tool that manages branch descriptions by storing them in `BRANCHREADME.md` files within each branch. Supports operations on both local and remote branches without requiring checkout, using low-level Git operations.

## Installation

```bash
# Build the project
cargo build --release

# Install globally
cargo install --path .
```

## Quick Start

```bash
# Add description to current branch
git-branch-desc edit "OAuth2 authentication implementation"

# Add description from GitLab issue
git-branch-desc edit --issue 123

# Add AI-summarized description from GitLab issue
git-branch-desc edit --issue 123 --ai-summarize

# Add description to any branch without checkout
git-branch-desc edit --branch feature/api "REST API implementation" --commit

# List all branch descriptions
git-branch-desc list
```

## Commands

### `edit` (alias: `e`)
Edit branch descriptions with multiple input methods:

```bash
# Direct text input
git-branch-desc edit "Description text"

# Interactive prompt (shows existing content for editing)
git-branch-desc edit

# From clipboard
git-branch-desc edit --clipboard

# From stdin
echo "Description" | git-branch-desc edit --stdin

# From GitLab issue
git-branch-desc edit --issue 123
git-branch-desc edit --issue "https://gitlab.com/owner/repo/-/issues/456"

# AI-summarized GitLab issue (requires Ollama)
git-branch-desc edit --issue 123 --ai-summarize

# AI-summarized content from clipboard
git-branch-desc edit --clipboard --ai-summarize

# AI-summarized content from stdin with custom timeout
echo "Long description..." | git-branch-desc edit --stdin --ai-summarize --ai-timeout 300
```

### `list` (alias: `ls`)
List all branch descriptions:

```bash
# Table view (default)
git-branch-desc list

# Detailed view with full descriptions
git-branch-desc list --detailed

# Include branches without descriptions
git-branch-desc list --all
```

## Input Methods

The `edit` command supports multiple input methods (mutually exclusive):

| Method | Flag | Description |
|--------|------|-------------|
| Direct | `[DESCRIPTION]` | Provide description as argument |
| Interactive | _(none)_ | Prompt for input (default when no text provided) |
| Clipboard | `--clipboard` | Read from system clipboard |
| Stdin | `--stdin` | Read from standard input |
| GitLab Issue | `--issue <REF>` | Fetch from GitLab issue |
| AI Summary | `--ai-summarize` | AI-generated summary (works with --issue, --stdin, --clipboard) |

## AI Summarization

The `--ai-summarize` flag works with `--issue`, `--stdin`, and `--clipboard` to create concise branch descriptions using AI:

### Setup
1. **Install Ollama**: Download from [https://ollama.ai](https://ollama.ai)
2. **Download a model**: `ollama run llama3.2:1b`
3. **Keep Ollama running** in the background

### Usage
```bash
# AI-summarize a GitLab issue
git-branch-desc edit --issue 123 --ai-summarize

# Works with issue URLs too
git-branch-desc edit --issue "https://gitlab.com/owner/repo/-/issues/456" --ai-summarize

# AI-summarize clipboard content
git-branch-desc edit --clipboard --ai-summarize

# AI-summarize stdin content with custom timeout
echo "Long verbose description..." | git-branch-desc edit --stdin --ai-summarize --ai-timeout 300

# For very large git diffs, use longer timeout
git diff HEAD~10 | git-branch-desc edit --stdin --ai-summarize --ai-timeout 600
```

### Benefits
- **Free**: Ollama and models are completely free
- **Fast**: llama3.2:1b optimized for speed and quality
- **Private**: Everything runs locally, no data sent externally
- **Smart**: Focuses on main goals rather than implementation details
- **Clean Output**: Automatically removes AI preamble text for clean, direct descriptions
- **Configurable**: Adjustable timeout for different content sizes and system performance

The AI creates concise 2-3 sentence descriptions that capture the essence of the issue without getting bogged down in technical details. The output is automatically cleaned to remove common AI preamble patterns like "Here is a concise branch description:" for clean, direct results.

## Options

| Flag | Description |
|------|-------------|
| `-b, --branch <NAME>` | Target branch (defaults to current branch) |
| `-c, --commit` | Automatically commit the BRANCHREADME.md file |
| `-p, --push` | Automatically commit and push changes |
| `-f, --force` | Skip confirmation prompts |
| `-d, --detailed` | Show full descriptions (list command) |
| `-a, --all` | Include branches without descriptions (list command) |
| `--ai-timeout <SECONDS>` | Timeout for AI processing in seconds (default: 120) |

## Key Features

- **No Checkout Required**: Modify any branch without switching to it using low-level Git operations
- **Safety First**: Confirmation prompts when modifying non-current branches with helpful error messages
- **Per-Branch Storage**: Each branch maintains its own `BRANCHREADME.md` file in the branch root
- **Fast Listing**: Read descriptions directly from Git objects without checkout for instant results
- **CI-Friendly**: All commits include `[skip ci]` flag to prevent unnecessary pipeline triggers
- **Smart Text Wrapping**: Adapts to terminal width (90% of available width) for optimal readability
- **Branch Validation**: Ensures target branches exist with helpful suggestions
- **Multiple Input Methods**: Supports direct input, clipboard, stdin, and GitLab issue integration
- **AI Integration**: Optional AI summarization for creating concise descriptions from verbose issues

## GitLab Integration

Requires `glab` CLI tool to be installed and configured:

```bash
# Install glab
# See: https://gitlab.com/gitlab-org/cli

# Configure glab
glab auth login

# Use with git-branch-desc
git-branch-desc edit --issue 123
```

## Examples

### Basic Usage
```bash
# Edit current branch description
git-branch-desc edit "Implement user authentication system"

# Edit remote branch without checkout
git-branch-desc edit --branch origin/feature/auth "Authentication implementation" --commit

# List all descriptions in table format
git-branch-desc list
```

### GitLab Issue Integration
```bash
# Use issue title and description
git-branch-desc edit --issue 123

# Get AI-generated summary (requires Ollama)
git-branch-desc edit --issue 123 --ai-summarize

# Works with URLs too
git-branch-desc edit --issue "https://gitlab.com/owner/repo/-/issues/456" --ai-summarize
```

### AI Summarization Examples
```bash
# Summarize any long content from clipboard
git-branch-desc edit --clipboard --ai-summarize

# Summarize content from a file with custom timeout
cat long_requirements.txt | git-branch-desc edit --stdin --ai-summarize --ai-timeout 180

# Summarize large git diff with extended timeout
git diff HEAD~5 | git-branch-desc edit --stdin --ai-summarize --ai-timeout 300

# Summarize issue content with default timeout
git-branch-desc edit --issue 123 --ai-summarize

# Quick summarization with shorter timeout
git-branch-desc edit --issue 123 --ai-summarize --ai-timeout 60
```

### Advanced Workflows
```bash
# From clipboard with auto-commit and AI summarization
git-branch-desc edit --clipboard --ai-summarize --commit

# From stdin with force and AI summarization (no prompts)
echo "Fix critical bug with detailed explanation..." | git-branch-desc edit --stdin --ai-summarize --force

# Edit and immediately push with AI summary and custom timeout
git-branch-desc edit --issue 123 --ai-summarize --ai-timeout 180 --push

# Handle very large content with extended timeout
git diff --cached | git-branch-desc edit --stdin --ai-summarize --ai-timeout 600 --commit
```

## Development

```bash
# Format code
cargo fmt

# Run lints
cargo clippy --fix

# Run tests
cargo test

# Test specific functionality
cargo run -- edit --help
./target/release/git-branch-desc list --detailed
```

## Architecture

- **Single File Implementation**: All functionality in `src/main.rs` for simplicity
- **Git2 Integration**: Uses `git2` crate for low-level Git operations
- **CLI with Clap**: Clean command-line interface using `clap` derive macros
- **Safety Features**: Validation, confirmation prompts, and helpful error messages
- **AI Integration**: Optional Ollama integration for intelligent summarization