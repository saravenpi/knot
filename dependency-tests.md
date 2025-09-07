# Knot Inter-Package Dependency Testing Scenarios

This document outlines comprehensive test scenarios for verifying inter-package dependencies functionality in Knot.

## Test Environment Setup

Before running tests, ensure:
1. Knot CLI is installed and available (`knot --version`)
2. Test directory is clean and isolated
3. No cached dependency data that could interfere with tests

## Test Scenarios

### 1. Basic Local Package Dependencies

#### Test 1.1: Simple Local Package Linking
- **Objective**: Verify basic local package linking works correctly
- **Steps**:
  1. Create test project with packages A and B
  2. Configure app to depend on package A
  3. Run `knot link`
  4. Verify package A is linked in `knot_packages/`
  5. Verify TypeScript aliases are configured

#### Test 1.2: Multi-Level Local Dependencies
- **Objective**: Test package A depends on package B, app depends on package A
- **Steps**:
  1. Create packages A and B where A depends on B
  2. Configure app to depend on package A
  3. Run `knot link`
  4. Verify both packages are available to the app
  5. Test import resolution works correctly

### 2. Online Package Dependencies

#### Test 2.1: Simple Online Package Installation
- **Objective**: Verify online packages are downloaded and linked correctly
- **Steps**:
  1. Add online package dependency (e.g., `@hono-modules-loader`)
  2. Run `knot install @test-package`
  3. Run `knot link`
  4. Verify package is downloaded and available

#### Test 2.2: Version-Specific Online Dependencies
- **Objective**: Test specific version installation
- **Steps**:
  1. Install specific version: `knot install @package@1.2.3`
  2. Verify correct version is downloaded
  3. Test different version patterns (`^`, `~`, exact)

### 3. Mixed Local and Online Dependencies

#### Test 3.1: Local Package with Online Dependencies
- **Objective**: Local package A depends on online package B
- **Steps**:
  1. Create local package A that depends on online package B
  2. Configure app to use package A
  3. Run dependency resolution
  4. Verify both packages are available

#### Test 3.2: Online Package with Local Fallback
- **Objective**: Test fallback mechanisms
- **Steps**:
  1. Configure package with online dependency
  2. Simulate network failure
  3. Verify appropriate error handling

### 4. Edge Cases and Error Scenarios

#### Test 4.1: Circular Dependencies
- **Objective**: Test circular dependency detection and handling
- **Steps**:
  1. Create packages A and B that depend on each other
  2. Run dependency resolution
  3. Verify proper error message and graceful failure

#### Test 4.2: Missing Dependencies
- **Objective**: Test missing local package handling
- **Steps**:
  1. Configure app to depend on non-existent local package
  2. Run `knot link`
  3. Verify clear error message

#### Test 4.3: Version Conflicts
- **Objective**: Test version conflict detection
- **Steps**:
  1. Create scenario where different packages require different versions of same dependency
  2. Run dependency resolution
  3. Verify conflict detection and resolution strategy

#### Test 4.4: Invalid Package Specifications
- **Objective**: Test handling of malformed package specs
- **Steps**:
  1. Try various invalid package specifications
  2. Verify appropriate validation errors

### 5. CLI Command Testing

#### Test 5.1: Link Command Variations
- **Objective**: Test all link command options
- **Commands to test**:
  - `knot link` (basic linking)
  - `knot link --symlink` (symlink mode)
  - Link behavior in different directory contexts

#### Test 5.2: Install Command Variations
- **Objective**: Test install command with different options
- **Commands to test**:
  - `knot install package`
  - `knot install package@version`
  - `knot install package --no-link`
  - `knot install @scoped/package`

### 6. Performance and Reliability Tests

#### Test 6.1: Large Dependency Trees
- **Objective**: Test performance with many dependencies
- **Steps**:
  1. Create project with 20+ interdependent packages
  2. Measure linking time
  3. Verify all packages are correctly linked

#### Test 6.2: Concurrent Operations
- **Objective**: Test multiple operations running simultaneously
- **Steps**:
  1. Run multiple `knot link` commands in parallel
  2. Verify no race conditions or corruption

### 7. TypeScript Integration Tests

#### Test 7.1: Alias Resolution
- **Objective**: Verify TypeScript aliases work correctly
- **Steps**:
  1. Set up project with TypeScript configuration
  2. Link packages
  3. Test import statements resolve correctly
  4. Verify IDE autocomplete works

#### Test 7.2: Different Alias Configurations
- **Objective**: Test various alias configurations
- **Steps**:
  1. Test `tsAlias: true` configuration
  2. Test `tsAlias: "#"` configuration
  3. Test `tsAlias: null` configuration

### 8. Symlink vs Copy Mode Testing

#### Test 8.1: Symlink Mode Verification
- **Objective**: Ensure symlink mode works correctly
- **Steps**:
  1. Run `knot link --symlink`
  2. Verify symlinks are created instead of copies
  3. Test that changes in source packages are reflected immediately

#### Test 8.2: Copy Mode Verification
- **Objective**: Ensure copy mode works correctly
- **Steps**:
  1. Run `knot link` (default copy mode)
  2. Verify files are copied, not symlinked
  3. Test isolation between source and linked packages

## Test Infrastructure Requirements

### Automated Test Runner
A test runner script should be created that:
1. Sets up clean test environments
2. Runs all test scenarios
3. Reports results in a structured format
4. Cleans up test artifacts

### Test Data Generation
Scripts to generate:
1. Sample project structures
2. Mock package dependencies
3. Various package.json configurations
4. Different app.yml configurations

### Validation Scripts
Scripts to verify:
1. Package linking correctness
2. TypeScript configuration validity
3. File system state after operations
4. Error message accuracy

## Expected Outcomes

### Success Criteria
- All basic functionality tests pass
- Edge cases are handled gracefully
- Error messages are clear and actionable
- Performance is acceptable for typical use cases
- TypeScript integration works seamlessly

### Documentation of Issues
Any failing tests should document:
1. Specific failure conditions
2. Expected vs actual behavior
3. Potential root causes
4. Suggested fixes or improvements

## Test Report Template

Each test execution should generate a report containing:
1. Test environment details
2. Pass/fail status for each scenario
3. Performance metrics where applicable
4. Error logs and stack traces for failures
5. Recommendations for fixes or improvements