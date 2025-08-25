# Git Branch Description Manager

A command-line tool to manage branch descriptions stored in `BRANCHREADME.md` files within each branch.

## Features

- **Add/Modify**: Set descriptions for any branch (stored in target branch's `BRANCHREADME.md`)
- **List**: View all local and remote branch descriptions by reading `BRANCHREADME.md` from each branch via Git (without checkout)
- **Remote Operations**: Modify any local branch without checking it out using low-level Git operations
- **Per-branch storage**: Each branch maintains its own `BRANCHREADME.md` file

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/git-branch-desc`.

You can also install it globally:

```bash
cargo install --path .
```

## Usage

### Add or Set a Branch Description

Add a description for the current branch:

```bash
git-branch-desc add "This branch implements user authentication"
```

Add a description for any local branch (without checking it out):

```bash
git-branch-desc add --branch feature/user-auth "OAuth2 authentication implementation"
```

Or add interactively (will prompt for input):

```bash
git-branch-desc add
```

Add with automatic commit:

```bash
git-branch-desc add "New feature description" --commit
```

Add with automatic commit and push:

```bash
git-branch-desc add "New feature description" --push
```

### Modify an Existing Description

Modify the description for the current branch:

```bash
git-branch-desc modify "Updated description for user authentication feature"
```

Modify the description for any local branch (without checking it out):

```bash
git-branch-desc modify --branch feature/api-v2 "Updated API implementation with better error handling"
```

Or modify interactively (will show current description first):

```bash
git-branch-desc modify
```

Modify with automatic commit:

```bash
git-branch-desc modify "Updated description" --commit
```

Modify with automatic commit and push:

```bash
git-branch-desc modify "Updated description" --push
```

### List All Branch Descriptions

View descriptions for all local and remote branches in a table format:

```bash
git-branch-desc list
```

View full descriptions with detailed formatting:

```bash
git-branch-desc list --detailed
# or
git-branch-desc list -d
```

View all branches including those without descriptions:

```bash
git-branch-desc list --all
# or
git-branch-desc list -a
```

## File Storage

Each branch stores its description in its own `BRANCHREADME.md` file in the branch root.

- When you add/modify a description, it writes to `BRANCHREADME.md` in your current working directory
- When listing descriptions, the tool reads `BRANCHREADME.md` from each branch using Git (without checking out)
- Each branch maintains its description independently

## How It Works

### Add/Modify Operations
- **Current Branch**: Write directly to `BRANCHREADME.md` in the current working directory
- **Remote Branch** (with `--branch`): Use low-level Git operations to commit directly to the target branch
- Include safety warnings and confirmations when modifying non-current branches
- Require `--commit` or `--push` flags when modifying non-current branches

### List Operation
- Uses Git to read `BRANCHREADME.md` from each local and remote branch's tree
- No checkout required - reads files directly from Git objects
- Shows descriptions for all local and remote branches that have a `BRANCHREADME.md` file

## Commands and Aliases

- `add` (alias: `set`) - Add or set a branch description
  - `--branch` / `-b` - Target branch name (defaults to current branch)
  - `--commit` / `-c` - Automatically commit the BRANCHREADME.md file
  - `--push` / `-p` - Automatically commit and push the BRANCHREADME.md file
  - `--force` / `-f` - Skip confirmation prompts (force operation)
- `modify` (alias: `edit`) - Modify an existing branch description  
  - `--branch` / `-b` - Target branch name (defaults to current branch)
  - `--commit` / `-c` - Automatically commit the BRANCHREADME.md file
  - `--push` / `-p` - Automatically commit and push the BRANCHREADME.md file
  - `--force` / `-f` - Skip confirmation prompts (force operation)
- `list` (alias: `ls`) - List all local and remote branch descriptions
  - `--detailed` / `-d` - Show full descriptions with terminal-aware text wrapping
  - `--all` / `-a` - Show all branches, including those without descriptions

## Examples

```bash
# Add a description to the current branch
git-branch-desc add "Implementing OAuth2 login flow"

# Add with automatic commit
git-branch-desc add "OAuth2 implementation" --commit

# Add with automatic commit and push
git-branch-desc add "OAuth2 implementation" -p

# Add to a different branch without checking it out
git-branch-desc add --branch feature/database "Database migration and schema updates" --commit

# Modify interactively
git-branch-desc edit

# Modify a different branch with confirmation
git-branch-desc modify --branch feature/api-v2 "Updated REST API with new endpoints" --push

# Force modification without confirmation prompts
git-branch-desc add --branch feature/database "Database schema updates" --commit --force

# Modify with auto-commit using short flag
git-branch-desc modify "Updated OAuth2 flow" -c

# List all descriptions in table format
git-branch-desc ls

# List with full descriptions
git-branch-desc ls --detailed

# List all branches including those without descriptions
git-branch-desc ls --all

# Combined flags: detailed view with all branches
git-branch-desc ls -ad
```

## Output Examples

### Table View (Default)
```
Branch                     Description
------                     -----------
feature/user-auth          OAuth2 authentication implementation with JWT tokens and session management...
main                       Main production branch with stable code and latest releases...
origin/feature/api-v2      New REST API version with enhanced performance and documentation...
origin/main                Remote main branch synced with production environment...

Use --detailed (-d) flag to see full descriptions, --all (-a) to include branches without descriptions.
```

### Table View with --all Flag
```
Branch                     Description
------                     -----------
feature/user-auth          OAuth2 authentication implementation with JWT tokens and session management...
feature/temp-fix           <no description>
main                       Main production branch with stable code and latest releases...
origin/feature/api-v2      New REST API version with enhanced performance and documentation...
origin/main                Remote main branch synced with production environment...

Use --detailed (-d) flag to see full descriptions.
```

### Detailed View
```
┌─ feature/user-auth ────────────────────────────────────────────
│ OAuth2 authentication implementation with JWT tokens.
│ Includes login, logout, and user session management.
│ Breaking changes from previous auth system.
└────────────────────────────────────────────────────────────────

┌─ main ─────────────────────────────────────────────────────────
│ Main production branch with stable code and latest releases.
│ All features are thoroughly tested before merging here.
└────────────────────────────────────────────────────────────────
```

## Workflow Examples

### Traditional Workflow
```bash
# Traditional workflow
git checkout feature/user-auth
git-branch-desc add "OAuth2 authentication implementation"
git add BRANCHREADME.md
git commit -m "Add branch description"

# Streamlined workflow with auto-commit
git checkout feature/api-v2
git-branch-desc add "New REST API version with breaking changes" --commit

# Auto-commit and push in one command
git checkout feature/mobile-app
git-branch-desc add "Mobile app React Native implementation" --push
```

### Remote Branch Operations (New!)
```bash
# Add descriptions to branches without checking them out
git-branch-desc add --branch feature/database "Database schema migration with new user tables" --commit

# Modify any branch from anywhere with safety confirmation
git-branch-desc modify --branch feature/payment "Updated payment processing with Stripe integration" --push

# Interactive modification of remote branch (shows current description first)
git-branch-desc modify --branch feature/auth --commit

# Batch documentation - add descriptions to multiple branches
git-branch-desc add --branch feature/notifications "Push notification system" --commit
git-branch-desc add --branch feature/analytics "User analytics and tracking" --commit
git-branch-desc add --branch feature/search "Elasticsearch integration" --commit

# Batch operations with force mode (no confirmations)
git-branch-desc add --branch feature/auth "Authentication system" -cf
git-branch-desc add --branch feature/billing "Billing integration" -cf
git-branch-desc add --branch feature/admin "Admin dashboard" -cf
```

### CI/CD Integration
```bash
# All commits automatically include [skip ci] to prevent unnecessary pipeline runs
git-branch-desc add "Feature implementation complete" --commit
# Commits as: "Add branch description [skip ci]"

git-branch-desc modify "Updated implementation details" --push  
# Commits as: "Update branch description [skip ci]"
```

### Listing and Discovery
```bash
# List all descriptions (from any branch)
git-branch-desc list
# Shows descriptions from all branches without switching

# See all branches including work-in-progress ones without descriptions
git-branch-desc list --all
# Shows branches like "feature/wip-auth" even if no BRANCHREADME.md exists yet

# Combine flags for comprehensive view
git-branch-desc list --all --detailed
# Shows detailed view of all branches, including those without descriptions
```

### Safety Features
```bash
# The tool will warn you when modifying a different branch
git-branch-desc add --branch feature/critical "Important changes" --push
# ⚠️  WARNING: You are about to modify branch 'feature/critical' while on 'main'
# This will create a commit directly on the target branch.
# Continue? [y/N]: 

# Attempting to modify without --commit will show helpful guidance
git-branch-desc add --branch feature/test "Test description"
# ⚠️  WARNING: Description prepared for branch 'feature/test' but not committed.
# Use --commit or --push to save the changes to the branch.

# Force mode bypasses confirmations for automation
git-branch-desc modify --branch feature/critical "Urgent fix applied" --push --force
# 🚀 Force mode: Modifying branch 'feature/critical' without confirmation
```

## Requirements

- Must be run from within a Git repository
- Rust 1.70+ for building from source

## Notes

### Branch Operations
- **Current Branch**: Traditional workflow - writes to working directory, requires manual commit
- **Remote Branch** (with `--branch`): Uses low-level Git operations to commit directly to target branch
- Requires `--commit` or `--push` when modifying non-current branches (safety feature)
- Includes confirmation prompts when modifying branches you're not currently on

### Listing and Display
- List command reads descriptions from all local and remote branches without checkout
- Remote branches are displayed with their remote name prefix (e.g., `origin/main`, `upstream/feature`)
- Local branches always take priority over remote branches with the same name
- Local branches with committed `BRANCHREADME.md` files are shown even if not yet pushed to remote
- The tool reads Git objects directly for listing, so it's fast and doesn't affect your working directory
- Detailed view automatically adapts text wrapping to your terminal width (90%) for optimal readability
- Use `--all` / `-a` flag to see all branches, including those without descriptions (helpful for finding branches that need documentation)

### Safety and Best Practices
- `BRANCHREADME.md` files should be committed to preserve descriptions
- Use `--commit` / `-c` flag to automatically commit changes
- Use `--push` / `-p` flag to automatically commit and push changes (requires configured remote)
- Each branch maintains its own independent description file
- The tool validates branch existence and provides helpful error messages
- Safety warnings prevent accidental modifications of the wrong branch
- Use `--force` / `-f` to bypass confirmation prompts for automation or batch operations
- Repository state checks ensure operations don't interfere with your working directory
- All commits include `[skip ci]` to prevent triggering CI/CD pipelines for documentation changes