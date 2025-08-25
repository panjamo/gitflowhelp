# Git Branch Description Manager

Manage branch descriptions stored in `BRANCHREADME.md` files within each branch.

## Installation

```bash
cargo build --release
# or install globally
cargo install --path .
```

## Quick Start

```bash
# Add description to current branch
git-branch-desc add "OAuth2 authentication implementation"

# Add description to any branch without checkout
git-branch-desc add --branch feature/api "REST API implementation" --commit

# List all branch descriptions
git-branch-desc list

# Modify existing description
git-branch-desc modify "Updated OAuth2 with JWT tokens"
```

## Commands

| Command | Aliases | Description |
|---------|---------|-------------|
| `add` | `set` | Add/set branch description |
| `modify` | `edit` | Modify existing description |
| `list` | `ls` | List all branch descriptions |

### Options

- `--branch` / `-b` - Target branch (defaults to current)
- `--commit` / `-c` - Auto-commit changes
- `--push` / `-p` - Auto-commit and push
- `--detailed` / `-d` - Show full descriptions (list only)
- `--all` / `-a` - Include branches without descriptions (list only)
- `--force` / `-f` - Skip confirmations

## Key Features

- **Remote operations**: Modify any branch without checkout using low-level Git operations
- **Safety first**: Confirmation prompts when modifying non-current branches
- **Per-branch storage**: Each branch maintains its own `BRANCHREADME.md` file
- **Fast listing**: Read descriptions directly from Git objects without checkout
- **CI-friendly**: All commits include `[skip ci]` flag