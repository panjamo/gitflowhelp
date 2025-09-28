Of course. Here is a `README.md` file that documents the provided Git aliases, organized into logical groups and presented in a table format.

---

# Git Alias Helper (`.gitconfigalias`)

This document provides a reference for the custom Git aliases defined in the `.gitconfigalias` file. The aliases are grouped by functionality to make them easier to find and understand.

## How to Use

To use these aliases, include this file in your main Git configuration (`C:\Users\<user>\.gitconfig` or `~/.gitconfig`):

```ini
[include]
    path = /path/to/your/.gitconfigalias
```

---

### Logging & History

These commands provide enhanced and formatted views of your repository's history.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `l` | | Shows a compact, one-line graph log of the last 5 commits. |
| `ll` | | Shows a compact, one-line graph log of the last 15 commits. |
| `lf` | | Like `git l`, but also shows the names of files changed in each commit. |
| `lfb` | | Shows a beautifully formatted graph log of the last 5 commits (excluding merges) with file names. |
| `lb` | | Shows a beautifully formatted graph log of the last 5 commits, excluding merges. |
| `lbf` | | Shows a beautifully formatted log of the last 5 commits, including merges. |
| `ln` | | Shows a compact, one-line graph log of the last 5 non-merge commits. |
| `lsha` | | Lists only the short SHA hashes of commits. |
| `lv` | `[<path/ref>...]` | Shows a verbose log with patches for the last 3 commits and opens the output in VS Code. |
| `lbr` | `[<count>]` | Fetches and displays the most recent commit for each remote branch, sorted by date. |
| `lg` / `lgt` | | Shows an interactive (via `less`) graph of all branches, simplifying by decoration. |
| `lgv` | | Shows a simplified one-line graph of all branches, excluding tags. |
| `lgvt` | | Shows a simplified one-line graph of all branches, including tags. |
| `lgvv` / `lgvvt` | | Shows a more verbose, decorated graph of all branches (with/without tags). |
| `ls` | `[<path/ref>...]` | Shows a log of the last 5 commits with submodule changes displayed inline. |
| `lc` | | Shows a log with stats for commits made since the `ORIG_HEAD` (e.g., after a merge or rebase). |
| `lp` | | Shows a log with the full patch and stats for each commit. |
| `rl` | `[<git-reflog-options>]` | Alias for `git reflog`. |
| `unreleasedCommits` | | Shows all commits made since the last annotated tag. |
| `uniqueCommits` | | Finds commits that are unique to the current branch history. |
| `showUnreachableCommits` | | Displays unreachable commit objects found by `fsck`. |
| `contains` | `<commit-sha>` | Finds which branches and tags contain the specified commit. |

### Searching History

Commands for finding specific changes or commits in the repository's history.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `lfind` | `[<git-log-options>]` | Base command for other search aliases, providing a formatted log output. |
| `search` | `<regex> [<path>...]` | Finds commits where the given regex is present in the diff (case-sensitive). |
| `searchi` | `<regex> [<path>...]` | Case-insensitive version of `search`. |
| `searchv` | `<regex> [<path>...]` | Case-insensitive search that shows matching commits and the names of files changed. |
| `searchvi` | `<regex> [<path>...]` | Case-sensitive version of `searchv`. |
| `searchvv` | `<regex> [<path>...]` | Case-sensitive search that shows matching lines from the diff. |
| `searchvvi` | `<regex> [<path>...]` | Case-insensitive version of `searchvv`. |
| `searchvvv` | `<regex> [<path>...]` | Finds commits with matching text in the diff and shows the full diff (case-sensitive). |
| `searchvvvi` | `<regex> [<path>...]` | Case-insensitive version of `searchvvv`. |
| `find` | `<regex> [<path>...]` | Finds commits where the commit message matches the regex (case-insensitive). |
| `findv` | `<regex> [<path>...]` | Like `find`, but also shows file names. |
| `findvv` | `<regex> [<path>...]` | Like `find`, but also shows the full patch. |
| `findc` | `<regex> [<path>...]` | Finds commits by message and then shows which branches contain them. |

### Status & Staging

Commands for inspecting the working directory, staging changes, and managing state.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `st` | | Comprehensive status: shows branch tracking, ahead/behind info, and short status. |
| `sta` | | Like `st`, but also explicitly lists untracked files. |
| `stm` / `stfu` | | Shows status without listing untracked files. |
| `staged` | | Lists the names of files that are currently staged (in the index). |
| `unstage` | `[<file>...]` | Unstages files from the index, moving them back to unstaged changes. |
| `addp` | | Interactively stage parts of files, ignoring whitespace differences. |
| `stagediff` | | Stages all tracked files that have modifications. |
| `stdiff` | | Shows the names of tracked files that have modifications. |

