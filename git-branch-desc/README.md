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
git-branch-desc edit --input=issue --issue-ref=123

# Add AI-summarized description from GitLab issue
git-branch-desc edit --input=issue --issue-ref=123 --ai-summarize

# Add description to any branch without checkout
git-branch-desc edit --branch feature/api "REST API implementation" --commit

# List all branch descriptions
git-branch-desc list
```

## Commands

### `edit` (alias: `e`)
Edit branch descriptions with multiple input methods:

```bash
# Direct text input (default --input=cli)
git-branch-desc edit "Description text"

# Interactive prompt (shows existing content for editing)
git-branch-desc edit

# From clipboard
git-branch-desc edit --input=clipboard

# From stdin
echo "Description" | git-branch-desc edit --input=stdin

# From GitLab issue
git-branch-desc edit --input=issue --issue-ref=123
git-branch-desc edit --input=issue --issue-ref="https://gitlab.com/owner/repo/-/issues/456"

# AI-summarized GitLab issue (requires Ollama)
git-branch-desc edit --input=issue --issue-ref=123 --ai-summarize

# AI-summarized content from clipboard
git-branch-desc edit --input=clipboard --ai-summarize

# AI-summarized content from stdin with custom timeout
echo "Long description..." | git-branch-desc edit --input=stdin --ai-summarize --ai-timeout 300
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

The `edit` command supports multiple input methods via the `--input` flag:

| Method | Flag | Description |
|--------|------|-------------|
| CLI | `--input=cli` | Direct text argument or interactive prompt (default) |
| Clipboard | `--input=clipboard` | Read from system clipboard |
| Stdin | `--input=stdin` | Read from standard input |
| GitLab Issue | `--input=issue --issue-ref=<REF>` | Fetch from GitLab issue |
| AI Summary | `--ai-summarize` | AI-generated summary (works with all input methods except direct text) |

## AI Summarization

The `--ai-summarize` flag works with `--input=issue`, `--input=stdin`, and `--input=clipboard` to create concise branch descriptions using AI:

### Setup
1. **Install Ollama**: Download from [https://ollama.ai](https://ollama.ai)
2. **Download a model**: `ollama run llama3.2:1b`
3. **Keep Ollama running** in the background

### Usage
```bash
# AI-summarize a GitLab issue
git-branch-desc edit --input=issue --issue-ref=123 --ai-summarize

# Works with issue URLs too
git-branch-desc edit --input=issue --issue-ref="https://gitlab.com/owner/repo/-/issues/456" --ai-summarize

# AI-summarize clipboard content
git-branch-desc edit --input=clipboard --ai-summarize

# AI-summarize stdin content with custom timeout
echo "Long verbose description..." | git-branch-desc edit --input=stdin --ai-summarize --ai-timeout 300

# For very large git diffs, use longer timeout
git diff HEAD~10 | git-branch-desc edit --input=stdin --ai-summarize --ai-timeout 600
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
| `--input <METHOD>` | Input source: cli, clipboard, stdin, issue (default: cli) |
| `--issue-ref <REF>` | GitLab issue reference (required when --input=issue) |
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
git-branch-desc edit --input=issue --issue-ref=123

# Get AI-generated summary (requires Ollama)
git-branch-desc edit --input=issue --issue-ref=123 --ai-summarize

# Works with URLs too
git-branch-desc edit --input=issue --issue-ref="https://gitlab.com/owner/repo/-/issues/456" --ai-summarize
```

### AI Summarization Examples
```bash
# Summarize any long content from clipboard
git-branch-desc edit --input=clipboard --ai-summarize

# Summarize content from a file with custom timeout
cat long_requirements.txt | git-branch-desc edit --input=stdin --ai-summarize --ai-timeout 180

# Summarize large git diff with extended timeout
git diff HEAD~5 | git-branch-desc edit --input=stdin --ai-summarize --ai-timeout 300

# Summarize issue content with default timeout
git-branch-desc edit --input=issue --issue-ref=123 --ai-summarize

# Quick summarization with shorter timeout
git-branch-desc edit --input=issue --issue-ref=123 --ai-summarize --ai-timeout 60
```

### Advanced Workflows
```bash
# From clipboard with auto-commit and AI summarization
git-branch-desc edit --input=clipboard --ai-summarize --commit

# From stdin with force and AI summarization (no prompts)
echo "Fix critical bug with detailed explanation..." | git-branch-desc edit --input=stdin --ai-summarize --force

# Edit and immediately push with AI summary and custom timeout
git-branch-desc edit --input=issue --issue-ref=123 --ai-summarize --ai-timeout 180 --push

# Handle very large content with extended timeout
git diff --cached | git-branch-desc edit --input=stdin --ai-summarize --ai-timeout 600 --commit
```

## Testing

The project includes a comprehensive test suite with unit tests, integration tests, and mock tests.

### Running Tests

#### Quick Test Commands
```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests --test mock_tests

# Run with verbose output
cargo test -- --verbose
```

#### Test Scripts
**Windows (PowerShell):**
```powershell
# Run all tests with comprehensive reporting
.\test.ps1 -All

# Run specific test types
.\test.ps1 -Unit
.\test.ps1 -Integration
.\test.ps1 -Coverage

# Run with filters
.\test.ps1 -Filter "parse_issue"
.\test.ps1 -Unit -Verbose
```

**Unix/Linux/macOS (Bash):**
```bash
# Run all tests with comprehensive reporting
./test.sh --all

# Run specific test types
./test.sh --unit
./test.sh --integration
./test.sh --coverage

# Run with filters
./test.sh --filter "parse_issue"
./test.sh --unit --verbose
```

### Test Coverage

#### Coverage Report
```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate HTML coverage report
cargo tarpaulin --out Html --output-dir target/coverage
open target/coverage/tarpaulin-report.html
```

#### Test Categories
- **Unit Tests** (`src/lib.rs`): Core functionality, parsing, utilities
- **Integration Tests** (`tests/integration_tests.rs`): Full workflows, Git operations
- **Mock Tests** (`tests/mock_tests.rs`): Edge cases, error conditions, boundary tests

### AI Testing (Optional)

AI-related tests require Ollama to be running locally:

```bash
# Setup Ollama
curl -fsSL https://ollama.ai/install.sh | sh
ollama run llama3.2:1b

# Run AI tests
cargo test --features ai_tests
```

### CI/CD

The project uses GitHub Actions for continuous integration:
- ✅ **Multi-platform testing** (Ubuntu, Windows, macOS)
- ✅ **Multiple Rust versions** (stable, beta)
- ✅ **Code formatting** and **linting**
- ✅ **Security audit**
- ✅ **Documentation checks**
- ✅ **Coverage reporting**
- ⚠️ **Optional AI tests**

See `.github/workflows/ci.yml` for full CI configuration.

## Development

```bash
# Format code
cargo fmt

# Run lints
cargo clippy --fix

# Run tests
cargo test

# Run comprehensive test suite
.\test.ps1 -All  # Windows
./test.sh --all  # Unix/Linux/macOS

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

### Hint, user GEMINI for AI

```
@REM short
gemini -y -p "sumarize diff between branches support/13.10 and support/13.0 in 2 or 3 sentenses for a branch description. Take commit messages and code diff. Do not explain your Procedure" | sed "/Data collection is disabled/d" | clip

@REM long
gemini -y -p "sumarize diff between branches support/11.10 and support/11.20. Take commit messages and code diff. Do not explain your Procedure. Markdown Output" | sed "/Data collection is disabled/d" | clip
```
