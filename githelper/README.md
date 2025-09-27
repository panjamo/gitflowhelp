# Git Aliases

This repository contains a collection of Git aliases that can significantly speed up your workflow and simplify common Git operations.

## Installation

To use these aliases, you need to include them in your global Git configuration file.

1.  **Open your global Git configuration file:**
    *   On Windows: `C:\Users\<your_username>\.gitconfig`
    *   On macOS/Linux: `~/.gitconfig`

2.  **Add the following lines to your `.gitconfig` file:**

    ```ini
    [include]
        path = /path/to/your/githelper/.gitconfigalias
    ```

    **Important:** Replace `/path/to/your/githelper/.gitconfigalias` with the actual absolute path to this `.gitconfigalias` file on your system.

    **Example on Windows:**
    If you have cloned this repository to `D:/Development/gitflowhelper/githelper/`, your `.gitconfig` would look like:

    ```ini
    [include]
        path = d:/Development/gitflowhelper/githelper/.gitconfigalias
    ```

## Available Aliases

Here's a categorized list of the available Git aliases and their descriptions:

### Log and History Aliases

| Alias | Description |
| :---- | :---------- |
| `ls` | Show a log of all branches, including stashes. |
| `l` | Concise log of the last 5 commits (graph, one-line). |
| `ll` | Concise log of the last 15 commits (graph, one-line). |
| `lf` | Log of the last 5 commits (graph, one-line, with changed files). |
| `lfb` | Detailed log of the last 5 commits, showing relative dates, author, commit message, and branch decorations. Includes files. |
| `lb` | Detailed log of the last 5 commits, showing relative dates, author, commit message, and branch decorations. |
| `lbf` | Detailed log of the last 5 commits, showing relative dates, author, commit message, and branch decorations. Includes files. |
| `ln` | Log of the last 5 commits (graph, one-line, excluding merges). |
| `lsha` | Log showing only abbreviated commit hashes. |
| `lv` | View the diff of the last 3 non-merge commits in VS Code. |
| `lbr` | List the latest commit from each remote branch, sorted by commit date. |
| `lbv` | (Helper) Shows the last N commits for a given branch. |
| `lg` | Graph log with decorations, simplified, colors, and paginated. |
| `lgt` | Same as `lg` but explicitly for tags. |
| `lgv` | Graph log with branches, one-line, simplified, and decorated. |
| `lgvt` | Graph log with branches, one-line, simplified. |
| `lgvv` | Verbose graph log with decorations, relative dates, and branches. |
| `lgvvt` | Verbose graph log with simplified decorations, relative dates, and branches. |
| `ls` | Log with submodules, showing graph, one-line, and changes. |
| `lfind` | Detailed log format, useful for `search*` aliases. |
| `search` | Find commits where a string (regex) is in the diff (case-sensitive). |
| `searchi` | Find commits where a string (regex) is in the diff (case-insensitive). |
| `searchv` | Find commits where a string (regex) is in the diff and show the matching line of the diff. |
| `searchvi` | Find commits where a string (regex) is in the diff and show the matching line of the diff (case-insensitive). |
| `searchvv` | Find commits where a string (regex) is in the diff and show the matching line of the diff, with Git output. |
| `searchvvi` | Find commits where a string (regex) is in the diff and show the matching line of the diff, with Git output (case-insensitive). |
| `searchvvv` | Find commits where a string (regex) is in the diff and show the diff. |
| `searchvvvi` | Find commits where a string (regex) is in the diff and show the diff (case-insensitive). |
| `lc` | Log of commits from `ORIG_HEAD` to HEAD (stats, no merges). |
| `lp` | Log with patch and stat. |
| `unreleasedCommits` | Show commits since the last tag. |
| `releases` | List commits tagged with `Thi*`. |
| `showUnreachableCommits` | Show unreachable commits with stats. |
| `applyCommitToWorkingDirectory` | Apply a specific commit's diff to the working directory. |
| `uniqueCommits` | Show commits that are not in other branches. |
| `parent` | Get the parent commit of the current branch. |

### Branch and Branch Management Aliases