### Committing

Aliases to streamline the commit process.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `ci` | `[<git-commit-options>]` | Alias for `commit`. |
| `ciam` | `[<git-commit-options>]` | Alias for `commit -am` (add and commit tracked files). |
| `snap` | | Commits all tracked, modified files with the message "snap!". |
| `fix` | `[<file>...]` | Interactively stages parts of files and commits them as a fixup to the most recent non-fixup commit. |
| `fixup` | | Creates a `fixup!` commit for the current `HEAD`. |
| `cia` | `[<file>...]` | Uses an external tool (`gia`) to generate a conventional commit message, then opens it in an editor. |
| `aco` | `<file> "<message>"` | Adds a single file and commits it with the provided message. |

### Branching & Merging

Commands for managing branches and merge operations.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `br` | `[<git-branch-options>]` | Lists local branches, sorted by most recent commit date. |
| `brs` | `[<count>] [<options>]` | Lists the N most recently committed-to local branches (defaults to 5). |
| `co` | `[<git-checkout-options>]` | Alias for `checkout --recurse-submodules`. |
| `mt` | | Alias for `mergetool`. |
| `branchdiff` | `<branch1> <branch2>` | Shows commits that are in one branch but not the other, for both branches. |
| `parent` | | Shows the parent branch of the current branch in the commit graph. |
| `delgonebr` | | Deletes local branches whose remote tracking branch has been removed. |
| `deleteBranchBoth` / `dbb` | `[-f] <branch>` | Deletes a branch locally and on the `origin` remote. |
| `prunebr` | | Fetches and prunes remotes, then deletes local gone branches. |
| `ut` / `theirs` | `<file>` | During a merge conflict, accepts the "theirs" version of a file and stages it. |
| `uo` / `ours` | `<file>` | During a merge conflict, accepts the "ours" version of a file and stages it. |
| `remergefile` | `<file>` | During a conflict, re-runs the checkout for a file to show conflict markers again. |

### Diffing & Viewing Changes

Commands to compare commits, branches, and files.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `udiff` | `[<path>...]` | Shows the diff between the current branch and its upstream tracking branch. |
| `sc` | `<commit-sha>` | Shows a directory diff of a commit against its parent using the configured difftool. |
| `scc` / `scc1` / `scc2` / `scc3` | `<commit-sha>` | Shows a colored word/char diff of a commit against its parent in the console. |
| `vc` / `vc2` | `<commit-sha> [<path>...]` | Diffs a commit against its parent and opens it in VS Code. |
| `vd` / `vd2` | `<commit1> <commit2> [...]`| Diffs two commits/branches and opens the result in VS Code. |
| `vdmr` | `[<path>...]` | Visual diff against the merge-base of the currently configured "merge branch". |
| `vdmrinit` | | Prompts to set the "merge branch" used by `vdmr`. |
| `dt` / `dtd` | `[<options>]` | Alias for `difftool` and `difftool -d`. |
| `dtc` / `dtcd`| `[<options>]` | Runs `difftool` explicitly using VS Code as the tool. |
| `diffcommit` | `<commit-sha>` | Shows a directory diff of a commit against its parent using `kdiff3`. |
| `mrd` | `<branch1> [<branch2>]` | Diffs a branch against the merge-base of another branch (defaults to HEAD). |
| `d` / `d1` / `d2` / `d3` | `[<options>]`| Various forms of word/character-based diffs directly in the terminal. |

### Remote & Synchronization

Commands for interacting with remote repositories.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `cloner` | `<repo-url>` | Alias for `clone --recursive`. |
| `gg` | | Fetches pruned tags and then runs `git st`. |
| `trackAll` | | Sets up local branches to track all existing remote branches from `origin`. |
| `bbehind` | | Lists local branches that are behind their remote counterparts. |
| `bahead` | | Lists local branches that are ahead of their remote counterparts. |
| `ffAll` | | Fast-forwards all local branches that are behind their remotes. Aborts if not clean. |
| `ffAllForce`| | Stashes changes, runs `ffAll`, and then pops the stash. |
| `sync` | | Pulls, tracks all remotes, fast-forwards all branches, and shows status. |
| `pushfwl` | `[<options>]`| Alias for `push --force-with-lease`. |
| `mp` | | Deprecated alias, redirects to `git st`. |

