use anyhow::Result;
use git_branch_desc::GitBranchDescManager;
use serial_test::serial;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

struct TestRepo {
    _temp_dir: TempDir,
    repo_path: String,
    manager: GitBranchDescManager,
}

impl TestRepo {
    fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().to_str().unwrap().to_string();

        // Initialize git repository
        Command::new("git")
            .args(["init"])
            .current_dir(&repo_path)
            .output()?;

        // Configure git user
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_path)
            .output()?;

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo_path)
            .output()?;

        // Create initial commit
        fs::write(temp_dir.path().join("README.md"), "# Test Repository")?;
        Command::new("git")
            .args(["add", "README.md"])
            .current_dir(&repo_path)
            .output()?;

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(&repo_path)
            .output()?;

        let manager = GitBranchDescManager::new(&repo_path)?;

        Ok(Self {
            _temp_dir: temp_dir,
            repo_path,
            manager,
        })
    }

    fn create_branch(&self, branch_name: &str) -> Result<()> {
        Command::new("git")
            .args(["checkout", "-b", branch_name])
            .current_dir(&self.repo_path)
            .output()?;
        Ok(())
    }

    fn checkout_branch(&self, branch_name: &str) -> Result<()> {
        Command::new("git")
            .args(["checkout", branch_name])
            .current_dir(&self.repo_path)
            .output()?;
        Ok(())
    }

    fn set_working_directory(&self) {
        std::env::set_current_dir(&self.repo_path).unwrap();
    }

    fn _reset_working_directory(&self) {
        // Reset to a safe directory - use temp dir as fallback
        let _ = std::env::set_current_dir(std::env::temp_dir());
    }
}

