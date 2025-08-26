#!/usr/bin/env pwsh
# Test runner script for git-branch-desc

param(
    [switch]$Unit,
    [switch]$Integration,
    [switch]$All,
    [switch]$Coverage,
    [switch]$Verbose,
    [switch]$Release,
    [string]$Filter = ""
)

# Set error action preference
$ErrorActionPreference = "Stop"

# Colors for output
$Green = "`e[32m"
$Red = "`e[31m"
$Yellow = "`e[33m"
$Blue = "`e[34m"
$Reset = "`e[0m"

function Write-ColoredOutput {
    param($Message, $Color = $Reset)
    Write-Host "$Color$Message$Reset"
}

function Write-Section {
    param($Title)
    Write-Host ""
    Write-ColoredOutput "=" * 60 $Blue
    Write-ColoredOutput "  $Title" $Blue
    Write-ColoredOutput "=" * 60 $Blue
    Write-Host ""
}

function Test-Prerequisites {
    Write-Section "Checking Prerequisites"
    
    # Check if we're in the right directory
    if (!(Test-Path "Cargo.toml")) {
        Write-ColoredOutput "Error: Cargo.toml not found. Please run this script from the project root." $Red
        exit 1
    }
    
    # Check if git is available
    try {
        git --version | Out-Null
        Write-ColoredOutput "✓ Git is available" $Green
    }
    catch {
        Write-ColoredOutput "✗ Git is not available or not in PATH" $Red
        exit 1
    }
    
    # Check if cargo is available
    try {
        cargo --version | Out-Null
        Write-ColoredOutput "✓ Cargo is available" $Green
    }
    catch {
        Write-ColoredOutput "✗ Cargo is not available or not in PATH" $Red
        exit 1
    }
    
    # Check if this is a git repository
    try {
        git rev-parse --git-dir | Out-Null
        Write-ColoredOutput "✓ Running in a git repository" $Green
    }
    catch {
        Write-ColoredOutput "✗ Not in a git repository" $Red
        exit 1
    }
    
    Write-ColoredOutput "All prerequisites met!" $Green
}

function Run-UnitTests {
    Write-Section "Running Unit Tests"
    
    $args = @("test", "--lib")
    
    if ($Release) {
        $args += "--release"
    }
    
    if ($Verbose) {
        $args += "--verbose"
    }
    
    if ($Filter) {
        $args += @("--", "--test-threads", "1", $Filter)
    } else {
        $args += @("--", "--test-threads", "1")
    }
    
    Write-ColoredOutput "Running: cargo $($args -join ' ')" $Yellow
    
    $result = & cargo $args
    $exitCode = $LASTEXITCODE
    
    if ($exitCode -eq 0) {
        Write-ColoredOutput "✓ Unit tests passed!" $Green
    } else {
        Write-ColoredOutput "✗ Unit tests failed!" $Red
        return $false
    }
    
    return $true
}

function Run-IntegrationTests {
    Write-Section "Running Integration Tests"
    
    $args = @("test", "--test", "integration_tests", "--test", "mock_tests")
    
    if ($Release) {
        $args += "--release"
    }
    
    if ($Verbose) {
        $args += "--verbose"
    }
    
    if ($Filter) {
        $args += @("--", "--test-threads", "1", $Filter)
    } else {
        $args += @("--", "--test-threads", "1")
    }
    
    Write-ColoredOutput "Running: cargo $($args -join ' ')" $Yellow
    
    $result = & cargo $args
    $exitCode = $LASTEXITCODE
    
    if ($exitCode -eq 0) {
        Write-ColoredOutput "✓ Integration tests passed!" $Green
    } else {
        Write-ColoredOutput "✗ Integration tests failed!" $Red
        return $false
    }
    
    return $true
}

function Run-OptionalAITests {
    Write-Section "Running Optional AI Tests (requires Ollama)"
    
    # Check if Ollama is running
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:11434/api/tags" -TimeoutSec 5 -ErrorAction Stop
        Write-ColoredOutput "✓ Ollama is running, proceeding with AI tests" $Green
        
        $args = @("test", "--features", "ai_tests")
        
        if ($Release) {
            $args += "--release"
        }
        
        if ($Verbose) {
            $args += "--verbose"
        }
        
        $args += @("--", "--test-threads", "1")
        
        Write-ColoredOutput "Running: cargo $($args -join ' ')" $Yellow
        
        $result = & cargo $args
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0) {
            Write-ColoredOutput "✓ AI tests passed!" $Green
        } else {
            Write-ColoredOutput "✗ AI tests failed!" $Red
            return $false
        }
    }
    catch {
        Write-ColoredOutput "⚠ Ollama not available, skipping AI tests" $Yellow
        Write-ColoredOutput "  To run AI tests:" $Yellow
        Write-ColoredOutput "  1. Install Ollama from https://ollama.ai" $Yellow
        Write-ColoredOutput "  2. Run: ollama run llama3.2:1b" $Yellow
        Write-ColoredOutput "  3. Keep Ollama running and re-run tests" $Yellow
    }
    
    return $true
}

function Run-CoverageTests {
    Write-Section "Running Tests with Coverage"
    
    # Check if tarpaulin is installed
    try {
        cargo tarpaulin --version | Out-Null
    }
    catch {
        Write-ColoredOutput "Installing cargo-tarpaulin..." $Yellow
        cargo install cargo-tarpaulin
    }
    
    $args = @("tarpaulin", "--verbose", "--out", "Html", "--output-dir", "target/coverage")
    
    if ($Filter) {
        $args += @("--", $Filter)
    }
    
    Write-ColoredOutput "Running: cargo $($args -join ' ')" $Yellow
    
    $result = & cargo $args
    $exitCode = $LASTEXITCODE
    
    if ($exitCode -eq 0) {
        Write-ColoredOutput "✓ Coverage tests completed!" $Green
        if (Test-Path "target/coverage/tarpaulin-report.html") {
            Write-ColoredOutput "Coverage report generated: target/coverage/tarpaulin-report.html" $Blue
        }
    } else {
        Write-ColoredOutput "✗ Coverage tests failed!" $Red
        return $false
    }
    
    return $true
}

