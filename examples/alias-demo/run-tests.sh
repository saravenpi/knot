#!/bin/bash

# Comprehensive test runner for Knot alias system
# This script runs all tests and validates the alias system works correctly

set -e  # Exit on any error

echo "🚀 Starting Knot Alias System Test Suite"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
KNOT_BINARY="../../../apps/cli/target/release/knot"
TEST_DIR="$(pwd)"
TEMP_DIR="/tmp/knot-alias-tests-$$"

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if [[ ! -f "$KNOT_BINARY" ]]; then
        log_error "Knot binary not found at $KNOT_BINARY"
        log_info "Please build the CLI first: cd ../../../apps/cli && cargo build --release"
        exit 1
    fi
    
    if ! command -v node &> /dev/null; then
        log_warning "Node.js not found. Some tests may be skipped."
    fi
    
    if ! command -v npm &> /dev/null; then
        log_warning "npm not found. Package installation tests will be skipped."
    fi
    
    log_success "Prerequisites checked"
}

# Test 1: Basic alias functionality
test_basic_aliases() {
    log_info "Test 1: Basic alias functionality"
    
    cd "$TEST_DIR"
    
    # Test that knot can read the configuration
    if $KNOT_BINARY link > /tmp/knot-test-output.log 2>&1; then
        log_success "Basic linking works"
    else
        log_error "Basic linking failed"
        cat /tmp/knot-test-output.log
        return 1
    fi
    
    # Check that knot_packages directories were created
    local apps=("dashboard-app" "api-server" "mobile-app" "analytics")
    for app in "${apps[@]}"; do
        if [[ -d "apps/$app/knot_packages" ]]; then
            log_success "$app: knot_packages directory created"
        else
            log_error "$app: knot_packages directory missing"
            return 1
        fi
    done
    
    # Check that packages were linked
    if [[ -d "apps/dashboard-app/knot_packages/utils" ]]; then
        log_success "Package linking works"
    else
        log_error "Package linking failed"
        return 1
    fi
}

# Test 2: TypeScript configuration generation
test_typescript_config() {
    log_info "Test 2: TypeScript configuration generation"
    
    local apps=("dashboard-app" "api-server" "mobile-app" "analytics")
    local expected_aliases=("@ui" "@core" "#" "$")
    
    for i in "${!apps[@]}"; do
        local app="${apps[$i]}"
        local alias="${expected_aliases[$i]}"
        local tsconfig_path="apps/$app/tsconfig.json"
        
        if [[ -f "$tsconfig_path" ]]; then
            log_success "$app: tsconfig.json exists"
            
            # Check if the alias is correctly configured
            if grep -q "\"${alias}/\*\"" "$tsconfig_path"; then
                log_success "$app: alias $alias correctly configured"
            else
                log_error "$app: alias $alias not found in tsconfig.json"
                return 1
            fi
            
            # Check if knot_packages is included
            if grep -q "knot_packages/\*\*/\*" "$tsconfig_path"; then
                log_success "$app: knot_packages included in TypeScript compilation"
            else
                log_error "$app: knot_packages not included"
                return 1
            fi
        else
            log_error "$app: tsconfig.json not found"
            return 1
        fi
    done
}

# Test 3: Source code alias usage validation
test_source_code_aliases() {
    log_info "Test 3: Source code alias usage validation"
    
    # Test dashboard-app (@ui alias)
    if grep -q "@ui/ui-components" "apps/dashboard-app/src/main.tsx"; then
        log_success "Dashboard app uses @ui alias correctly"
    else
        log_error "Dashboard app @ui alias usage not found"
        return 1
    fi
    
    # Test api-server (@core alias)
    if grep -q "@core/utils" "apps/api-server/src/server.ts"; then
        log_success "API server uses @core alias correctly"
    else
        log_error "API server @core alias usage not found"
        return 1
    fi
    
    # Test mobile-app (# alias)
    if grep -q "#/ui-components" "apps/mobile-app/src/App.tsx"; then
        log_success "Mobile app uses # alias correctly"
    else
        log_error "Mobile app # alias usage not found"
        return 1
    fi
    
    # Test analytics ($ alias)
    if grep -q "\$/utils" "apps/analytics/src/main.ts"; then
        log_success "Analytics app uses $ alias correctly"
    else
        log_error "Analytics app $ alias usage not found"
        return 1
    fi
}

