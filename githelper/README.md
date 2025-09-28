# Git Alias Reference

This document provides a comprehensive reference for the custom Git aliases defined in the `.gitconfigalias` file. The aliases are grouped by functionality for easier navigation.

### Table of Contents
*   [Log & History](#log--history)
*   [Searching](#searching)
*   [Diff & View](#diff--view)
*   [Branching](#branching)
*   [Working Directory & Staging](#working-directory--staging)
*   [Merge & Conflict Resolution](#merge--conflict-resolution)
*   [Remote & Syncing](#remote--syncing)
*   [Feature Workflow](#feature-workflow)
*   [Advanced Merge Workflow](#advanced-merge-workflow)
*   [Hide / Ignore / Exclude](#hide--ignore--exclude)
*   [Submodules](#submodules)
*   [External Links & Tools](#external-links--tools)
*   [Repository Maintenance](#repository-maintenance)
*   [File Permissions](#file-permissions)
*   [Miscellaneous Utilities](#miscellaneous-utilities)

---

## Log & History

Commands for viewing commit history in various formats.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `l` | | Shows the last 5 commits in a compact, one-line graph format. |
| `ll` | | Shows the last 15 commits in a compact, one-line graph format. |
| `lf` | | Like `l`, but also shows the names of changed files. |
| `ln` | | Like `l`, but excludes merge commits. |
| `ls` | `[path...]` | Shows the last 5 commits with submodule diffs. |
| `lb` | | Shows a decorated, formatted log of the last 5 non-merge commits. |
| `lfb` | | Like `lb`, but also shows changed file names. |
| `lbf` | | Like `lb`, but includes merge commits. |
| `lg` | | Shows a graphical log of all branches and tags, simplified by decoration. |
| `lgv` | | Shows a compact, one-line graphical log of all branches. |
| `lgvv` | | Shows a detailed, decorated graphical log of all branches. |
| `lgt`, `lgvt`, `lgvvt` | | Variants of `lg`, `lgv`, and `lgvv` that include tags in the decoration simplification. |
| `lsha` | | Lists only the short commit SHAs from the log. |
| `lv` | `[path...]` | Shows a diff of the last 3 commits for given file(s) and opens it in VS Code. |
| `lbr` | `[count]` | Displays the most recent commit for each remote branch, sorted by date. |
| `lc` | | Shows commits since the last operation that changed HEAD (e.g., merge, reset). |
| `lp` | | Shows the log with full patch diffs and stats. |
| `rl` | | Displays the reflog. |
| `unreleasedCommits` | | Lists all commits made since the last annotated tag. |

## Searching

Commands for finding commits by content in diffs or commit messages.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `search` | `<regex>` | Finds commits where the diff contains a string (case-sensitive, word match). |
| `searchi` | `<regex>` | Case-insensitive version of `search`. |
| `searchv` | `<regex>` | Like `search`, but shows file names and diffs. |
| `searchvi` | `<regex>` | Case-insensitive version of `searchv`. |
| `searchvv` | `<regex> [path...]` | Like `searchv`, but filters the diff to only show matching lines. |
| `searchvvi` | `<regex> [path...]` | Case-insensitive version of `searchvv`. |
| `contains` | `<commit>` | Shows all branches and tags that contain the given commit. |
| `find` | `<pattern> [path...]` | Finds commits by commit message content (case-insensitive). |
| `findv` | `<pattern> [path...]` | Like `find`, but also shows diffs with file names. |
| `findvv` | `<pattern> [path...]` | Like `find`, but shows full diffs. |
| `findc` | `<pattern> [path...]` | Finds commits by message, then shows which branches/tags contain them. |

## Diff & View

Commands for comparing changes and viewing content.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `d` | | `diff` using a color-words algorithm. |
| `d1`, `d2`, `d3` | | `diff` using different word/character regex algorithms. |
| `udiff` | | Shows differences between the current branch and its upstream branch (`@{u}`). |
| `sc` | `<commit>` | Shows a commit's changes using the configured `difftool`. |
| `scc`, `scc1`,... | `<commit>` | Shows a commit's changes in the console using various word-diff algorithms. |
| `vc` | `<commit> [file...]` | Views a commit's changes in VS Code. |
| `vd` | `<ref1> <ref2> [path...]` | Views the diff between two refs in VS Code. |
| `vdmrinit` | | Interactively prompts to set the default branch for `vdmr`. |
| `vdmr` | `[path...]` | Views the diff between `HEAD` and the configured merge branch in VS Code. |
| `mrd` | `<branch1> [branch2]` | Views the diff from the merge-base between branches. |
| `dt` | | Launches the configured graphical difftool. |
| `dtd` | | Launches `difftool` in directory-diff mode. |
| `dtc` | | Launches `difftool` specifically using VS Code. |
| `dtcd` | | Launches directory-diff specifically using VS Code. |
| `dtg` | `<ref1> [ref2]` | Opens the GitLab compare view URL for the given refs in a browser. |
| `dia` | `"<prompt>"` | Pipes the current diff to an AI (`gia`) to generate a summary. |

## Branching

Commands for managing branches.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `br` | | Lists local branches, sorted by most recent commit date. |
| `brs` | `[-n] [options]` | Lists the top N (default 5) most recent local branches. |
| `delgonebr` | | Deletes local branches whose remote counterparts are gone. |
| `prunebr` | | Fetches (pruning), then deletes local branches for gone remotes. |
| `dbb` | `<branch> [-f]` | Deletes a branch locally and on the `origin` remote. |
| `deleteBranchBoth` | `<branch> [-f]` | Alias for `dbb`. |
| `trackAll` | | Creates local tracking branches for all remote branches. |
| `branchdiff` | `<branch1> <branch2>` | Shows commits unique to each of the two branches. |
| `parent` | | Shows the parent branch of the current branch from the log history. |

## Working Directory & Staging

Commands for managing the state of your files.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `st` | | An enhanced `git status` showing branch tracking info and short status. |
| `sta` | | Like `st`, but also lists untracked files separately. |
| `stm` / `stfu` | | `status --untracked-files=no`. |
| `co` | | `checkout --recurse-submodules`. |
| `ci` | | `commit`. |
| `ciam` | | `commit -am`. |
| `aco` | `<file> "<message>"` | Adds a single file and commits with a specific message. |
| `snap` | | Commits all changes with the message "snap!". |
| `staged` | | Lists the names of all staged files. |
| `unstage` | `<file...>` | Unstages file(s) from the index. |
| `addp` | | Interactively stages parts of files, ignoring whitespace changes. |
| `stdiff` | | Shows the names of all modified (but not staged) files. |
| `resetfd` | | Hard resets and cleans the working directory (`-fd`). |
| `resetfdx` | | Hard resets and cleans the working directory, including ignored files (`-fdx`). |
| `isClean` | | A script that fails if the repository has uncommitted changes. |

## Merge & Conflict Resolution

Commands to help with merging branches and resolving conflicts.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `mt` | | `mergetool`. |
| `fixup` | | Creates a `fixup!` commit for the previous commit (`HEAD`). |
| `fix` | `[file...]` | Interactively stages changes and creates a `fixup!` commit for the latest non-fixup commit. |
| `remergefile` | `<file>` | Re-checks out a conflicted file with `diff3` style markers. |
| `ut` / `theirs` | `<file...>` | Resolves conflict by taking the "theirs" version of a file and staging it. |
| `uo` / `ours` | `<file...>` | Resolves conflict by taking the "ours" version of a file and staging it. |

## Remote & Syncing

Commands for interacting with remote repositories.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `gg` | | Fetches with tags and prunes, then shows status. A quick update. |
| `pushfwl` | | `push --force-with-lease`. A safer force push. |
| `ffAll` | | Fetches and fast-forwards all local branches that are behind their remotes. |
| `ffAllForce` | | Stashes changes if needed, runs `ffAll`, then pops the stash. |
| `sync` | | Pulls, tracks remote branches, and runs `ffAllForce`. A comprehensive sync. |
| `serve` | | Serves the repository over the `git://` protocol. |
| `cloner` | | `clone --recursive`. |
| `mp` | | Deprecated alias, now runs `git st`. |

## Feature Workflow

A set of commands for a specific feature branch workflow.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `openfeat` | `<name>` | Creates and pushes a new `feature/<name>` branch. |
| `closefeat` | `<parent-branch>` | Interactively rebases the current feature branch onto a parent branch. |
| `finishclosefeat` | `<parent-branch>` | Finalizes the `closefeat` action by merging and cleaning up. |
| `finishfeat` | `<name>` | Deletes the `feature/<name>` branch both locally and remotely. |

## Advanced Merge Workflow

Commands for a specific workflow of merging between `develop` and a dev/support branch.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `mergedevelop` | `<dev-branch>` | Performs a series of merges between the current branch, `develop`, and `<dev-branch>`. |
| `mergepushdevelop`| `<dev-branch>` | Runs `mergedevelop` and then pushes both branches to origin. |
| `md` | `[dev-branch]` | Interactive wrapper for `mergedevelop` using a configured default branch. |
| `mpd` | `[dev-branch]` | Interactive wrapper for `mergepushdevelop` using a configured default branch. |

## Hide / Ignore / Exclude

Commands for managing which files Git should ignore or temporarily stop tracking.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `hide` | `<file...>` | Hides tracked files from Git (`assume-unchanged`). |
| `unhide` | `<file...>` | Un-hides files (`no-assume-unchanged`). |
| `hidechanged` | | Hides all currently modified tracked files. |
| `hideModified` | | Alias for `hidechanged`. |
| `hidden` | | Lists all hidden files. |
| `cohidden` | | Checks out the `HEAD` version of all hidden files. |
| `unhideall` | | Un-hides all hidden files in the repository. |
| `ignore` | `<pattern...>` | Appends patterns to the project's `.gitignore` file. |
| `ignoreUnknown` | `[path]` | Appends all currently untracked files to `.gitignore`. |
| `exclude` | `<pattern...>` | Appends patterns to the local `.git/info/exclude` file. |
| `excludeUnknown` | `[path]` | Appends all currently untracked files to the local exclude file. |
| `excluded` | | Displays the contents of the local exclude file. |
| `includeall` | | Clears the local exclude file, making Git track all previously excluded files again. |

## Submodules

Commands for working with Git submodules.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `sub` | `<command...>` | Runs a specified Git command in each submodule. |
| `subget` | | Initializes, fetches, and updates all submodules recursively. |

## External Links & Tools

Commands that open external applications or websites.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `curl` | `[commit]` | Opens the GitLab commit URL for the given commit (or `HEAD`) and copies it. |
| `gl` | `[topic]` | Opens the GitLab URL for the current repo/branch, optionally to a specific topic (e.g., `network`). |
| `jk` | | Opens the Jenkins search URL for the current repository name. |
| `nx` | | Opens the Nexus search URL for artifacts related to the current repository. |
| `graph` | | Opens the TortoiseGit revision graph GUI. |
| `flowhelp` | | Opens the git-flow-avh wiki in a browser. |

## Repository Maintenance

Commands for cleaning up and managing the repository.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `findDangelingCommits`| | Finds and shows unreachable "dangling" commits. |
| `showUnreachableCommits` | | Shows details and stats for unreachable commits. |
| `deleteUnreachable` | | Expires the reflog and garbage collects unreachable objects. |
| `applyCommitToWorkingDirectory` | `<commit>` | Applies the changes from a commit as a patch to the working directory. |
| `deleteAllTrackedFiles` | | Deletes all files tracked by Git from the working directory. |

## File Permissions

Commands for managing file permissions in the Git index.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `xbit` | `[path]` | Shows staging information for files, useful for checking the executable bit. |
| `setxbit` | `[path]` | Sets the executable bit (`+x`) on specified files in the index. |

## Miscellaneous Utilities

Other helpful commands.

| Alias | Parameters | Description |
| :---- | :--- | :---------- |
| `list` | | Shows all Git configuration settings, their scope, and origin. |
| `alias` | `[pattern]` | Lists all defined Git aliases, optionally filtered by a pattern. |
| `root` | | Prints the absolute path to the repository's root directory. |
| `cia` | `[file...]` | Uses an AI (`gia`) to generate a conventional commit message for staged changes. |
| `brdesc` | `<command...>` | External script for managing branch descriptions. |
| `brdesclist` | | Lists the content of `BRANCHREADME.md` from all branches. |