| Alias | Description |
| :---- | :---------- |
| `br` | List branches, sorted by committer date. |
| `brs [count]` | List the latest `count` branches (default 5), sorted by committer date. |
| `st` | Show branch status, including local branches not connected to remote, and ahead/behind status. |
| `sta` | Show branch status, including untracked files (no). |
| `co` | Checkout with `--recurse-submodules`. |
| `cohidden` | Checkout all hidden files. |
| `deleteBranchBoth [branch] [remote]` | Delete a branch locally and from a remote. |
| `dbb` | Alias for `deleteBranchBoth`. |
| `openfeat [feature-name]` | Create a new feature branch and push it upstream. |
| `closefeat [parent-branch]` | Rebase the current branch onto a parent branch for closing a feature. |
| `finishclosefeat [parent-branch]` | Finish closing a feature branch after rebase and conflict resolution. |
| `finishfeat [feature-name]` | Finish a feature branch by deleting it from both local and remote. |
| `prunebr` | Fetch and prune remote branches that have been deleted. |
| `storeDevCorrespondingSupportBranch` | Configures `devbranch.name` and `build` symbolic-ref based on the latest support branch. |

### Staging and Commit Aliases

| Alias | Description |
| :---- | :---------- |
| `ci` | Commit staged changes. |
| `ciam` | Commit staged changes with `-a` and `-m`. |
| `addp` | Interactively add patches, then commit. |
| `staged` | Show staged files. |
| `stagediff` | Stage all files with unstaged changes in the current directory. |
| `reset0diff` | Stage and then unstage all files with unstaged changes. |
| `stdiff` | Show unstaged changes. |
| `unstage` | Unstage changes from the index. |
| `fix` | Create a `fixup!` commit for the last commit. |
| `cia` | Add all changes and create a conventional commit message with emojis. |
| `snap` | Commit with the message "snap!". |
| `aco [file] [message]` | Add a file and commit with a message. |
| `diffcommit` | Use KDiff3 to diff a specific commit. |

### Working Directory and File Management Aliases

| Alias | Description |
| :---- | :---------- |
| `hide` | Mark files as assumed unchanged. |
| `unhide` | Unmark files as assumed unchanged. |
| `hidechanged` | Hide modified files. |
| `hidden` | List hidden files. |
| `unhideall` | Unhide all hidden files. |
| `hideModified` | Hide all modified files. |
| `resetfdx` | Hard reset, clean all untracked files and directories (including ignored, except specific patterns), and show status. |
| `resetfd` | Hard reset, clean all untracked files and directories, and show status. |
| `deleteAllTrackedFiles` | Delete all tracked files in the repository. |
| `exclude [file/dir...]` | Add files/directories to `.git/info/exclude`. |
| `excludeUnknown` | Add unknown files to `.git/info/exclude`. |
| `excluded` | Show the contents of `.git/info/exclude`. |
| `includeall` | Remove `.git/info/exclude`. |
| `theirs` | Checkout `--theirs` for specified files and add them. |
| `ours` | Checkout `--ours` for specified files and add them. |
| `ut` | Use theirs for a file during a merge conflict. |
| `uo` | Use ours for a file during a merge conflict. |
| `setxbit` | Set execute permissions for specified files. |
| `xbit` | Show files with execute permissions. |

### Remote and Fetching Aliases

| Alias | Description |
| :---- | :---------- |
| `cloner` | Clone a repository recursively. |
| `gg` | Fetch tags and prune, then show status. |
| `trackAll` | Track all remote branches locally. |
| `sync` | Pull, track all remotes, fast-forward all branches, and show status. |
| `pushfwl` | Force push with lease. |
| `delgonebr` | Delete local branches that have been removed from the remote. |

### Submodule Aliases

| Alias | Description |
| :---- | :---------- |
| `sub` | Run a command in all submodules. |
| `subget` | Update and initialize submodules recursively. |

### Diff and Mergetool Aliases

