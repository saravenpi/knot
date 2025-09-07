# Knot Package Alias System - Complete Examples & Testing

This directory contains comprehensive examples and testing for Knot's package alias system. It demonstrates how to use aliases effectively across different project configurations and provides thorough test coverage to ensure everything works correctly.

## 🚀 Quick Start

```bash
# Run all tests and examples
./run-tests.sh

# Link packages with aliases
knot link

# Explore the working examples
ls apps/         # See different alias configurations
cat knot.yml     # View project configuration
```

## 📁 Project Structure

```
alias-demo/
├── README.md                    # This file
├── knot.yml                     # Main project with global & app-specific aliases
├── run-tests.sh                 # Comprehensive test runner
├── docs/
│   └── COMPLETE_GUIDE.md        # Detailed documentation and examples
├── packages/                    # Shared packages used by apps
│   ├── utils/                   # Common utilities
│   ├── ui-components/           # React UI components
│   ├── data-layer/              # Data access and caching
│   └── api-client/              # HTTP client utilities
├── apps/                        # Applications using different aliases
│   ├── dashboard-app/           # Uses @ui alias
│   ├── api-server/              # Uses @core alias  
│   ├── mobile-app/              # Uses # alias (global)
│   └── analytics/               # Uses $ alias
└── tests/                       # Comprehensive test suites
    ├── alias-functionality.test.js    # Core functionality tests
    ├── edge-cases.test.js             # Edge case and error handling
    └── integration-workflow.test.js   # End-to-end workflow tests
```

## 🎯 What This Demonstrates

### 1. Multiple Alias Configurations

Each app uses a different alias to show the flexibility of the system:

```yaml
# knot.yml
name: alias-demo
tsAlias: "#"  # Global default

apps:
  dashboard-app:
    tsAlias: "@ui"    # UI-focused development
  api-server:
    tsAlias: "@core"  # Backend services
  mobile-app:
    # Inherits global "#"
  analytics:
    tsAlias: "$"      # Short for frequent imports
```

### 2. Real-World Import Patterns

See how aliases work in practice:

```typescript
// dashboard-app/src/main.tsx (uses @ui)
import { Button, Input } from '@ui/ui-components';
import { formatDate } from '@ui/utils/dates';
import { MemoryCache } from '@ui/data-layer/cache';

// api-server/src/server.ts (uses @core)
import { isEmail } from '@core/utils/validation';
import { globalCache } from '@core/data-layer/cache';

// mobile-app/src/App.tsx (uses #)
import { Button } from '#/ui-components';
import { formatDate } from '#/utils/dates';

// analytics/src/main.ts (uses $)
import { average, median } from '$/utils/math';
import { MemoryCache } from '$/data-layer/cache';
```

### 3. TypeScript Integration

Generated `tsconfig.json` files show how aliases are configured:

```json
{
  "compilerOptions": {
    "paths": {
      "@ui/*": ["./knot_packages/*"]
    }
  },
  "include": [
    "src/**/*",
    "knot_packages/**/*"
  ]
}
```

### 4. Package Dependencies

Packages can reference each other using direct imports, which become aliased in apps:

```typescript
// In packages/ui-components/src/Button/index.ts
import { debounce } from 'utils';  // Direct import

// In apps, this becomes:
// import { Button } from '@ui/ui-components'; 
// (Button component has debounce via alias resolution)
```

## 🧪 Test Coverage

### Functionality Tests (`alias-functionality.test.js`)

- ✅ Basic alias configuration reading
- ✅ Package linking with aliases  
- ✅ TypeScript configuration generation
- ✅ Import resolution validation
- ✅ Multiple alias configurations
- ✅ Performance benchmarks

### Edge Cases (`edge-cases.test.js`)

- ✅ Invalid alias configurations
- ✅ Conflicting aliases
- ✅ File system edge cases
- ✅ Special character handling
- ✅ Error recovery scenarios
- ✅ Unicode alias testing

### Integration Tests (`integration-workflow.test.js`)

- ✅ End-to-end workflow testing
- ✅ Multi-app alias setups
- ✅ Error recovery and retry
- ✅ TypeScript compilation
- ✅ Performance under load

## 🔧 Running the Tests

### Automated Test Suite

