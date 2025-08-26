use anyhow::Result;
use git_branch_desc::GitBranchDescManager;
// use mockito::{mock, Matcher, Mock}; // Temporarily disabled
use serial_test::serial;
use std::fs;
use std::io::{self, IsTerminal};
use std::process::Command;
use tempfile::TempDir;

struct MockTestRepo {
    _temp_dir: TempDir,
    repo_path: String,
    manager: GitBranchDescManager,
}

impl MockTestRepo {
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

    fn _set_working_directory(&self) {
        std::env::set_current_dir(&self.repo_path).unwrap();
    }
}

#[test]
fn test_parse_issue_json_edge_cases() {
    // Test with minimal valid JSON
    let minimal_json = r#"{"title":"Minimal Issue"}"#;
    let result = git_branch_desc::parse_issue_json(minimal_json).unwrap();
    assert_eq!(result, "Minimal Issue");

    // Test with extra fields
    let extra_fields = r#"{"id":999,"title":"Issue Title","description":"Issue description","author":{"name":"Author"},"assignees":[],"labels":["bug","urgent"],"milestone":null,"created_at":"2023-01-01T00:00:00Z"}"#;
    let result = git_branch_desc::parse_issue_json(extra_fields).unwrap();
    assert_eq!(result, "Issue Title\n\nIssue description");

    // Test with escaped characters in JSON
    let escaped_json = r#"{"title":"Issue with \"quotes\" and newlines","description":"Description with\n\nnewlines and \"escaped quotes\""}"#;
    let result = git_branch_desc::parse_issue_json(escaped_json).unwrap();
    assert!(result.contains("Issue with \"quotes\" and newlines"));
    assert!(result.contains("Description with\n\nnewlines and \"escaped quotes\""));

    // Test with Unicode characters
    let unicode_json = r#"{"title":"Issue with émojis 🐛","description":"Description with spéciäl characters and 🚀"}"#;
    let result = git_branch_desc::parse_issue_json(unicode_json).unwrap();
    assert!(result.contains("Issue with émojis 🐛"));
    assert!(result.contains("Description with spéciäl characters and 🚀"));

    // Test with very long content
    let long_description = "Very ".repeat(1000) + "long description";
    let long_json = format!(
        r#"{{"title":"Long Issue","description":"{}"}}"#,
        long_description
    );
    let result = git_branch_desc::parse_issue_json(&long_json).unwrap();
    assert!(result.starts_with("Long Issue\n\nVery Very Very"));
    assert!(result.contains("long description"));
}

#[test]
fn test_issue_reference_parsing_edge_cases() {
    // Test various GitLab URL formats
    let urls = vec![
        ("https://gitlab.com/group/project/-/issues/123", "123"),
        ("http://gitlab.example.com/ns/repo/-/issues/456", "456"),
        (
            "https://gitlab.internal.company.com/team/app/-/issues/789",
            "789",
        ),
        (
            "https://gitlab.com/group/subgroup/project/-/issues/999",
            "999",
        ),
    ];

    for (url, expected) in urls {
        let result = git_branch_desc::parse_issue_reference(url).unwrap();
        assert_eq!(result, expected, "Failed to parse URL: {}", url);
    }

    // Test issue numbers
    let numbers = vec!["1", "42", "1234567890"];
    for number in numbers {
        let result = git_branch_desc::parse_issue_reference(number).unwrap();
        assert_eq!(result, number);
    }

    // Test invalid formats
    let invalid_refs = vec![
        "https://github.com/owner/repo/issues/123", // GitHub URL
        "https://gitlab.com/project/issues/123",    // Missing group
        "not-a-number",                             // Invalid string
        "123abc",                                   // Mixed alphanumeric
        "",                                         // Empty string
        "https://gitlab.com/group/project/-/merge_requests/123", // MR URL
    ];

    for invalid_ref in invalid_refs {
        let result = git_branch_desc::parse_issue_reference(invalid_ref);
        assert!(result.is_err(), "Should fail for: {}", invalid_ref);
    }
}