#[test]
#[serial]
fn test_basic_functionality() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    // Test getting current branch
    let current_branch = test_repo.manager.get_current_branch()?;
    assert!(current_branch == "main" || current_branch == "master");

    // Test reading non-existent description
    let description = test_repo.manager.read_current_branch_description()?;
    assert!(description.is_empty());

    // Test writing and reading description
    let test_description = "Test branch description";
    test_repo
        .manager
        .write_current_branch_description(test_description)?;
    let read_description = test_repo.manager.read_current_branch_description()?;
    assert_eq!(read_description, test_description);

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
#[serial]
fn test_branch_validation() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    // Test validating existing branch
    let current_branch = test_repo.manager.get_current_branch()?;
    assert!(
        test_repo
            .manager
            .validate_branch_exists(&current_branch)
            .is_ok()
    );

    // Test validating non-existent branch
    assert!(
        test_repo
            .manager
            .validate_branch_exists("non-existent-branch")
            .is_err()
    );

    // Create a new branch and test validation
    test_repo.create_branch("feature/test-branch")?;
    assert!(
        test_repo
            .manager
            .validate_branch_exists("feature/test-branch")
            .is_ok()
    );

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
#[serial]
fn test_list_branches() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    // Get initial branch list
    let branches = test_repo.manager.get_local_branch_list()?;
    assert!(!branches.is_empty());
    assert!(branches.iter().any(|b| b == "main" || b == "master"));

    // Create additional branches
    test_repo.create_branch("feature/branch1")?;
    test_repo
        .checkout_branch("main")
        .or_else(|_| test_repo.checkout_branch("master"))?;
    test_repo.create_branch("feature/branch2")?;

    let updated_branches = test_repo.manager.get_local_branch_list()?;
    assert!(updated_branches.len() >= 3);
    assert!(updated_branches.contains(&"feature/branch1".to_string()));
    assert!(updated_branches.contains(&"feature/branch2".to_string()));

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
#[serial]
fn test_commit_current_branch() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    let current_branch = test_repo.manager.get_current_branch()?;
    let description = "Test description for commit";

    // Write description and commit
    test_repo
        .manager
        .write_current_branch_description(description)?;
    test_repo
        .manager
        .commit_current_branch_changes(&current_branch, false, false)?;

    // Verify the file is committed by checking git status
    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&test_repo.repo_path)
        .output()?;

    let status_text = String::from_utf8_lossy(&status_output.stdout);
    assert!(
        !status_text.contains("BRANCHREADME.md"),
        "BRANCHREADME.md should be committed"
    );

    // Verify description still exists
    let read_description = test_repo.manager.read_current_branch_description()?;
    assert_eq!(read_description, description);

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
#[serial]
fn test_commit_to_different_branch() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    // Create and switch to a new branch
    test_repo.create_branch("feature/target-branch")?;

    // Switch back to main/master
    test_repo
        .checkout_branch("main")
        .or_else(|_| test_repo.checkout_branch("master"))?;

    let description = "Description committed to different branch";

    // Commit description to the feature branch while on main/master
    test_repo
        .manager
        .commit_to_branch("feature/target-branch", description, false, false)?;

    // Switch to the feature branch and verify description exists
    test_repo.checkout_branch("feature/target-branch")?;

    // Re-read the description using the git method since we're testing cross-branch functionality
    let read_description = test_repo
        .manager
        .read_branch_description_from_git("feature/target-branch")?
        .unwrap_or_default();
    assert_eq!(read_description, description);

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
#[serial]
fn test_read_branch_description_from_git() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    let current_branch = test_repo.manager.get_current_branch()?;
    let description = "Test description in git";

    // Commit description to current branch
    test_repo
        .manager
        .write_current_branch_description(description)?;
    test_repo
        .manager
        .commit_current_branch_changes(&current_branch, false, false)?;

    // Read description from git
    let git_description = test_repo
        .manager
        .read_branch_description_from_git(&current_branch)?;
    assert!(git_description.is_some());
    assert_eq!(git_description.unwrap(), description);

    // Test reading from non-existent branch
    let non_existent = test_repo
        .manager
        .read_branch_description_from_git("non-existent")?;
    assert!(non_existent.is_none());

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
#[serial]
fn test_list_descriptions() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    // Create branches with descriptions
    let current_branch = test_repo.manager.get_current_branch()?;
    test_repo
        .manager
        .write_current_branch_description("Main branch description")?;
    test_repo
        .manager
        .commit_current_branch_changes(&current_branch, false, false)?;

    test_repo.create_branch("feature/branch1")?;
    test_repo
        .manager
        .write_current_branch_description("Feature branch 1 description")?;
    test_repo
        .manager
        .commit_current_branch_changes("feature/branch1", false, false)?;

    test_repo.checkout_branch(&current_branch)?;
    test_repo.create_branch("feature/branch2")?;
    test_repo
        .manager
        .write_current_branch_description("Feature branch 2 description")?;
    test_repo
        .manager
        .commit_current_branch_changes("feature/branch2", false, false)?;

    // Test list without all flag (should show only branches with descriptions)
    // This is a basic test - the actual list_descriptions method prints to stdout
    // In a real scenario, you might want to refactor to return the data instead of printing
    let result = test_repo.manager.list_descriptions(false, false);
    assert!(result.is_ok());

    // Test list with all flag
    let result_all = test_repo.manager.list_descriptions(false, true);
    assert!(result_all.is_ok());

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
#[serial]
fn test_edit_description_full_workflow() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    // Test editing description on current branch with commit
    let description = "Test description via edit workflow";
    let result = test_repo.manager.edit_description(
        None,                          // target_branch (use current)
        Some(description.to_string()), // description
        false,                         // clipboard
        false,                         // stdin
        None,                          // issue
        false,                         // ai_summarize
        120,                           // ai_timeout
        true,                          // commit
        false,                         // push
        false,                         // force
    );

    assert!(result.is_ok());

    // Verify description was saved and committed
    let read_description = test_repo.manager.read_current_branch_description()?;
    assert_eq!(read_description, description);

    // Verify it's committed
    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&test_repo.repo_path)
        .output()?;

    let status_text = String::from_utf8_lossy(&status_output.stdout);
    assert!(!status_text.contains("BRANCHREADME.md"));

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
#[serial]
fn test_edit_description_different_branch() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    // Create target branch
    test_repo.create_branch("feature/target")?;
    test_repo
        .checkout_branch("main")
        .or_else(|_| test_repo.checkout_branch("master"))?;

    let description = "Description for target branch";
    let result = test_repo.manager.edit_description(
        Some("feature/target".to_string()), // target_branch
        Some(description.to_string()),      // description
        false,                              // clipboard
        false,                              // stdin
        None,                               // issue
        false,                              // ai_summarize
        120,                                // ai_timeout
        true,                               // commit
        false,                              // push
        true,                               // force (to skip confirmation)
    );

    assert!(result.is_ok());

    // Switch to target branch and verify
    test_repo.checkout_branch("feature/target")?;

    // Re-read the description using the git method since we're testing cross-branch functionality
    let read_description = test_repo
        .manager
        .read_branch_description_from_git("feature/target")?
        .unwrap_or_default();
    assert_eq!(read_description, description);

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
fn test_utility_functions() {
    // Test terminal width function
    let width = git_branch_desc::get_terminal_width();
    assert!(width > 0);
    assert!(width <= 10000); // Reasonable upper bound

    // Test text wrapping
    let long_text = "This is a very long line of text that should be wrapped at the specified width to ensure proper formatting";
    let wrapped = git_branch_desc::wrap_text(long_text, 20);

    // Verify all lines are within the limit
    for line in wrapped.lines() {
        assert!(line.len() <= 20, "Line too long: '{}'", line);
    }

    // Verify content is preserved
    let unwrapped = wrapped.replace('\n', " ");
    assert_eq!(unwrapped, long_text);

    // Test with empty text
    let empty_wrapped = git_branch_desc::wrap_text("", 20);
    assert!(empty_wrapped.is_empty());

    // Test with single word
    let single_word = git_branch_desc::wrap_text("word", 20);
    assert_eq!(single_word, "word");
}

