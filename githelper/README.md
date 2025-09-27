# Git Aliases

This repository provides a collection of useful Git aliases to streamline your workflow. These aliases are designed to simplify common Git operations and provide more informative output.

## Installation

To use these aliases, you need to include them in your global Git configuration file.

1.  **Locate your `.gitconfig` file:**
    *   On Windows, it's typically located at `C:\Users\<your_username>\.gitconfig`.
    *   On macOS and Linux, it's usually `~/.gitconfig`.

2.  **Add the following lines to your `.gitconfig` file:**

    ```gitconfig
    [include]
        path = <path_to_this_repository>/.gitconfigalias
    ```

    **Replace `<path_to_this_repository>` with the actual absolute path to the directory containing the `.gitconfigalias` file.**

    For example, if you cloned this repository to `d:/Development/gitflowhelper/githelper/`, the line would be:

    ```gitconfig
    [include]
        path = d:/Development/gitflowhelper/githelper/.gitconfigalias
    ```

3.  **Save the `.gitconfig` file.**

Now, you can use the defined Git aliases in your terminal.

## Aliases

Here's a detailed list of the available Git aliases with their descriptions and parameters:

### Log and History Aliases

*   `ls`
    *   **Description:** Shows the Git log with entries from all branches, including the stash, ordered by date.
    *   **Parameters:** None.
*   `l`
    *   **Description:** Displays a concise log of the last 5 commits, including a graph representation.
    *   **Parameters:** None.
*   `ll`
    *   **Description:** Displays a more detailed log of the last 15 commits, including a graph representation.
    *   **Parameters:** None.
*   `lf`
    *   **Description:** Shows the last 5 commits with their modified files listed.
    *   **Parameters:** None.
*   `lfb`
    *   **Description:** A beautifully formatted log of the last 5 commits, showing commit hash, author, relative date, subject, and decorations.
    *   **Parameters:** None.
*   `lb`
    *   **Description:** Similar to `lfb`, but without showing file changes.
    *   **Parameters:** None.
*   `lbf`
    *   **Description:** A formatted log of the last 5 commits, showing commit hash, author, relative date, subject, and decorations (without the graph).
    *   **Parameters:** None.
*   `ln`
    *   **Description:** Shows the last 5 commits with a graph, excluding merge commits.
    *   **Parameters:** None.
*   `lsha`
    *   **Description:** Displays only the abbreviated commit hashes for all commits.
    *   **Parameters:** None.
*   `lv`
    *   **Description:** Shows a diff of the last 3 commits (excluding merges) and opens it in VS Code.
    *   **Parameters:**
        *   `$1`, `$2`, ...: Commit references or branches to compare (up to 7).
*   `lbr`
    *   **Description:** Fetches all remote branches and lists the latest commit for each, sorted by date.
    *   **Parameters:**
        *   `$1` (optional): Number of latest remote branches to display.
*   `lbv`
    *   **Description:** Combines `git lb` (last 5 commits) with an optional branch filter.
    *   **Parameters:**
        *   `$1`: Branch name.
        *   `$2` (optional): Number of commits to show (default is 2).
*   `lg`
    *   **Description:** A comprehensive, colorized, and paginated log with graph, author, and relative date, excluding tags.
    *   **Parameters:** None.
*   `lgt`
    *   **Description:** Similar to `lg`, but specifically focuses on tags.
    *   **Parameters:** None.
*   `lgv`
    *   **Description:** A detailed, colorized log showing branches, oneline format, with graph and decorations, excluding tags.
    *   **Parameters:** None.
*   `lgvt`
    *   **Description:** Similar to `lgv`, but without excluding tags.
    *   **Parameters:** None.
*   `lgvv`
    *   **Description:** An enhanced, colorized log with graph, decorations, author, and relative date for all branches, excluding tags.
    *   **Parameters:** None.
*   `lgvvt`
    *   **Description:** Similar to `lgvv`, but without excluding tags.
    *   **Parameters:** None.
*   `ls` (Submodule log)
    *   **Description:** Shows a log of commits, including submodule changes, for the last 5 commits.
    *   **Parameters:**
        *   `$1`, `$2`, ...: Commit references or branches (up to 9).