| Alias | Description |
| :---- | :---------- |
| `udiff` | Show diff between current branch and its upstream. |
| `mt` | Run the configured mergetool. |
| `remergefile` | Checkout a file with conflict=diff3. |
| `sc [commit-ish]` | Show the commit diff. |
| `scc` | Diff with `code`. |
| `scc1` | Diff with `code` (word-diff-regex: `[^[:space:]]|([[:alnum:]]|UTF_8_GUARD)+`). |
| `scc2` | Diff with `code` (word-diff-regex: `\w`). |
| `scc3` | Diff with `code` (word-diff-regex: `.`). |
| `vc [commit-ish]` | View diff of a commit in VS Code. |
| `vc2 [commit-ish]` | View diff of a commit in VS Code (uses a different diff file). |
| `vd [ref1] [ref2]` | View diff between two refs in VS Code. |
| `vd2 [ref1] [ref2]` | View diff between two refs in VS Code (uses a different diff file). |
| `d` | Diff with `--color-words -U0`. |
| `d1` | Diff with `--color-words="[^[:space:]]|([[:alnum:]]|UTF_8_GUARD)+" -U0`. |
| `d2` | Diff with `--word-diff-regex='\\w' -U0`. |
| `d3` | Diff with `--word-diff-regex=. -U0`. |
| `dt` | Use the configured difftool. |
| `dtd` | Use the configured difftool with `-d`. |
| `dtc` | Use VS Code as the difftool. |
| `dtcd` | Use VS Code as the difftool with `-d`. |
| `dtg [ref1] [ref2]` | Generate a GitLab compare URL and copy it to the clipboard. |
| `mrd [branch] [ref1] [ref2]` | Diff a merge base with VS Code. |
| `mergedevelop [default-branch]` | Merges the default branch into the current branch and vice-versa, then switches to the default branch. |
| `mergepushdevelop [default-branch]` | Merges and pushes the default branch and the current branch. |
| `md` | Merge `develop` and the default branch. |
| `mpd` | Merge and push `develop` and the default branch. |

### Search and Find Aliases

| Alias | Description |
| :---- | :---------- |
| `contains [commit-ish]` | Show commits that contain a specific commit, and list branches/tags containing it. |
| `find [pattern]` | Find commits where a pattern exists in the commit message or diff (case-insensitive). |
| `findv [pattern]` | Find commits where a pattern exists in the commit message or diff and show changed files (case-insensitive). |
| `findvv [pattern]` | Find commits where a pattern exists in the commit message or diff and show the diff (case-insensitive). |
| `findc [pattern]` | Find commits containing a pattern and list the branches/tags that contain them. |

### Configuration and Utility Aliases

| Alias | Description |
| :---- | :---------- |
| `list` | List all Git configuration. |
| `graph` | Open TortoiseGit revision graph. |
| `ffAll` | Fast-forward all branches that are behind their remote counterparts. |
| `ffAllForce` | Similar to `ffAll` but with force option, handling unstaged changes. |
| `flowhelp` | Open the Gitflow AVH wiki. |
| `alias` | List all defined Git aliases. |
| `generatesAliases` | Generate Git aliases from the output of `git alias`. |
| `templateFunction` | A placeholder for a template function. |
| `root` | Get the root directory of the Git repository. |
| `brdesc` | Execute `git-branch-desc.exe` with provided arguments. |
| `brdesclist` | List `BRANCHREADME.md` contents from all branches. |
| `ignore [file/dir...]` | Add files/directories to `.gitignore`. |
| `ignoreUnknown` | Add unknown files to `.gitignore`. |
| `serve` | Start a Git daemon. |
| `stm` | Show status (untracked files: no). |
| `stfu` | Show status (untracked files: no). |
| `rl` | Show reflog. |
| `deleteUnreachable` | Clean up unreachable objects and perform garbage collection. |
| `isClean` | Check if the repository is clean (no uncommitted changes). |
| `build-symbolic-ref` | Display the content of `.git/build` symbolic ref. |
| `curl` | Get the URL of the current commit and copy it to the clipboard. |
| `gl [topic]` | Get a GitLab URL for a topic or the current branch and copy it to the clipboard. |
| `jk` | Get a Jenkins search URL and copy it to the clipboard. |
| `nx` | Get a Jenkins search URL for a project and copy it to the clipboard. |

## Contributing

Contributions are welcome! If you have a useful Git alias you'd like to add, please submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.