### Custom Workflows

Multi-step processes for common development tasks.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| **Feature Flow** | | |
| `openfeat` | `<name>` | Creates and pushes a new `feature/<name>` branch. |
| `closefeat` | `<parent-branch>` | Interactively rebases the current feature branch onto the specified parent branch. |
| `finishclosefeat` | `<parent-branch>` | Merges the temporary rebased branch into the parent and cleans up. Used after `closefeat`. |
| `finishfeat` | `<name>` | Deletes the `feature/<name>` branch locally and remotely. |
| **Merge Develop** | | |
| `mergedevelop` | `[<branch>]` | Merges the specified branch (or configured `devbranch.name`) with `develop` and vice-versa. |
| `mergepushdevelop` | `[<branch>]` | Runs `mergedevelop` and then pushes both branches to origin. |
| `md` | `[<branch>]` | Interactive wrapper for `mergedevelop`. Asks for confirmation. |
| `mpd` | `[<branch>]` | Interactive wrapper for `mergepushdevelop`. Asks for confirmation. |

### File & Working Directory Management

Commands for managing files, including untracked, ignored, and hidden files.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `hide` | `<file>...` | Marks file(s) as "assume-unchanged", hiding them from `git status`. |
| `unhide` | `<file>...` | Unmarks file(s) as "assume-unchanged". |
| `hidechanged` | | Hides all currently modified (but unstaged) files. |
| `hidden` | | Lists all files currently marked as "assume-unchanged". |
| `cohidden` | | Force checks out the repository version of all hidden (assume-unchanged) files. |
| `unhideall` | | Un-hides all files currently marked as "assume-unchanged". |
| `hideModified` | | Hides all currently modified files. |
| `resetfdx` | | Hard resets, then aggressively cleans the directory, excluding certain patterns. |
| `resetfd` | | Hard resets and cleans the directory (`clean -fd`). |
| `deleteAllTrackedFiles`| | **DANGEROUS**: Deletes all files tracked by Git from the working directory. |
| `root` | | Prints the root directory of the repository. |
| `ignore` | `<pattern>...` | Appends pattern(s) to the root `.gitignore` file. |
| `ignoreUnknown`| | Appends all current untracked files to the root `.gitignore`. |
| `exclude` | `<pattern>...` | Appends pattern(s) to `.git/info/exclude` to be ignored locally. |
| `excludeUnknown` | | Appends all current untracked files to `.git/info/exclude`. |
| `excluded` | | Shows the contents of the local exclude file and runs `git st`. |
| `includeall` | | Clears the local exclude file (`.git/info/exclude`). |
| `setxbit` | `[<path>]` | Sets the executable bit for the specified file(s). |

### Submodules & External Tools

Commands for submodules and interacting with external programs.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `sub` | `<git-command>` | Runs the specified Git command in each submodule. |
| `subget` | | Initializes and recursively updates all submodules. |
| `graph` | | Opens the TortoiseGit revision graph GUI. |
| `dtg` | `<commit1> [<commit2>]` | Opens a GitLab "compare" URL for the given commits in a browser. |
| `curl` | `[<commit-sha>]` | Opens the GitLab commit URL for the specified commit (or HEAD) in a browser. |
| `gl` | `[<topic>]` | Opens a GitLab URL for the current repo/branch. Defaults to `README.md`. Can be `network`, etc. |
| `jk` | | Opens the Jenkins search page for the current repository name. |
| `nx` | | Opens the Nexus search page for artifacts related to the current repository. |
| `flowhelp` | | Opens the `gitflow-avh` wiki in a browser. |
| `brdesc` | `[<options>]` | Runs the external `git-branch-desc.exe` tool. |
| `brdesclist`| | Lists the contents of `BRANCHREADME.md` from all local and remote branches. |

### Repository Maintenance & Meta

Commands for repository cleanup and introspection.

| Alias | Parameters | Description |
| :--- | :--- | :--- |
| `list` | | Lists all Git configuration settings, showing their origin and scope. |
| `alias` | `[<search-term>]` | Lists all defined aliases, optionally filtered by a search term. |
| `isClean` | | Checks if the working directory is clean. Exits with an error if it is not. |
| `findDangelingCommits` | | Finds and logs information about unreachable (dangling) commits. |
| `deleteUnreachable` | | **DANGEROUS**: Expires reflogs and garbage collects all unreachable objects. |
| `applyCommitToWorkingDirectory`| `<commit-sha>`| Applies the changes from a specific commit as a patch to the current working directory. |