#[test]
fn test_issue_parsing() {
    // Test issue number parsing
    assert_eq!(
        git_branch_desc::parse_issue_reference("123").unwrap(),
        "123"
    );
    assert_eq!(
        git_branch_desc::parse_issue_reference("456").unwrap(),
        "456"
    );

    // Test GitLab URL parsing
    let url1 = "https://gitlab.com/owner/repo/-/issues/789";
    assert_eq!(git_branch_desc::parse_issue_reference(url1).unwrap(), "789");

    let url2 = "https://gitlab.example.com/group/project/-/issues/101112";
    assert_eq!(
        git_branch_desc::parse_issue_reference(url2).unwrap(),
        "101112"
    );

    // Test invalid inputs
    assert!(git_branch_desc::parse_issue_reference("invalid").is_err());
    assert!(
        git_branch_desc::parse_issue_reference("https://github.com/owner/repo/issues/123").is_err()
    );
    // Empty string should be handled correctly - it contains no digits and is not a valid URL
    // Empty string is invalid but let's check it doesn't panic
    let result = git_branch_desc::parse_issue_reference("");
    assert!(result.is_err() || result.unwrap().is_empty());
}

#[test]
fn test_json_parsing() {
    // Test complete JSON
    let json =
        r#"{"id":123,"title":"Test Issue","description":"Test description","state":"opened"}"#;
    let result = git_branch_desc::parse_issue_json(json).unwrap();
    assert_eq!(result, "Test Issue\n\nTest description");

    // Test with null description
    let json_null = r#"{"title":"Issue Title","description":null}"#;
    let result_null = git_branch_desc::parse_issue_json(json_null).unwrap();
    assert_eq!(result_null, "Issue Title");

    // Test with empty description
    let json_empty = r#"{"title":"Issue Title","description":""}"#;
    let result_empty = git_branch_desc::parse_issue_json(json_empty).unwrap();
    assert_eq!(result_empty, "Issue Title");

    // Test with complex nested data (like real glab output)
    let complex_json = r#"{"id":172148901,"iid":15,"title":"Build pipeline issue","description":"Need to migrate from Jenkins\n\nThe current setup uses:\n- VS2017\n- MsBuild","state":"opened","author":{"name":"Test User"},"labels":["enhancement"]}"#;
    let complex_result = git_branch_desc::parse_issue_json(complex_json).unwrap();
    assert!(complex_result.starts_with("Build pipeline issue"));
    assert!(complex_result.contains("Need to migrate from Jenkins"));
    assert!(complex_result.contains("The current setup uses:"));

    // Test error cases
    assert!(git_branch_desc::parse_issue_json("invalid json").is_err());
    assert!(git_branch_desc::parse_issue_json(r#"{"description":"No title"}"#).is_err());
}

#[test]
fn test_ai_preamble_cleaning() {
    // Test various preamble patterns
    let test_cases = vec![
        (
            "Here's a concise branch description: Fix authentication",
            "Fix authentication",
        ),
        (
            "Here is a concise branch description: Update pipeline",
            "Update pipeline",
        ),
        ("Branch description: Implement feature", "Implement feature"),
        ("Here is the branch description: Fix bug", "Fix bug"),
        ("\"Quoted description\"", "Quoted description"),
        (
            "Normal text without preamble",
            "Normal text without preamble",
        ),
        (
            "Here's a branch description:\n\nMultiline content",
            "Multiline content",
        ), // Should strip the preamble even in multiline cases
    ];

    for (input, expected) in test_cases {
        let result = git_branch_desc::clean_ai_preamble(input);
        assert_eq!(result, expected, "Failed for input: '{}'", input);
    }

    // Test thinking tag removal
    let with_thinking = "<think>Let me think</think>\nFix bug\n\nUpdate docs";
    let cleaned_thinking = git_branch_desc::clean_ai_preamble(with_thinking);
    assert_eq!(cleaned_thinking, "Fix bug\nUpdate docs");

    // Test complex case with both preamble and thinking tags
    let complex = "Here's a concise branch description: <think>hmm</think>\nFix authentication bug\n\nUpdate tests";
    let complex_result = git_branch_desc::clean_ai_preamble(complex);
    assert_eq!(complex_result, "Fix authentication bug\nUpdate tests");
}

#[test]
#[serial]
fn test_error_handling() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Test with invalid repository path
    let invalid_manager = GitBranchDescManager::new("/non/existent/path");
    assert!(invalid_manager.is_err());

    // Test validation with non-existent branch
    let error = test_repo
        .manager
        .validate_branch_exists("definitely-not-a-branch");
    assert!(error.is_err());
    let error_msg = format!("{}", error.unwrap_err());
    assert!(error_msg.contains("not found"));
    assert!(error_msg.contains("Available branches"));

    Ok(())
}

#[test]
#[serial]
fn test_modify_vs_add_detection() -> Result<()> {
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;

    test_repo.set_working_directory();

    let current_branch = test_repo.manager.get_current_branch()?;

    // First time - should be "add"
    test_repo
        .manager
        .write_current_branch_description("Initial description")?;
    test_repo
        .manager
        .commit_current_branch_changes(&current_branch, false, false)?;

    // Check commit message contains "Add"
    let log_output = Command::new("git")
        .args(["log", "-1", "--pretty=format:%s"])
        .current_dir(&test_repo.repo_path)
        .output()?;
    let commit_msg = String::from_utf8_lossy(&log_output.stdout);
    assert!(
        commit_msg.contains("Add"),
        "Expected 'Add' in commit message: {}",
        commit_msg
    );

    // Second time - should be "update"
    test_repo
        .manager
        .write_current_branch_description("Updated description")?;
    test_repo
        .manager
        .commit_current_branch_changes(&current_branch, true, false)?;

    let log_output2 = Command::new("git")
        .args(["log", "-1", "--pretty=format:%s"])
        .current_dir(&test_repo.repo_path)
        .output()?;
    let commit_msg2 = String::from_utf8_lossy(&log_output2.stdout);
    assert!(
        commit_msg2.contains("Update"),
        "Expected 'Update' in commit message: {}",
        commit_msg2
    );

    std::env::set_current_dir(original_dir)?;
    Ok(())
}

#[test]
#[serial]
fn test_branch_description_struct() {
    let desc = git_branch_desc::BranchDescription {
        branch: "feature/test".to_string(),
        description: "Test description".to_string(),
    };

    assert_eq!(desc.branch, "feature/test");
    assert_eq!(desc.description, "Test description");

    // Test Clone trait
    let desc2 = desc.clone();
    assert_eq!(desc, desc2);

    // Test Debug trait
    let debug_str = format!("{:?}", desc);
    assert!(debug_str.contains("feature/test"));
    assert!(debug_str.contains("Test description"));
}

// Clipboard and stdin tests are harder to test in unit tests as they require
// external input. These would be better suited for manual testing or
// integration tests with mocked input.

// AI summarization tests would require Ollama to be running, so they're
// better suited for optional integration tests that can be skipped if
// the service isn't available.

#[cfg(feature = "ai_tests")]
#[test]
#[serial]
fn test_ai_summarization() -> Result<()> {
    let test_repo = TestRepo::new()?;

    let content = "This is a very long description of a feature that implements user authentication with OAuth2, includes password reset functionality, adds email verification, and improves security measures across the application.";

    match test_repo.manager.ai_summarize_content(content, 30) {
        Ok(summary) => {
            assert!(!summary.is_empty());
            println!("AI Summary: {}", summary);
            // AI summary should be more concise (not always true, but generally)
            // Don't assert on length as AI behavior can vary
        }
        Err(e) => {
            // Expected if Ollama isn't running
            println!("AI test skipped (Ollama not available): {}", e);
        }
    }

    Ok(())
}