*   `lfind`
    *   **Description:** A flexible log command for finding commits with a custom, colored format.
    *   **Parameters:**
        *   `$1`, `$2`, ...: Commit references or branches (up to 9).
*   `search`
    *   **Description:** Finds commits where a given string (regex) is present in the diff, showing matching lines. Uses `-w --text -G`.
    *   **Parameters:**
        *   `$1`: The string (regex) to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `searchi`
    *   **Description:** Similar to `search`, but case-insensitive. Uses `-w --regexp-ignore-case --text -G`.
    *   **Parameters:**
        *   `$1`: The string (regex) to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `searchv`
    *   **Description:** Finds commits where a given string (regex) is present in the diff and shows the matching lines and files. Uses `-w -p --name-only --regexp-ignore-case --text -G`.
    *   **Parameters:**
        *   `$1`: The string (regex) to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `searchvi`
    *   **Description:** Similar to `searchv`, but case-sensitive. Uses `-w -p --name-only --text -G`.
    *   **Parameters:**
        *   `$1`: The string (regex) to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `searchvv`
    *   **Description:** Finds commits with a specific string in the diff and filters the output to show the commit hash and the matching diff line. Uses `-w -p --text -G`.
    *   **Parameters:**
        *   `$1`: The string (regex) to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `searchvvi`
    *   **Description:** Similar to `searchvv`, but case-insensitive. Uses `-w --regexp-ignore-case -p --text -G`.
    *   **Parameters:**
        *   `$1`: The string (regex) to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `searchvvv`
    *   **Description:** Finds commits where a given string (regex) is present in the diff and shows the diff. Uses `-w -p --text -G`.
    *   **Parameters:**
        *   `$1`: The string (regex) to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `searchvvvi`
    *   **Description:** Similar to `searchvvv`, but case-insensitive. Uses `-w -p --regexp-ignore-case --text -G`.
    *   **Parameters:**
        *   `$1`: The string (regex) to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.

### Branch and Commit Aliases

*   `cloner`
    *   **Description:** Clones a repository recursively, including submodules.
    *   **Parameters:** `<repository_url>`
*   `sub`
    *   **Description:** Executes a command for each submodule.
    *   **Parameters:** `<command>` (e.g., `git sub update`)
*   `co`
    *   **Description:** Checks out a branch or commit, recursively including submodules.
    *   **Parameters:** `<branch_or_commit>`
*   `ci`
    *   **Description:** Commits staged changes.
    *   **Parameters:** `-m "<commit_message>"`
*   `ciam`
    *   **Description:** Commits staged changes with a message, adding all modified and deleted files.
    *   **Parameters:** `-m "<commit_message>"`
*   `st`
    *   **Description:** Shows the status of the repository, including untracked files, and highlights branches not connected to remote origin and branches with ahead/behind commits.
    *   **Parameters:** None.
*   `sta`
    *   **Description:** Similar to `st`, but also lists untracked files.
    *   **Parameters:** None.
*   `staged`
    *   **Description:** Shows a list of staged files.
    *   **Parameters:** None.
*   `br`
    *   **Description:** Lists all local branches, sorted by committer date in descending order.
    *   **Parameters:** None.
*   `brs`
    *   **Description:** Lists a specified number of the latest local branches, sorted by committer date.
    *   **Parameters:**
        *   `$1` (optional): Number of branches to display (defaults to 5).
        *   `$@`: Additional arguments to pass to `git branch`.
*   `hide`
    *   **Description:** Marks files as "assume unchanged" to temporarily ignore them.
    *   **Parameters:** `<file1> <file2> ...`
*   `unhide`
    *   **Description:** Removes the "assume unchanged" flag from files.
    *   **Parameters:** `<file1> <file2> ...`
*   `hidechanged`
    *   **Description:** Marks modified files as "assume unchanged".
    *   **Parameters:** None.
*   `hidden`
    *   **Description:** Lists all files marked as "assume unchanged".
    *   **Parameters:** None.
*   `cohidden`
    *   **Description:** Checks out all files marked as "assume unchanged" (effectively reverting them).
    *   **Parameters:** None.
*   `unhideall`
    *   **Description:** Unmarks all files marked as "assume unchanged".
    *   **Parameters:** None.
