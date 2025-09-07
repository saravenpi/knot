# Alias Configuration Support in Knot CLI

This document outlines the comprehensive alias configuration support that has been implemented in the Knot CLI configuration system.

## Overview

The Knot CLI now supports package aliases, allowing users to assign short, memorable names to packages. This is particularly useful for:

- Long package names (`long-package-name` → `utils`)
- Complex scoped packages (`@team/complex-package@1.0.0` → `api`)
- Improving import readability in TypeScript projects
- Reducing cognitive overhead when working with many packages

## Configuration Structure

### New Types

#### PackageEntry
```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PackageEntry {
    pub name: String,
    pub alias: Option<String>,
}
```

#### PackageSpec  
```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum PackageSpec {
    String(String),           // Simple package name
    Object(PackageEntry),     // Package with optional alias
}
```

### Configuration Examples

#### Basic Alias Configuration in knot.yml
```yaml
name: My Project
apps:
  my-app:
    packages:
      - name: long-package-name
        alias: utils
      - name: "@team/complex-package@1.0.0"
        alias: api
      - simple-package  # No alias
```

#### Advanced Configuration in app.yml
```yaml
name: my-app
description: "My application with package aliases"
packages:
  - name: database-connector
    alias: db
  - name: "@auth/jwt-handler@2.1.0"
    alias: auth
  - shared-types
```

## Features Implemented

### 1. Alias Parsing and Validation

- **Valid alias names**: Must be valid JavaScript/TypeScript identifiers
- **Character restrictions**: Only alphanumeric, underscore (`_`), and dollar sign (`$`)
- **Reserved word detection**: Prevents using JavaScript/TypeScript keywords
- **Length limits**: Maximum 50 characters
- **No digit prefix**: Aliases cannot start with numbers

### 2. Conflict Detection

#### Local Conflict Detection
- Prevents duplicate aliases within a single app
- Clear error messages indicating conflicting packages

#### Global Conflict Detection  
- Detects alias conflicts across different apps in the same project
- Provides detailed error messages with app and package context

### 3. Backward Compatibility

- Existing configurations with simple string packages continue to work
- Mixed configurations (strings + objects with aliases) are supported
- No breaking changes to existing API

### 4. Alias Resolution Methods

#### Project-level Methods
```rust
impl Project {
    // Get all package entries with alias information for an app
    pub fn get_app_package_entries(&self, app_name: &str) -> Vec<PackageEntry>
    
    // Resolve an alias to its actual package name
    pub fn resolve_alias(&self, app_name: &str, alias: &str) -> Option<String>
    
    // Get the alias for a specific package (reverse lookup)
    pub fn get_package_alias(&self, app_name: &str, package_name: &str) -> Option<String>
    
    // Get all aliases for an app as a HashMap
    pub fn get_all_aliases(&self, app_name: &str) -> HashMap<String, String>
    
    // Validate aliases within an app
    pub fn validate_app_aliases(&self, app_name: &str) -> Result<()>
}
```

## Validation Rules

### Alias Name Validation
1. **Non-empty**: Alias cannot be empty or only whitespace
2. **Valid characters**: Only `a-zA-Z0-9_$` allowed
3. **Start character**: Must start with letter, underscore, or dollar sign
4. **Reserved words**: Cannot be JavaScript/TypeScript keywords:
   - `import`, `export`, `default`, `const`, `let`, `var`
   - `function`, `class`, `interface`, `type`, `namespace`, `enum`
5. **Length limit**: Maximum 50 characters

### Conflict Detection
1. **Within app**: No duplicate aliases in the same app
2. **Across apps**: No duplicate aliases across different apps (global scope)
3. **Clear error messages** with context about which packages conflict

## Error Messages

The system provides user-friendly error messages:

```
Alias conflict: alias 'utils' is used by both 'package-a' and 'package-b'
Global alias conflict: alias 'api' is used by package 'pkg1' in app 'web' and package 'pkg2' in app 'mobile'
Alias 'invalid-name!' contains invalid characters. Only alphanumeric characters, underscores, and dollar signs allowed
Alias 'import' is a reserved keyword
```

## Integration Points

### Configuration Loading
- `AppConfig` and `AppDependencies` updated to handle `PackageSpec`
- Automatic conversion between string packages and `PackageEntry` objects
- Seamless integration with existing configuration loading logic

### TypeScript Integration
- Foundation laid for TypeScript path mapping using aliases
- Alias information available for import generation
- Compatible with existing `tsAlias` functionality

## Usage Examples

### Simple Package with Alias
```yaml
packages:
  - name: very-long-package-name-that-is-hard-to-type
    alias: utils
```

### Scoped Package with Version and Alias
```yaml
packages:
  - name: "@organization/complex-package-name@2.1.0"
    alias: api
```

### Mixed Configuration
```yaml
packages:
  - name: package-with-alias
    alias: aliased
  - simple-package
  - another-simple-package
```

## Files Modified

### Core Configuration (`config.rs`)
- Added `PackageEntry` and `PackageSpec` structures
- Updated `AppDependencies` and `AppConfig` to support aliases
- Added comprehensive validation methods
- Implemented conflict detection logic

### Project Management (`project.rs`)
- Added alias resolution methods
- Updated dependency retrieval to work with new structures
- Added package entry processing logic

### Test Support
- Created comprehensive test suite for alias functionality
- Examples for valid and invalid configurations
- Conflict detection test cases

## Future Enhancements

The implemented system provides a solid foundation for:
1. **TypeScript path mapping generation** using aliases
2. **Import statement optimization** in generated code
3. **IDE integration** for better package discovery
4. **Documentation generation** with alias cross-references

This alias configuration support significantly improves the developer experience when working with complex package hierarchies in Knot-managed monorepos.