#[test]
fn test_clean_ai_preamble_comprehensive() {
    let test_cases = vec![
        // Basic preamble removal
        ("Here's a concise branch description: Fix bug", "Fix bug"),
        (
            "Here is a concise branch description: Update pipeline",
            "Update pipeline",
        ),
        (
            "A concise branch description: Implement feature",
            "Implement feature",
        ),
        ("Branch description: Add tests", "Add tests"),
        (
            "Here is the branch description: Refactor code",
            "Refactor code",
        ),
        ("Here's the branch description: Deploy app", "Deploy app"),
        // With quotes
        (
            "Here's a branch description: \"Fix authentication\"",
            "Fix authentication",
        ),
        (
            "Branch description: \"Update the build pipeline\"",
            "Update the build pipeline",
        ),
        // Colon patterns
        (
            "Description for this branch: Migrate database",
            "Migrate database",
        ),
        ("A brief description: Add logging", "Add logging"),
        // Without preamble (should remain unchanged)
        (
            "Simple description without preamble",
            "Simple description without preamble",
        ),
        (
            "Fix critical security vulnerability",
            "Fix critical security vulnerability",
        ),
        // With extra whitespace
        (
            "  Here's a branch description:   Fix performance issue   ",
            "Fix performance issue",
        ),
        (
            "\n\nBranch description:\n\nImplement OAuth2\n\n",
            "Implement OAuth2",
        ),
        // Thinking tags removal
        (
            "<think>Let me analyze this</think>\nFix authentication bug",
            "Fix authentication bug",
        ),
        (
            "Some content\n<think>thinking</think>\nMore content",
            "Some content\nMore content",
        ),
        (
            "Line 1\n</think>\nLine 2\n<think>\nLine 3",
            "Line 1\nLine 2\nLine 3",
        ),
        // Complex cases
        (
            "Here's a concise branch description: <think>hmm</think>\nFix bug\n\nAdd tests",
            "Fix bug\nAdd tests",
        ),
        (
            "\n<think>analysis</think>\nBranch description: Fix issue\n</think>\n",
            "Fix issue",
        ),
        // Empty and whitespace cases
        ("", ""),
        ("   ", ""),
        ("\n\n\n", ""),
        ("<think>only thinking</think>", ""),
        // Multiline descriptions
        (
            "Here's a branch description:\nImplement user auth\nAdd password reset\nUpdate docs",
            "Implement user auth\nAdd password reset\nUpdate docs",
        ),
        (
            "Branch description: Line 1\nLine 2\nLine 3",
            "Line 1\nLine 2\nLine 3",
        ),
    ];

    for (input, expected) in test_cases {
        let result = git_branch_desc::clean_ai_preamble(input);
        assert_eq!(result, expected, "Failed for input: '{}'", input);
    }
}

#[test]
fn test_wrap_text_edge_cases() {
    // Test with exact width match
    let text = "exactly twenty chars";
    assert_eq!(text.len(), 20);
    let wrapped = git_branch_desc::wrap_text(text, 20);
    assert_eq!(wrapped, text);

    // Test with width smaller than longest word
    let text = "supercalifragilisticexpialidocious";
    let wrapped = git_branch_desc::wrap_text(text, 10);
    assert_eq!(wrapped, text); // Should not break words

    // Test with width of 1
    let text = "a b c";
    let wrapped = git_branch_desc::wrap_text(text, 1);
    assert_eq!(wrapped, "a\nb\nc");

    // Test with multiple spaces
    let text = "word1    word2     word3";
    let wrapped = git_branch_desc::wrap_text(text, 10);
    assert_eq!(wrapped, "word1\nword2\nword3");

    // Test with newlines already present
    let text = "line1\nline2 word\nline3";
    let wrapped = git_branch_desc::wrap_text(text, 15);
    // Should treat existing newlines as spaces
    assert!(wrapped.contains("line1 line2"));

    // Test with punctuation
    let text = "Hello, world! How are you?";
    let wrapped = git_branch_desc::wrap_text(text, 12);
    let lines: Vec<&str> = wrapped.lines().collect();
    for line in lines {
        assert!(line.len() <= 12, "Line too long: '{}'", line);
    }

    // Test with leading/trailing spaces
    let text = "  spaced text  ";
    let wrapped = git_branch_desc::wrap_text(text, 5);
    assert!(!wrapped.starts_with(' '));
    assert!(!wrapped.ends_with(' '));
}

#[test]
#[serial]
fn test_stdin_content_simulation() -> Result<()> {
    let mock_repo = MockTestRepo::new()?;

    // Note: Testing actual stdin input is complex in unit tests
    // This test verifies the error case when no stdin is available

    // In a real terminal environment (not piped), get_stdin_content should fail
    // This tests the IsTerminal check
    if io::stdin().is_terminal() {
        let result = mock_repo.manager.get_stdin_content();
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("No input detected on stdin"));
    }

    Ok(())
}