*   `hideModified`
    *   **Description:** Marks files that have been modified and are not staged as "assume unchanged".
    *   **Parameters:** None.
*   `addp`
    *   **Description:** Interactively stages parts of modified files.
    *   **Parameters:** None.
*   `stagediff`
    *   **Description:** Stages all modified files and then shows the status.
    *   **Parameters:** None.
*   `reset0diff`
    *   **Description:** Stages all modified files, unstages them, and then shows the status.
    *   **Parameters:** None.
*   `stdiff`
    *   **Description:** Shows a list of modified files that would be staged by `git diff -G"."`.
    *   **Parameters:** None.
*   `unstage`
    *   **Description:** Unstages files from the index.
    *   **Parameters:** `<file1> <file2> ...`
*   `resetfdx`
    *   **Description:** Resets the working directory, cleans untracked files (including ignored ones), and shows the status. Excludes `.url` and `.cmd` files.
    *   **Parameters:** None.
*   `resetfd`
    *   **Description:** Resets the working directory, cleans untracked files, and shows the status.
    *   **Parameters:** None.
*   `b`
    *   **Description:** Lists local and remote branches, sorted by author date.
    *   **Parameters:** None.
*   `deleteAllTrackedFiles`
    *   **Description:** Deletes all tracked files in the repository. **Use with extreme caution!**
    *   **Parameters:** None.
*   `findDangelingCommits`
    *   **Description:** Finds and lists unreachable or dangling commits.
    *   **Parameters:** None.
*   `graph`
    *   **Description:** Opens TortoiseGit's revision graph in a minimized window.
    *   **Parameters:** None.
*   `bbehind`
    *   **Description:** Lists local branches that are behind their remote counterparts.
    *   **Parameters:** None.
*   `bahead`
    *   **Description:** Lists local branches that are ahead of their remote counterparts.
    *   **Parameters:** None.
*   `ffAll`
    *   **Description:** Fetches all changes, attempts to fast-forward all branches, and then checks out the original branch.
    *   **Parameters:** None.
*   `ffAllForce`
    *   **Description:** Fetches all changes, attempts to fast-forward all branches with force, stashing changes if the repository is not clean.
    *   **Parameters:** None.
*   `trackAll`
    *   **Description:** Configures all remote branches to be tracked locally.
    *   **Parameters:** None.
*   `sync`
    *   **Description:** Pulls changes, tracks all remote branches, force fast-forwards all branches, and shows the verbose branch status.
    *   **Parameters:** None.
*   `fix`
    *   **Description:** Interactively stages changes and creates a `fixup!` commit for the latest commit matching the pattern.
    *   **Parameters:** `$1`, `$2`, ...: Optional files to add.
*   `cia`
    *   **Description:** Adds all staged changes, generates a conventional commit message with emojis, and commits.
    *   **Parameters:** `$@`: Files to add.
*   `udiff`
    *   **Description:** Shows the differences between the current branch and its upstream counterpart.
    *   **Parameters:** None.
*   `list`
    *   **Description:** Lists all Git configurations, including scope and origin.
    *   **Parameters:** None.
*   `delgonebr`
    *   **Description:** Deletes local branches that have been removed from their remote counterparts.
    *   **Parameters:** None.
*   `pushfwl`
    *   **Description:** Pushes the current branch to its upstream, using `force-with-lease`.
    *   **Parameters:** None.
*   `branchdiff`
    *   **Description:** Shows the differences between two branches in terms of commits.
    *   **Parameters:**
        *   `$1`: The first branch.
        *   `$2`: The second branch.
*   `fixup`
    *   **Description:** Creates a `fixup!` commit for the latest commit.
    *   **Parameters:** None.
*   `mp`
    *   **Description:** Alias for `git st` (deprecated).
    *   **Parameters:** None.
*   `remergefile`
    *   **Description:** Checks out a file with conflict resolution set to `diff3`.
    *   **Parameters:** `<file>`
*   `sc`
    *   **Description:** Shows the commit diff for a given commit using `kdiff3`.
    *   **Parameters:** `$1`: Commit hash or reference.
*   `scc`
    *   **Description:** Shows the commit diff using the default diff tool.
    *   **Parameters:** `$1`: Commit hash or reference.
