# Git Branch Description Manager - Test Suite

This directory contains comprehensive tests for the `git-branch-desc` tool. The test suite is designed to ensure reliability, correctness, and maintainability of the codebase.

## Test Structure

### Unit Tests (`src/lib.rs`)
- **Location**: Embedded in `src/lib.rs` using `#[cfg(test)]`
- **Purpose**: Test individual functions and components in isolation
- **Coverage**: Core functionality, utility functions, parsing, and data structures

### Integration Tests (`tests/integration_tests.rs`)
- **Purpose**: Test complete workflows and interactions between components
- **Features**:
  - Real Git repository operations
  - Branch creation and management
  - File operations and commits
  - Cross-branch operations
  - Error handling scenarios

### Mock Tests (`tests/mock_tests.rs`)
- **Purpose**: Test hard-to-test functionality using mocks and simulations
- **Features**:
  - Edge cases for parsing functions
  - Error condition testing
  - Boundary condition validation
  - Clipboard and stdin simulation

## Test Categories

### 🧪 Core Functionality Tests
- Git repository operations
- Branch validation and listing
- Description reading/writing
- Commit and push operations
- Current vs. remote branch handling

### 📝 Parsing and Validation Tests
- Issue reference parsing (numbers and URLs)
- JSON parsing from GitLab CLI output
- AI response cleaning and formatting
- Text wrapping and terminal width handling

### 🔒 Error Handling Tests
- Invalid repository paths
- Non-existent branches
- Malformed input data
- Network timeouts (AI features)
- Permission and access errors

### 🤖 AI Integration Tests (Optional)
- Content summarization
- Timeout handling
- Model availability checks
- Response validation

### 🎯 Edge Case Tests
- Very long content
- Unicode characters
- Empty inputs
- Boundary conditions
- Cross-platform compatibility

## Running Tests

### Prerequisites
1. **Rust toolchain** (stable or beta)
2. **Git** installed and configured
3. **Repository context** (run from project root)
4. **Ollama** (optional, for AI tests)

### Test Scripts

#### Windows (PowerShell)
```powershell
# Run all tests
.\test.ps1 -All

# Run specific test types
.\test.ps1 -Unit
.\test.ps1 -Integration
.\test.ps1 -Coverage

# Run with filters
.\test.ps1 -Filter "parse_issue"
.\test.ps1 -Unit -Verbose
```

#### Unix/Linux/macOS (Bash)
```bash
# Run all tests
./test.sh --all

# Run specific test types
./test.sh --unit
./test.sh --integration
./test.sh --coverage

# Run with filters
./test.sh --filter "parse_issue"
./test.sh --unit --verbose
```

### Manual Cargo Commands

#### Unit Tests Only
```bash
cargo test --lib
```

#### Integration Tests Only
```bash
cargo test --test integration_tests --test mock_tests
```

#### All Tests with Coverage
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --verbose --out Html --output-dir target/coverage
```

#### AI Tests (requires Ollama)
```bash
cargo test --features ai_tests
```

## Test Data and Fixtures

### Temporary Repositories
Tests create temporary Git repositories using `tempfile::TempDir`:
- Isolated from your actual Git repositories
- Automatically cleaned up after tests
- Pre-configured with test user credentials

### Mock Data
- Sample GitLab issue JSON responses
- Various URL formats for issue references
- Edge case text content for parsing
- AI response examples for cleaning tests

## CI/CD Integration

### GitHub Actions
The test suite runs automatically on:
- **Push** to main branches
- **Pull requests**
- **Multiple platforms**: Ubuntu, Windows, macOS
- **Multiple Rust versions**: Stable and Beta

### Test Matrix
- ✅ **Formatting** (`cargo fmt --check`)
- ✅ **Linting** (`cargo clippy`)
- ✅ **Unit Tests** (all platforms)
- ✅ **Integration Tests** (all platforms)
- ✅ **Security Audit** (`cargo audit`)
- ✅ **Documentation** (`cargo doc`)
- ✅ **MSRV Check** (Minimum Supported Rust Version)
- ⚠️ **AI Tests** (optional, requires Ollama)
- 📊 **Coverage** (Codecov integration)

## Test Configuration

### Serial Execution
Many tests use `#[serial]` attribute to prevent conflicts:
- File system operations
- Git repository state
- Working directory changes
- AI service connections