```bash
# Run all tests with detailed output
./run-tests.sh

# The script will:
# 1. Verify prerequisites (knot binary, node, npm)
# 2. Test basic alias functionality
# 3. Validate TypeScript configuration
# 4. Check source code alias usage
# 5. Test package dependencies
# 6. Verify multiple alias configs
# 7. Test template integration
# 8. Test error handling
# 9. Measure performance
# 10. Run JavaScript test suites
```

### Manual Testing

```bash
# Link packages and generate configs
knot link

# Check that aliases work
ls apps/dashboard-app/knot_packages/  # Should show linked packages
cat apps/dashboard-app/tsconfig.json  # Should show @ui alias

# Test different apps
cd apps/api-server && cat tsconfig.json  # Should show @core alias
cd apps/mobile-app && cat tsconfig.json  # Should show # alias
cd apps/analytics && cat tsconfig.json   # Should show $ alias
```

### JavaScript Test Suites

```bash
# If you have Node.js and npm installed
npm install    # Install test dependencies
npm test       # Run Jest tests

# Or run individual test files
node tests/alias-functionality.test.js
node tests/edge-cases.test.js
node tests/integration-workflow.test.js
```

## 📖 Documentation

### Complete Guide

See [`docs/COMPLETE_GUIDE.md`](docs/COMPLETE_GUIDE.md) for:

- 📚 Detailed configuration options
- 🏗️ Real-world project examples
- 💡 Best practices and conventions
- 🐛 Troubleshooting guide
- 🔄 Migration strategies
- 🎯 Advanced usage patterns

### Package Templates

Updated templates in `../../../apps/cli/src/templates/` show:

- How to structure packages for alias usage
- Best practices for exports and imports
- TypeScript integration examples
- Documentation standards

## 🌟 Key Features Demonstrated

### 1. Flexible Alias Configuration

- **Global aliases** for consistent project-wide imports
- **App-specific overrides** for specialized workflows  
- **Mixed configurations** within the same project
- **Semantic naming** for different contexts

### 2. TypeScript Integration

- **Automatic path mapping** generation
- **Include path management** for linked packages
- **Existing config preservation** when updating
- **IDE support** through proper tsconfig.json

### 3. Developer Experience

- **Clear import paths** that indicate package source
- **Consistent patterns** across different applications
- **Team-friendly** configurations for collaborative development
- **Error handling** with helpful diagnostics

### 4. Build Tool Compatibility

- **Webpack** configuration examples
- **Jest** test configuration examples
- **ESLint** integration patterns
- **Other build tools** via standard TypeScript paths

## 🛠️ Usage in Your Projects

### 1. Copy Configuration Pattern

```yaml
# Copy this pattern to your knot.yml
name: your-project
tsAlias: "#"  # or your preferred global alias

apps:
  your-app:
    tsAlias: "@yourteam"  # optional override
    packages: [utils, components]
```

### 2. Follow Import Patterns

```typescript
// Use semantic aliases that match your workflow
import { Component } from '@ui/components';      // UI development
import { Service } from '@api/services';         // Backend services  
import { Helper } from '@utils/helpers';         // Utility functions
import { Type } from '@types/models';            // Type definitions
```

### 3. Structure Packages

```
packages/
├── core-utils/          # Foundation utilities
├── ui-components/       # Reusable UI components
├── business-logic/      # Domain-specific logic
└── integrations/        # External service integrations
```

### 4. Test Your Setup

```bash
# Copy and run the test script
cp run-tests.sh your-project/
./run-tests.sh  # Adapt for your project structure
```

## 🤝 Contributing

This example project serves as both documentation and test suite. When contributing:

1. **Add test cases** for new alias features
2. **Update examples** to show new capabilities  
3. **Maintain documentation** in sync with implementation
4. **Test compatibility** with different project structures

## 📋 Prerequisites

- **Knot CLI** built from source (`cargo build --release`)
- **Node.js** (optional, for JavaScript tests)
- **npm** (optional, for package management tests)
- **TypeScript** (optional, for compilation tests)

## 🎉 Success Criteria

After running the tests, you should see:

- ✅ All test suites passing
- ✅ TypeScript configurations generated correctly
- ✅ Packages linked and accessible via aliases
- ✅ Source code imports working with different aliases
- ✅ Performance within acceptable limits
- ✅ Error handling working gracefully

This comprehensive test suite ensures that Knot's alias system works reliably across different project configurations and use cases.

---

**Next Steps:** Explore the [`docs/COMPLETE_GUIDE.md`](docs/COMPLETE_GUIDE.md) for detailed usage patterns, or adapt these examples to your own projects!