*   `scc1`, `scc2`, `scc3`
    *   **Description:** Shows the commit diff using different word-diff regex patterns.
    *   **Parameters:** `$1`: Commit hash or reference.
*   `vc`
    *   **Description:** Opens the diff of a commit in VS Code.
    *   **Parameters:**
        *   `$1`: The commit hash or reference.
        *   `$2`, `$3`, ...: Additional Git diff parameters.
*   `vc2`
    *   **Description:** Similar to `vc`, but uses a different diff output file.
    *   **Parameters:**
        *   `$1`: The commit hash or reference.
        *   `$2`, `$3`, ...: Additional Git diff parameters.
*   `vdmrinit`
    *   **Description:** Initializes a merge branch for `git vdmr`. Prompts for the merge branch name.
    *   **Parameters:** None.
*   `vdmr`
    *   **Description:** Diffs the current branch against a configured merge branch and opens it in VS Code.
    *   **Parameters:** `$1`, `$2`, ...: Additional Git diff parameters.
*   `vd`
    *   **Description:** Opens a diff in VS Code.
    *   **Parameters:** `$1`, `$2`, ...: Commit references or branches to compare.
*   `vd2`
    *   **Description:** Similar to `vd`, but uses a different diff output file.
    *   **Parameters:** `$1`, `$2`, ...: Commit references or branches to compare.
*   `dt`
    *   **Description:** Uses the default configured difftool.
    *   **Parameters:** None.
*   `dtd`
    *   **Description:** Uses the default configured difftool with directory diff mode.
    *   **Parameters:** None.
*   `dtc`
    *   **Description:** Uses VS Code as the difftool.
    *   **Parameters:** None.
*   `dtcd`
    *   **Description:** Uses VS Code as the difftool with directory diff mode.
    *   **Parameters:** None.
*   `dtg`
    *   **Description:** Generates a GitLab compare URL for two commit hashes and copies it to the clipboard.
    *   **Parameters:**
        *   `$1`: The first commit hash.
        *   `$2` (optional): The second commit hash. If not provided, HEAD is used for the second hash.
*   `rl`
    *   **Description:** Shows the Git reflog.
    *   **Parameters:** None.
*   `subget`
    *   **Description:** Updates and initializes submodules.
    *   **Parameters:** None.
*   `deleteUnreachable`
    *   **Description:** Cleans up unreachable objects, prunes the repository aggressively, and garbage collects.
    *   **Parameters:** None.
*   `uniqueCommits`
    *   **Description:** Lists commits that are present in all branches but not in the history leading up to HEAD (excluding build server commits).
    *   **Parameters:** None.
*   `alias`
    *   **Description:** Lists all Git aliases, optionally filtered by a keyword.
    *   **Parameters:** `<keyword>` (optional)
*   `flowhelp`
    *   **Description:** Opens the Gitflow AVH wiki in a browser.
    *   **Parameters:** None.
*   `diffcommit`
    *   **Description:** Shows the diff of a commit using `kdiff3`.
    *   **Parameters:** `$1`: Commit hash or reference.
*   `mrd`
    *   **Description:** Diffs the current branch against another branch (or its merge base) and opens the diff in VS Code.
    *   **Parameters:**
        *   `$1`: The branch to compare against.
        *   `$2` (optional): The merge base branch.
        *   `$3`, `$4`, ...: Additional Git diff parameters.
*   `mergedevelop`
    *   **Description:** Merges a specified branch into the current branch, then into develop, and then fast-forwards the specified branch into develop.
    *   **Parameters:** `$1`: The branch to merge.
*   `mergepushdevelop`
    *   **Description:** Merges and pushes the develop branch and the specified branch.
    *   **Parameters:** `$1`: The branch to merge.
*   `md`
    *   **Description:** Merges a specified default branch into the current branch and then into develop, prompting for confirmation.
    *   **Parameters:** `$1` (optional): The default branch name.
*   `mpd`
    *   **Description:** Merges and pushes a specified default branch into the current branch and then into develop, prompting for confirmation.
    *   **Parameters:** `$1` (optional): The default branch name.