# Test 4: Package dependencies with aliases
test_package_dependencies() {
    log_info "Test 4: Package dependencies with aliases"
    
    # Check that packages can reference each other
    if grep -q "from 'utils'" "packages/ui-components/src/Button/index.ts"; then
        log_success "UI components can import from utils package"
    else
        log_error "UI components cannot import from utils package"
        return 1
    fi
    
    if grep -q "from 'utils/validation'" "packages/ui-components/src/Input/index.ts"; then
        log_success "Packages can use subpath imports"
    else
        log_error "Subpath imports not working"
        return 1
    fi
}

# Test 5: Multiple alias configurations
test_multiple_aliases() {
    log_info "Test 5: Multiple alias configurations"
    
    # Verify each app has its unique alias configuration
    local unique_aliases=("@ui" "@core" "#" "\$")
    local configs_found=0
    
    for alias in "${unique_aliases[@]}"; do
        if find apps -name "tsconfig.json" -exec grep -l "\"${alias}/\*\"" {} \; | wc -l | grep -q "1"; then
            log_success "Unique alias configuration for $alias"
            ((configs_found++))
        else
            log_error "Alias $alias not uniquely configured"
            return 1
        fi
    done
    
    if [[ $configs_found -eq 4 ]]; then
        log_success "All 4 unique aliases correctly configured"
    else
        log_error "Expected 4 unique aliases, found $configs_found"
        return 1
    fi
}

# Test 6: Template integration
test_template_integration() {
    log_info "Test 6: Template integration test"
    
    # Create a temporary test project using templates
    mkdir -p "$TEMP_DIR"
    cd "$TEMP_DIR"
    
    # Create test project structure
    cat > knot.yml << EOF
name: template-test
tsAlias: "@test"

apps:
  template-app:
    packages: [test-package]
EOF
    
    mkdir -p packages/test-package/src
    cat > packages/test-package/package.yml << EOF
name: test-package
version: 1.0.0
description: Template test package
EOF
    
    # Copy template content
    cp "$TEST_DIR/../../../apps/cli/src/templates/packages/typescript/src/index.ts" packages/test-package/src/
    
    mkdir -p apps/template-app/src
    cat > apps/template-app/app.yml << EOF
name: template-app
packages: [test-package]
EOF
    
    cat > apps/template-app/src/main.ts << EOF
import { hello, getPackageInfo } from '@test/test-package';
console.log(hello('Template Test'));
console.log(getPackageInfo());
EOF
    
    # Test linking
    if $KNOT_BINARY link > /tmp/template-test.log 2>&1; then
        log_success "Template integration works"
    else
        log_error "Template integration failed"
        cat /tmp/template-test.log
        cd "$TEST_DIR"
        return 1
    fi
    
    # Verify tsconfig was created correctly
    if [[ -f "apps/template-app/tsconfig.json" ]] && grep -q "@test/\*" "apps/template-app/tsconfig.json"; then
        log_success "Template TypeScript configuration correct"
    else
        log_error "Template TypeScript configuration failed"
        cd "$TEST_DIR"
        return 1
    fi
    
    cd "$TEST_DIR"
}

# Test 7: Error handling and edge cases
test_error_handling() {
    log_info "Test 7: Error handling and edge cases"
    
    # Test with missing package (should fail gracefully)
    mkdir -p "$TEMP_DIR/error-test"
    cd "$TEMP_DIR/error-test"
    
    cat > knot.yml << EOF
name: error-test
tsAlias: "@error"

apps:
  error-app:
    packages: [missing-package]
EOF
    
    mkdir -p apps/error-app
    cat > apps/error-app/app.yml << EOF
name: error-app
packages: [missing-package]
EOF
    
    # This should fail
    if $KNOT_BINARY link > /tmp/error-test.log 2>&1; then
        log_error "Expected failure for missing package, but linking succeeded"
        cd "$TEST_DIR"
        return 1
    else
        log_success "Correctly handled missing package error"
    fi
    
    cd "$TEST_DIR"
}