function Run-Lint {
    Write-Section "Running Linting and Formatting Checks"
    
    # Check formatting
    Write-ColoredOutput "Checking formatting..." $Yellow
    $result = & cargo fmt -- --check
    if ($LASTEXITCODE -ne 0) {
        Write-ColoredOutput "✗ Code formatting issues found. Run 'cargo fmt' to fix." $Red
        return $false
    }
    Write-ColoredOutput "✓ Code formatting is correct" $Green
    
    # Run clippy
    Write-ColoredOutput "Running clippy..." $Yellow
    $result = & cargo clippy -- -D warnings
    if ($LASTEXITCODE -ne 0) {
        Write-ColoredOutput "✗ Clippy warnings found" $Red
        return $false
    }
    Write-ColoredOutput "✓ No clippy warnings" $Green
    
    return $true
}

function Test-BuildAndInstall {
    Write-Section "Testing Build and Install"
    
    # Clean build
    Write-ColoredOutput "Cleaning previous builds..." $Yellow
    cargo clean
    
    # Build in debug mode
    Write-ColoredOutput "Building in debug mode..." $Yellow
    $result = & cargo build
    if ($LASTEXITCODE -ne 0) {
        Write-ColoredOutput "✗ Debug build failed" $Red
        return $false
    }
    Write-ColoredOutput "✓ Debug build successful" $Green
    
    # Build in release mode
    Write-ColoredOutput "Building in release mode..." $Yellow
    $result = & cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-ColoredOutput "✗ Release build failed" $Red
        return $false
    }
    Write-ColoredOutput "✓ Release build successful" $Green
    
    # Test basic functionality
    Write-ColoredOutput "Testing basic functionality..." $Yellow
    $result = & .\target\release\git-branch-desc.exe --help
    if ($LASTEXITCODE -ne 0) {
        Write-ColoredOutput "✗ Basic functionality test failed" $Red
        return $false
    }
    Write-ColoredOutput "✓ Basic functionality works" $Green
    
    return $true
}

function Show-TestSummary {
    param($Results)
    
    Write-Section "Test Summary"
    
    $passed = 0
    $failed = 0
    
    foreach ($result in $Results.GetEnumerator()) {
        if ($result.Value) {
            Write-ColoredOutput "✓ $($result.Key)" $Green
            $passed++
        } else {
            Write-ColoredOutput "✗ $($result.Key)" $Red
            $failed++
        }
    }
    
    Write-Host ""
    Write-ColoredOutput "Total: $($passed + $failed), Passed: $passed, Failed: $failed" $Blue
    
    if ($failed -eq 0) {
        Write-ColoredOutput "🎉 All tests passed!" $Green
        return $true
    } else {
        Write-ColoredOutput "💥 Some tests failed!" $Red
        return $false
    }
}

# Main execution
function Main {
    $results = @{}
    
    # Always check prerequisites
    Test-Prerequisites
    
    # Determine what to run
    $runUnit = $Unit -or $All -or (!$Unit -and !$Integration -and !$Coverage)
    $runIntegration = $Integration -or $All
    $runCoverage = $Coverage
    $runAll = $All
    
    if ($runAll) {
        Write-ColoredOutput "Running comprehensive test suite..." $Blue
        
        $results["Lint and Format"] = Run-Lint
        $results["Build and Install"] = Test-BuildAndInstall
        $results["Unit Tests"] = Run-UnitTests
        $results["Integration Tests"] = Run-IntegrationTests
        $results["AI Tests"] = Run-OptionalAITests
        
        if ($Coverage) {
            $results["Coverage Tests"] = Run-CoverageTests
        }
    } else {
        if ($runUnit) {
            $results["Unit Tests"] = Run-UnitTests
        }
        
        if ($runIntegration) {
            $results["Integration Tests"] = Run-IntegrationTests
        }
        
        if ($runCoverage) {
            $results["Coverage Tests"] = Run-CoverageTests
        }
    }
    
    # Show summary
    $success = Show-TestSummary $results
    
    if (!$success) {
        exit 1
    }
}

# Show help if no parameters
if (!$Unit -and !$Integration -and !$All -and !$Coverage -and !$PSBoundParameters.Count) {
    Write-Host @"
Git Branch Description Manager - Test Runner

Usage: .\test.ps1 [OPTIONS]

Options:
    -Unit           Run unit tests only
    -Integration    Run integration tests only  
    -All            Run all tests (lint, build, unit, integration, AI)
    -Coverage       Run tests with coverage report
    -Verbose        Enable verbose output
    -Release        Run tests in release mode
    -Filter <name>  Filter tests by name pattern

Examples:
    .\test.ps1                          # Run unit tests
    .\test.ps1 -All                     # Run comprehensive test suite
    .\test.ps1 -Integration             # Run integration tests only
    .\test.ps1 -Coverage                # Run with coverage
    .\test.ps1 -Unit -Verbose           # Run unit tests with verbose output
    .\test.ps1 -Filter "parse_issue"    # Run tests matching pattern

Note: AI tests require Ollama to be running locally with llama3.2:1b model.
"@
    exit 0
}

# Run main function
Main