*   `ut`
    *   **Description:** Resolves merge conflicts by accepting "theirs" and staging the file.
    *   **Parameters:** `<file>`
*   `uo`
    *   **Description:** Resolves merge conflicts by accepting "ours" and staging the file.
    *   **Parameters:** `<file>`
*   `contains`
    *   **Description:** Shows commits, branches, and tags that contain a given commit.
    *   **Parameters:** `$1`: Commit hash or reference.
*   `find`
    *   **Description:** Finds commits that match a given pattern, optionally using case-insensitivity and other Git log parameters.
    *   **Parameters:**
        *   `$1`: The pattern to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `findv`
    *   **Description:** Similar to `find`, but also shows the diff and file names.
    *   **Parameters:**
        *   `$1`: The pattern to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `findvv`
    *   **Description:** Similar to `find`, but also shows the full diff.
    *   **Parameters:**
        *   `$1`: The pattern to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `findc`
    *   **Description:** Finds commits matching a pattern and then lists all branches, tags, and commits containing those found commits.
    *   **Parameters:**
        *   `$1`: The pattern to search for.
        *   `$2`, `$3`, ...: Additional Git log parameters.
*   `unreleasedCommits`
    *   **Description:** Shows commits since the latest tag.
    *   **Parameters:** None.
*   `releases`
    *   **Description:** Lists all tags containing the prefix "Thi".
    *   **Parameters:** None.
*   `showUnreachableCommits`
    *   **Description:** Shows unreachable commits with a stat summary.
    *   **Parameters:** None.
*   `applyCommitToWorkingDirectory`
    *   **Description:** Applies the diff of a specific commit to the working directory.
    *   **Parameters:** `$1`: Commit hash or reference.
*   `isClean`
    *   **Description:** Checks if the repository is clean (no uncommitted changes). Aborts if not clean.
    *   **Parameters:** None.
*   `deleteBranchBoth`
    *   **Description:** Deletes a branch locally and from the remote origin.
    *   **Parameters:**
        *   `$1`: The branch name.
        *   `$2`: The remote name (usually `origin`).
*   `dbb`
    *   **Description:** Alias for `git deleteBranchBoth`.
    *   **Parameters:**
        *   `$1`: The branch name.
        *   `$2`: The remote name (usually `origin`).

### Feature Branch Workflow Aliases

*   `openfeat`
    *   **Description:** Creates a new feature branch with the prefix `feature/` and sets it to upstream.
    *   **Parameters:** `$1`: The name of the feature.
*   `closefeat`
    *   **Description:** Rebases the current branch onto a specified parent branch, prompting for confirmation.
    *   **Parameters:** `$1`: The parent branch name.
*   `finishclosefeat`
    *   **Description:** Merges a temporary rebase branch and cleans up.
    *   **Parameters:** `$1`: The target branch.
*   `finishfeat`
    *   **Description:** Deletes a feature branch from both local and remote.
    *   **Parameters:** `$1`: The feature branch name.

### Other Aliases

*   `lc`
    *   **Description:** Shows the log of changes since the last commit (ORIG_HEAD) without merges.
    *   **Parameters:** None.
*   `lp`
    *   **Description:** Shows the log with patch and stat information.
    *   **Parameters:** None.
*   `mt`
    *   **Description:** Starts the Git mergetool.
    *   **Parameters:** None.
*   `serve`
    *   **Description:** Starts a Git daemon for serving the repository.
    *   **Parameters:** None.
*   `stm`
    *   **Description:** Shows the status of the repository, excluding untracked files.
    *   **Parameters:** None.
*   `stfu`
    *   **Description:** Alias for `git stm`.
    *   **Parameters:** None.
*   `gg`
    *   **Description:** Fetches all tags, prunes remote branches, and shows the status.
    *   **Parameters:** None.
*   `prunebr`
    *   **Description:** Fetches and prunes remote branches, then deletes local branches that no longer exist remotely.
    *   **Parameters:** None.
*   `generatesAliases`
    *   **Description:** Generates Git aliases from the current `alias` configuration.
    *   **Parameters:** None.
*   `templateFunction`
    *   **Description:** A placeholder for a template function (currently just runs `git l`).
    *   **Parameters:** None.