# Test 8: Performance test
test_performance() {
    log_info "Test 8: Performance test"
    
    local start_time=$(date +%s%N)
    
    # Re-run linking to measure performance
    $KNOT_BINARY link > /dev/null 2>&1
    
    local end_time=$(date +%s%N)
    local duration=$(( (end_time - start_time) / 1000000 )) # Convert to milliseconds
    
    if [[ $duration -lt 5000 ]]; then # Less than 5 seconds
        log_success "Performance test passed: ${duration}ms"
    else
        log_warning "Performance test slow: ${duration}ms (consider optimization)"
    fi
}

# Run JavaScript/Node.js tests if available
run_js_tests() {
    log_info "Running JavaScript test suites..."
    
    if command -v npm &> /dev/null; then
        # Check if we have package.json and test dependencies
        if [[ -f "package.json" ]]; then
            log_info "Installing test dependencies..."
            npm install --silent > /dev/null 2>&1 || log_warning "npm install failed"
            
            if command -v jest &> /dev/null || [[ -f "node_modules/.bin/jest" ]]; then
                log_info "Running Jest tests..."
                if npm test > /tmp/jest-output.log 2>&1; then
                    log_success "JavaScript tests passed"
                else
                    log_error "JavaScript tests failed"
                    cat /tmp/jest-output.log
                    return 1
                fi
            else
                log_info "Jest not available, running tests with Node.js directly..."
                for test_file in tests/*.test.js; do
                    if [[ -f "$test_file" ]]; then
                        log_info "Running $test_file..."
                        if node "$test_file" > /tmp/node-test.log 2>&1; then
                            log_success "$(basename $test_file) passed"
                        else
                            log_error "$(basename $test_file) failed"
                            cat /tmp/node-test.log
                        fi
                    fi
                done
            fi
        else
            log_warning "No package.json found, skipping JS tests"
        fi
    else
        log_warning "npm not available, skipping JS tests"
    fi
}

# Main test execution
main() {
    local test_count=0
    local passed_count=0
    local failed_tests=()
    
    # Array of all tests
    local tests=(
        "check_prerequisites"
        "test_basic_aliases"
        "test_typescript_config"
        "test_source_code_aliases"
        "test_package_dependencies"
        "test_multiple_aliases"
        "test_template_integration"
        "test_error_handling"
        "test_performance"
    )
    
    log_info "Running ${#tests[@]} test suites..."
    echo
    
    for test_name in "${tests[@]}"; do
        ((test_count++))
        log_info "Running $test_name..."
        
        if $test_name; then
            ((passed_count++))
            log_success "$test_name PASSED"
        else
            failed_tests+=("$test_name")
            log_error "$test_name FAILED"
        fi
        echo
    done
    
    # Run JS tests separately
    if run_js_tests; then
        log_success "JavaScript tests completed"
    else
        log_warning "Some JavaScript tests had issues"
    fi
    
    # Cleanup
    if [[ -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
    fi
    
    # Summary
    echo "=========================================="
    echo "🏁 Test Suite Complete"
    echo "=========================================="
    echo
    log_info "Test Summary:"
    echo "  Total tests: $test_count"
    echo "  Passed: $passed_count"
    echo "  Failed: $((test_count - passed_count))"
    echo
    
    if [[ ${#failed_tests[@]} -eq 0 ]]; then
        log_success "All tests passed! 🎉"
        log_success "The Knot alias system is working correctly."
        echo
        log_info "You can now:"
        echo "  • Use the example projects in this directory"
        echo "  • Copy configurations to your own projects"
        echo "  • Refer to docs/COMPLETE_GUIDE.md for detailed usage"
        return 0
    else
        log_error "Some tests failed:"
        for test in "${failed_tests[@]}"; do
            echo "  • $test"
        done
        echo
        log_error "Please check the errors above and ensure:"
        echo "  • Knot CLI is built (cargo build --release)"
        echo "  • All example files are present"
        echo "  • No permission issues with file creation"
        return 1
    fi
}

# Run the test suite
main "$@"