#!/bin/bash
# Test runner script for git-branch-desc

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
UNIT=false
INTEGRATION=false
ALL=false
COVERAGE=false
VERBOSE=false
RELEASE=false
FILTER=""

function print_colored() {
    local message="$1"
    local color="${2:-$NC}"
    echo -e "${color}${message}${NC}"
}

function print_section() {
    local title="$1"
    echo ""
    print_colored "============================================================" "$BLUE"
    print_colored "  $title" "$BLUE"
    print_colored "============================================================" "$BLUE"
    echo ""
}

function show_help() {
    cat << EOF
Git Branch Description Manager - Test Runner

Usage: $0 [OPTIONS]

Options:
    -u, --unit           Run unit tests only
    -i, --integration    Run integration tests only  
    -a, --all            Run all tests (lint, build, unit, integration, AI)
    -c, --coverage       Run tests with coverage report
    -v, --verbose        Enable verbose output
    -r, --release        Run tests in release mode
    -f, --filter <name>  Filter tests by name pattern
    -h, --help          Show this help message

Examples:
    $0                          # Run unit tests
    $0 --all                    # Run comprehensive test suite
    $0 --integration            # Run integration tests only
    $0 --coverage               # Run with coverage
    $0 --unit --verbose         # Run unit tests with verbose output
    $0 --filter "parse_issue"   # Run tests matching pattern

Note: AI tests require Ollama to be running locally with llama3.2:1b model.
EOF
}

function parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -u|--unit)
                UNIT=true
                shift
                ;;
            -i|--integration)
                INTEGRATION=true
                shift
                ;;
            -a|--all)
                ALL=true
                shift
                ;;
            -c|--coverage)
                COVERAGE=true
                shift
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -r|--release)
                RELEASE=true
                shift
                ;;
            -f|--filter)
                FILTER="$2"
                shift 2
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                echo "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

function check_prerequisites() {
    print_section "Checking Prerequisites"
    
    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]]; then
        print_colored "Error: Cargo.toml not found. Please run this script from the project root." "$RED"
        exit 1
    fi
    
    # Check if git is available
    if ! command -v git &> /dev/null; then
        print_colored "✗ Git is not available or not in PATH" "$RED"
        exit 1
    fi
    print_colored "✓ Git is available" "$GREEN"
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        print_colored "✗ Cargo is not available or not in PATH" "$RED"
        exit 1
    fi
    print_colored "✓ Cargo is available" "$GREEN"
    
    # Check if this is a git repository
    if ! git rev-parse --git-dir &> /dev/null; then
        print_colored "✗ Not in a git repository" "$RED"
        exit 1
    fi
    print_colored "✓ Running in a git repository" "$GREEN"
    
    print_colored "All prerequisites met!" "$GREEN"
}

function run_unit_tests() {
    print_section "Running Unit Tests"
    
    local args=("test" "--lib")
    
    if [[ "$RELEASE" == "true" ]]; then
        args+=("--release")
    fi
    
    if [[ "$VERBOSE" == "true" ]]; then
        args+=("--verbose")
    fi
    
    if [[ -n "$FILTER" ]]; then
        args+=("--" "--test-threads" "1" "$FILTER")
    else
        args+=("--" "--test-threads" "1")
    fi
    
    print_colored "Running: cargo ${args[*]}" "$YELLOW"
    
    if cargo "${args[@]}"; then
        print_colored "✓ Unit tests passed!" "$GREEN"
        return 0
    else
        print_colored "✗ Unit tests failed!" "$RED"
        return 1
    fi
}

function run_integration_tests() {
    print_section "Running Integration Tests"
    
    local args=("test" "--test" "integration_tests" "--test" "mock_tests")
    
    if [[ "$RELEASE" == "true" ]]; then
        args+=("--release")
    fi
    
    if [[ "$VERBOSE" == "true" ]]; then
        args+=("--verbose")
    fi
    
    if [[ -n "$FILTER" ]]; then
        args+=("--" "--test-threads" "1" "$FILTER")
    else
        args+=("--" "--test-threads" "1")
    fi
    
    print_colored "Running: cargo ${args[*]}" "$YELLOW"
    
    if cargo "${args[@]}"; then
        print_colored "✓ Integration tests passed!" "$GREEN"
        return 0
    else
        print_colored "✗ Integration tests failed!" "$RED"
        return 1
    fi
}

function run_optional_ai_tests() {
    print_section "Running Optional AI Tests (requires Ollama)"
    
    # Check if Ollama is running
    if curl -s --connect-timeout 5 "http://localhost:11434/api/tags" &> /dev/null; then
        print_colored "✓ Ollama is running, proceeding with AI tests" "$GREEN"
        
        local args=("test" "--features" "ai_tests")
        
        if [[ "$RELEASE" == "true" ]]; then
            args+=("--release")
        fi
        
        if [[ "$VERBOSE" == "true" ]]; then
            args+=("--verbose")
        fi
        
        args+=("--" "--test-threads" "1")
        
        print_colored "Running: cargo ${args[*]}" "$YELLOW"
        
        if cargo "${args[@]}"; then
            print_colored "✓ AI tests passed!" "$GREEN"
            return 0
        else
            print_colored "✗ AI tests failed!" "$RED"
            return 1
        fi
    else
        print_colored "⚠ Ollama not available, skipping AI tests" "$YELLOW"
        print_colored "  To run AI tests:" "$YELLOW"
        print_colored "  1. Install Ollama from https://ollama.ai" "$YELLOW"
        print_colored "  2. Run: ollama run llama3.2:1b" "$YELLOW"
        print_colored "  3. Keep Ollama running and re-run tests" "$YELLOW"
        return 0
    fi
}