*   `parent`
    *   **Description:** Shows the parent commit of the current branch.
    *   **Parameters:** None.
*   `storeDevCorrespondingSupportBranch`
    *   **Description:** Configures the `devbranch.name` and `build` symbolic ref to point to the corresponding support branch.
    *   **Parameters:** None.
*   `build-symbolic-ref`
    *   **Description:** Displays the content of the `.git/build` symbolic ref.
    *   **Parameters:** None.
*   `exclude`
    *   **Description:** Adds specified patterns to the `.git/info/exclude` file, ensuring uniqueness.
    *   **Parameters:** `<pattern1> <pattern2> ...`
*   `excludeUnknown`
    *   **Description:** Adds untracked files (excluding standard ones) to `.git/info/exclude`.
    *   **Parameters:** `$1`: Optional path to search within.
*   `excluded`
    *   **Description:** Displays the contents of `.git/info/exclude` and then shows the repository status.
    *   **Parameters:** None.
*   `includeall`
    *   **Description:** Removes the `.git/info/exclude` file.
    *   **Parameters:** None.
*   `root`
    *   **Description:** Shows the root directory of the Git repository.
    *   **Parameters:** None.
*   `brdesc`
    *   **Description:** Executes the `git-branch-desc.exe` command (assumes it's in your PATH).
    *   **Parameters:** `$@`: Arguments to pass to `git-branch-desc.exe`.
*   `brdesclist`
    *   **Description:** Lists the contents of `BRANCHREADME.md` from all branches.
    *   **Parameters:** None.
*   `ignore`
    *   **Description:** Adds specified patterns to the `.gitignore` file.
    *   **Parameters:** `<pattern1> <pattern2> ...`
*   `ignoreUnknown`
    *   **Description:** Adds untracked files (excluding standard ones) to `.gitignore`.
    *   **Parameters:** `$1`: Optional path to search within.
*   `theirs`
    *   **Description:** Checks out the "theirs" version of specified files and stages them.
    *   **Parameters:** `<file1> <file2> ...`
*   `ours`
    *   **Description:** Checks out the "ours" version of specified files and stages them.
    *   **Parameters:** `<file1> <file2> ...`
*   `curl`
    *   **Description:** Gets the URL of the current commit on GitLab and copies it to the clipboard.
    *   **Parameters:** `$1` (optional): Commit hash. If not provided, HEAD is used.
*   `gl`
    *   **Description:** Gets a GitLab URL for a file or topic and copies it to the clipboard.
    *   **Parameters:**
        *   `$1` (optional): The topic (e.g., `blob/main/README.md`, `network`). If not provided, `blob/$branch/README.md` is used.
*   `jk`
    *   **Description:** Gets a Jenkins search URL for the repository and copies it to the clipboard.
    *   **Parameters:** None.
*   `nx`
    *   **Description:** Gets a Jenkins search URL (different format) for the repository and copies it to the clipboard.
    *   **Parameters:** None.
*   `d`
    *   **Description:** Shows word-level differences with a unified context of 0.
    *   **Parameters:** `<file1> <file2>`
*   `d1`
    *   **Description:** Shows word-level differences with a custom regex and a unified context of 0.
    *   **Parameters:** `<file1> <file2>`
*   `d2`
    *   **Description:** Shows word-level differences using `\w` regex and a unified context of 0.
    *   **Parameters:** `<file1> <file2>`
*   `d3`
    *   **Description:** Shows character-level differences with a unified context of 0.
    *   **Parameters:** `<file1> <file2>`
*   `flowhelp`
    *   **Description:** Opens the Gitflow AVH wiki in a browser.
    *   **Parameters:** None.
*   `aco`
    *   **Description:** Adds a file and commits it with a message.
    *   **Parameters:**
        *   `$1`: The file to add.
        *   `$2`: The commit message.
*   `snap`
    *   **Description:** Commits all changes with the message "snap!".
    *   **Parameters:** None.
*   `xbit`
    *   **Description:** Shows files with executable permissions.
    *   **Parameters:** `$1`: Optional file path.
*   `setxbit`
    *   **Description:** Sets executable permissions on specified files.
    *   **Parameters:** `<file1> <file2> ...`

---

Feel free to contribute more aliases or suggest improvements!