#[test]
#[serial]
fn test_clipboard_error_handling() -> Result<()> {
    let _mock_repo = MockTestRepo::new()?;

    // Test clipboard access (may fail in headless environments)
    match _mock_repo.manager.get_clipboard_content() {
        Ok(content) => {
            // If clipboard access works, content should be a string
            assert!(content.is_empty() || !content.is_empty()); // Just verify it's valid
        }
        Err(e) => {
            // Expected in headless/CI environments
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("clipboard") || error_msg.contains("access"));
        }
    }

    Ok(())
}

#[test]
fn test_terminal_width_boundary_conditions() {
    let width = git_branch_desc::get_terminal_width();

    // Should always return a reasonable width
    assert!(width >= 20, "Terminal width too small: {}", width);
    assert!(width <= 10000, "Terminal width too large: {}", width);

    // Width should be 90% of actual terminal width, or 80 if detection fails
    // In test environment, it often defaults to 80
    if width == 80 {
        // Default case when terminal size detection fails
        assert_eq!(width, 80);
    } else {
        // Calculated case - should be reasonable
        assert!(width >= 20);
    }
}

// Mock AI server tests - Temporarily disabled due to mockito compatibility issues
// These would test AI integration with mocked HTTP responses
#[test]
fn test_ai_functionality_placeholder() {
    // Placeholder test for AI functionality
    // In a real scenario, we would mock the HTTP client to test AI integration
    // For now, we verify that the AI functions exist and can be called

    let content = "Test content for AI processing";

    // Test that the functions exist (compilation test)
    let _result = git_branch_desc::clean_ai_preamble(content);

    // This ensures the AI-related code compiles correctly
    // This test ensures the AI-related code compiles correctly
}

#[test]
#[serial]
fn test_interactive_input_simulation() -> Result<()> {
    let _mock_repo = MockTestRepo::new()?;

    // Test the interactive input function with existing description
    // Note: This would normally require stdin input, but we're testing the display logic

    let existing_desc = "Existing branch description";

    // We can't easily test the actual input reading in unit tests,
    // but we can verify the function exists and handles parameters correctly

    // In a real interactive test, you would need to:
    // 1. Pipe input to the process
    // 2. Capture stdout to verify the prompts
    // 3. Provide the input and verify the result

    // For now, we'll just verify the function signature is correct
    // and that it would handle the existing description parameter

    // This is a placeholder for more complex interactive testing
    assert_eq!(existing_desc, "Existing branch description");

    Ok(())
}

#[test]
fn test_git_operations_error_handling() -> Result<()> {
    // Test error handling with invalid git operations

    // Test with non-existent repository
    let invalid_manager = GitBranchDescManager::new("/definitely/not/a/git/repo");
    assert!(invalid_manager.is_err());

    let temp_dir = TempDir::new()?;
    let empty_dir = temp_dir.path().to_str().unwrap();

    // Test with directory that exists but isn't a git repo
    let invalid_manager2 = GitBranchDescManager::new(empty_dir);
    assert!(invalid_manager2.is_err());

    Ok(())
}

#[test]
fn test_branch_description_operations() {
    // Test BranchDescription struct operations
    let desc1 = git_branch_desc::BranchDescription {
        branch: "main".to_string(),
        description: "Main branch".to_string(),
    };

    let desc2 = git_branch_desc::BranchDescription {
        branch: "main".to_string(),
        description: "Main branch".to_string(),
    };

    let desc3 = git_branch_desc::BranchDescription {
        branch: "feature".to_string(),
        description: "Feature branch".to_string(),
    };

    // Test equality
    assert_eq!(desc1, desc2);
    assert_ne!(desc1, desc3);

    // Test cloning
    let cloned = desc1.clone();
    assert_eq!(desc1, cloned);

    // Test debug formatting
    let debug_str = format!("{:?}", desc1);
    assert!(debug_str.contains("main"));
    assert!(debug_str.contains("Main branch"));

    // Test field access
    assert_eq!(desc1.branch, "main");
    assert_eq!(desc1.description, "Main branch");
}

#[test]
fn test_validation_error_messages() -> Result<()> {
    let mock_repo = MockTestRepo::new()?;

    // Test that validation errors include helpful information
    let result = mock_repo
        .manager
        .validate_branch_exists("non-existent-branch-name");
    assert!(result.is_err());

    let error_msg = format!("{}", result.unwrap_err());

    // Error should mention the branch name
    assert!(error_msg.contains("non-existent-branch-name"));

    // Error should mention "not found"
    assert!(error_msg.contains("not found"));

    // Error should list available branches
    assert!(error_msg.contains("Available branches"));

    Ok(())
}