function run_coverage_tests() {
    print_section "Running Tests with Coverage"
    
    # Check if tarpaulin is installed
    if ! cargo tarpaulin --version &> /dev/null; then
        print_colored "Installing cargo-tarpaulin..." "$YELLOW"
        cargo install cargo-tarpaulin
    fi
    
    local args=("tarpaulin" "--verbose" "--out" "Html" "--output-dir" "target/coverage")
    
    if [[ -n "$FILTER" ]]; then
        args+=("--" "$FILTER")
    fi
    
    print_colored "Running: cargo ${args[*]}" "$YELLOW"
    
    if cargo "${args[@]}"; then
        print_colored "✓ Coverage tests completed!" "$GREEN"
        if [[ -f "target/coverage/tarpaulin-report.html" ]]; then
            print_colored "Coverage report generated: target/coverage/tarpaulin-report.html" "$BLUE"
        fi
        return 0
    else
        print_colored "✗ Coverage tests failed!" "$RED"
        return 1
    fi
}

function run_lint() {
    print_section "Running Linting and Formatting Checks"
    
    # Check formatting
    print_colored "Checking formatting..." "$YELLOW"
    if ! cargo fmt -- --check; then
        print_colored "✗ Code formatting issues found. Run 'cargo fmt' to fix." "$RED"
        return 1
    fi
    print_colored "✓ Code formatting is correct" "$GREEN"
    
    # Run clippy
    print_colored "Running clippy..." "$YELLOW"
    if ! cargo clippy -- -D warnings; then
        print_colored "✗ Clippy warnings found" "$RED"
        return 1
    fi
    print_colored "✓ No clippy warnings" "$GREEN"
    
    return 0
}

function test_build_and_install() {
    print_section "Testing Build and Install"
    
    # Clean build
    print_colored "Cleaning previous builds..." "$YELLOW"
    cargo clean
    
    # Build in debug mode
    print_colored "Building in debug mode..." "$YELLOW"
    if ! cargo build; then
        print_colored "✗ Debug build failed" "$RED"
        return 1
    fi
    print_colored "✓ Debug build successful" "$GREEN"
    
    # Build in release mode
    print_colored "Building in release mode..." "$YELLOW"
    if ! cargo build --release; then
        print_colored "✗ Release build failed" "$RED"
        return 1
    fi
    print_colored "✓ Release build successful" "$GREEN"
    
    # Test basic functionality
    print_colored "Testing basic functionality..." "$YELLOW"
    if ! ./target/release/git-branch-desc --help &> /dev/null; then
        print_colored "✗ Basic functionality test failed" "$RED"
        return 1
    fi
    print_colored "✓ Basic functionality works" "$GREEN"
    
    return 0
}

function show_test_summary() {
    local -A results=("$@")
    
    print_section "Test Summary"
    
    local passed=0
    local failed=0
    
    for test in "${!results[@]}"; do
        if [[ "${results[$test]}" == "0" ]]; then
            print_colored "✓ $test" "$GREEN"
            ((passed++))
        else
            print_colored "✗ $test" "$RED"
            ((failed++))
        fi
    done
    
    echo ""
    print_colored "Total: $((passed + failed)), Passed: $passed, Failed: $failed" "$BLUE"
    
    if [[ $failed -eq 0 ]]; then
        print_colored "🎉 All tests passed!" "$GREEN"
        return 0
    else
        print_colored "💥 Some tests failed!" "$RED"
        return 1
    fi
}

function main() {
    declare -A results
    
    # Always check prerequisites
    check_prerequisites
    
    # Determine what to run
    if [[ "$ALL" == "true" ]] || ([[ "$UNIT" == "false" ]] && [[ "$INTEGRATION" == "false" ]] && [[ "$COVERAGE" == "false" ]]); then
        if [[ "$ALL" == "true" ]]; then
            print_colored "Running comprehensive test suite..." "$BLUE"
            
            run_lint && results["Lint and Format"]=0 || results["Lint and Format"]=1
            test_build_and_install && results["Build and Install"]=0 || results["Build and Install"]=1
            run_unit_tests && results["Unit Tests"]=0 || results["Unit Tests"]=1
            run_integration_tests && results["Integration Tests"]=0 || results["Integration Tests"]=1
            run_optional_ai_tests && results["AI Tests"]=0 || results["AI Tests"]=1
            
            if [[ "$COVERAGE" == "true" ]]; then
                run_coverage_tests && results["Coverage Tests"]=0 || results["Coverage Tests"]=1
            fi
        else
            # Default: run unit tests only
            run_unit_tests && results["Unit Tests"]=0 || results["Unit Tests"]=1
        fi
    else
        if [[ "$UNIT" == "true" ]]; then
            run_unit_tests && results["Unit Tests"]=0 || results["Unit Tests"]=1
        fi
        
        if [[ "$INTEGRATION" == "true" ]]; then
            run_integration_tests && results["Integration Tests"]=0 || results["Integration Tests"]=1
        fi
        
        if [[ "$COVERAGE" == "true" ]]; then
            run_coverage_tests && results["Coverage Tests"]=0 || results["Coverage Tests"]=1
        fi
    fi
    
    # Show summary
    if ! show_test_summary "$(declare -p results)"; then
        exit 1
    fi
}

# Parse command line arguments
parse_args "$@"

# Show help if no parameters and not in default unit test mode
if [[ $# -eq 0 ]]; then
    main
else
    main
fi