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
./target/release/git-branch-desc add "Test description"
./target/release/git-branch-desc list
./target/release/git-branch-desc modify "Updated description"
```

## Architecture

### Core Components
- **main.rs**: Single-file implementation containing all functionality
- **CLI Structure**: Uses `clap` for command-line parsing with three main commands:
  - `add`/`set`: Add or set branch descriptions
  - `modify`/`edit`: Modify existing descriptions  
  - `list`/`ls`: List all branch descriptions

### Key Dependencies
- `git2`: Low-level Git operations for reading/writing to branches without checkout
- `clap`: Command-line argument parsing with derive macros
- `tabwriter`: Table formatting for list output
- `terminal_size`: Terminal width detection for text wrapping
- `anyhow`: Error handling

### Git Integration
- **Current Branch Operations**: Traditional Git workflow (write file → stage → commit)
- **Remote Branch Operations**: Low-level Git object manipulation to commit directly to target branches
- **Safety Features**: Confirmation prompts when modifying non-current branches, branch validation
- **Storage**: Each branch maintains its own `BRANCHREADME.md` file in the branch root

### Key Functions
- `add_or_modify_description()`: Main logic for add/modify operations with branch validation
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