### Feature Flags
```toml
[features]
ai_tests = []  # Enables AI-dependent tests
```

### Dependencies
```toml
[dev-dependencies]
tempfile = "3.8"      # Temporary directories
serial_test = "3.0"   # Serial test execution
mockito = "1.2"       # HTTP mocking (future use)
```

## Writing New Tests

### Guidelines
1. **Use descriptive test names** that explain what is being tested
2. **Test both success and failure cases**
3. **Use `#[serial]` for tests that modify global state**
4. **Clean up resources** (temporary files, directories)
5. **Mock external dependencies** when possible
6. **Document complex test scenarios**

### Example Test Structure
```rust
#[test]
#[serial]
fn test_feature_with_git_repo() -> Result<()> {
    // Setup
    let test_repo = TestRepo::new()?;
    let original_dir = std::env::current_dir()?;
    test_repo.set_working_directory();
    
    // Test logic
    let result = test_repo.manager.some_operation()?;
    
    // Assertions
    assert_eq!(result.expected_field, "expected_value");
    
    // Cleanup
    std::env::set_current_dir(original_dir)?;
    Ok(())
}
```

### Integration Test Helper
```rust
struct TestRepo {
    _temp_dir: TempDir,
    repo_path: String,
    manager: GitBranchDescManager,
}

impl TestRepo {
    fn new() -> Result<Self> { /* ... */ }
    fn create_branch(&self, name: &str) -> Result<()> { /* ... */ }
    fn set_working_directory(&self) { /* ... */ }
}
```

## Troubleshooting Tests

### Common Issues

#### Test Failures in CI
- **Permission errors**: Ensure proper Git configuration
- **Timeout issues**: Increase timeouts for slow operations
- **Platform differences**: Use cross-platform paths and commands

#### Local Test Issues
- **Working directory**: Tests may change `cwd`, ensure proper cleanup
- **Git configuration**: Tests require `user.name` and `user.email`
- **Cleanup failures**: Temporary directories should auto-cleanup

#### AI Test Issues
- **Ollama not running**: AI tests require local Ollama service
- **Model not available**: Ensure `llama3.2:1b` model is pulled
- **Network timeouts**: Adjust timeout values for slow systems

### Debug Tips
1. **Run tests individually**: `cargo test test_name`
2. **Enable verbose output**: `cargo test -- --verbose`
3. **Check test isolation**: Use `cargo test -- --test-threads=1`
4. **Inspect temporary files**: Add `println!` to see temp directory paths

## Performance Considerations

### Test Speed
- **Parallel execution** where safe
- **Minimal Git operations** in tests
- **Efficient temporary directory usage**
- **Mocked external services**

### Resource Usage
- **Memory**: Large test data is generated programmatically
- **Disk**: Temporary repositories are cleaned up automatically
- **Network**: AI tests are optional and skippable

## Coverage Goals

Target coverage metrics:
- **Unit tests**: >90% line coverage
- **Integration tests**: All major workflows
- **Error paths**: All error conditions tested
- **Edge cases**: Boundary conditions covered

Current coverage can be viewed in the generated HTML report:
```bash
cargo tarpaulin --out Html
open target/coverage/tarpaulin-report.html
```

## Contributing

When adding new features:
1. **Write tests first** (TDD approach recommended)
2. **Update test documentation** if adding new test patterns
3. **Ensure CI passes** on all platforms
4. **Add performance tests** for operations that scale with repository size
5. **Consider edge cases** and error conditions

For questions about testing approach or CI failures, refer to the main project documentation or open an issue.