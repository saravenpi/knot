# YAML Variable Interpolation Examples

This directory contains comprehensive examples demonstrating the YAML variable interpolation system for the Knot project.

## Example Files

### Basic Usage Examples

- **`basic_interpolation.yml`** - Simple variable interpolation patterns
  - String interpolation in different contexts
  - Multiple variables in single values
  - Variables in arrays and objects
  - Quoted vs unquoted interpolation

### Advanced Patterns

- **`nested_variables.yml`** - Complex nested variable references
  - Multi-level variable dependencies
  - Environment and region-specific configurations
  - Service naming conventions
  - URL and path construction

### Configuration Examples

- **`knot_config_example.yml`** - Complete knot.yml with variables
  - Project-wide configuration
  - Script definitions with interpolated commands
  - App and package configurations
  - CI/CD and deployment settings

- **`package_config_example.yml`** - Package configuration with variables  
  - Package metadata and versioning
  - Build and publishing scripts
  - Dependency management
  - Registry and repository configurations

- **`app_config_example.yml`** - Application configuration with variables
  - Environment-specific settings
  - Build and development scripts
  - Service integrations
  - Performance and security configurations

### Error Testing

- **`error_cases.yml`** - Various error scenarios for testing
  - Undefined variable references
  - Circular dependencies
  - Malformed syntax
  - Edge cases and boundary conditions

## Variable Interpolation Syntax

The variable interpolation system uses the `${variable_name}` syntax:

```yaml
variables:
  app_name: "my-app"
  version: "1.0.0"
  
name: ${app_name}
full_name: "${app_name} v${version}"
```

## Features Demonstrated

### 1. Basic Interpolation
- Simple variable substitution
- Multiple variables in single values
- Variables in different YAML data types

### 2. Nested Variables
- Variables that reference other variables
- Multi-level dependency resolution
- Complex interpolation patterns

### 3. Built-in Variables
- System-provided variables (project_name, workspace_root, etc.)
- Environment variable fallbacks
- Variable override precedence

### 4. Advanced Patterns
- Variables in YAML keys
- File path construction
- URL and connection string building
- Conditional configurations

### 5. Error Handling
- Undefined variable detection
- Circular dependency prevention
- Malformed syntax handling
- Performance limits and safeguards

## Usage in Tests

These examples are used in the comprehensive test suite to verify:

1. **Functionality** - All interpolation patterns work correctly
2. **Error Handling** - Errors are detected and reported appropriately  
3. **Performance** - Large configurations process efficiently
4. **Edge Cases** - Unusual but valid scenarios are handled
5. **Integration** - Real Knot configurations work with interpolation

## Implementation Notes

The interpolation system is designed to be:

- **Safe** - Prevents infinite recursion and circular dependencies
- **Fast** - Efficient processing of large configuration files
- **Flexible** - Supports complex nested variable patterns
- **Compatible** - Works with existing Knot configuration formats
- **Extensible** - Easy to add new built-in variables and features

## Testing Strategy

The examples support multiple testing approaches:

1. **Unit Tests** - Individual interpolation features
2. **Integration Tests** - Real configuration parsing
3. **Performance Tests** - Large file processing
4. **Error Tests** - Invalid configuration handling
5. **Edge Case Tests** - Boundary conditions and special characters

Each example file is designed to be both a demonstration of capabilities and a source of test cases for the interpolation system.