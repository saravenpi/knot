# TypeScript Package Template

This is a TypeScript package template that demonstrates how to create packages that work seamlessly with Knot's alias system.

## Alias System Integration

When this package is linked to apps, it can be imported using the app's configured alias:

```typescript
// If app uses tsAlias: "@ui"
import { hello, PackageManager } from '@ui/package-name';

// If app uses tsAlias: "#" (global default)
import { hello, PackageManager } from '#/package-name';

// If app uses tsAlias: "$"
import { hello, PackageManager } from '$/package-name';

// If app uses tsAlias: "@core"
import { hello, PackageManager } from '@core/package-name';
```

## Package Structure

```
package-name/
├── src/
│   ├── index.ts          # Main exports with alias support
│   ├── utils.ts          # Utility functions
│   └── types.ts          # Type definitions
├── package.yml           # Knot package configuration
├── package.json          # npm compatibility
├── tsconfig.json         # TypeScript configuration
└── README.md            # This file
```

## Usage Examples

### Basic Usage

```typescript
import { hello, getPackageInfo } from 'alias/package-name';

console.log(hello('World'));
console.log(getPackageInfo());
```

### Using Package Manager Class

```typescript
import { PackageManager, PackageConfig } from 'alias/package-name';

const manager = new PackageManager();

const config: PackageConfig = {
  name: 'my-package',
  version: '1.0.0',
  dependencies: ['utils', 'ui-components']
};

manager.addPackage(config);
console.log(manager.listPackages());
```

### Cross-Package Dependencies

When creating packages that depend on other local packages, use direct imports:

```typescript
// In this package's source files
import { someUtility } from 'utils';  // Direct reference to local package
import { Button } from 'ui-components';  // Another local package

// Apps importing this package will use aliases:
// import { MyComponent } from '@ui/my-package';
```

## Best Practices

1. **Use direct imports** within packages for local dependencies
2. **Document alias usage** in your package README
3. **Export interfaces** that might be shared across packages
4. **Provide clear examples** of how to import from your package
5. **Test with different alias configurations** to ensure compatibility

## Alias Configuration Examples

### Global Alias (knot.yml)
```yaml
name: my-project
tsAlias: "#"  # All apps inherit this unless overridden
```

### App-Specific Alias (app.yml)
```yaml
name: dashboard
tsAlias: "@ui"  # Override global alias for this app
packages:
  - package-name
  - utils
```

### Multiple Apps with Different Aliases
```yaml
# knot.yml
name: multi-alias-project
tsAlias: "#"  # Default

apps:
  dashboard-app:
    tsAlias: "@ui"    # Dashboard uses @ui
    packages: [package-name, ui-components]
    
  api-server:
    tsAlias: "@core"  # API uses @core  
    packages: [package-name, data-layer]
    
  mobile-app:
    # Inherits global "#"
    packages: [package-name, ui-components]
```

## TypeScript Configuration

The generated `tsconfig.json` will automatically include the correct paths for your alias:

```json
{
  "compilerOptions": {
    "paths": {
      "@ui/*": ["./knot_packages/*"],  // If tsAlias is "@ui"
      "#/*": ["./knot_packages/*"],    // If tsAlias is "#"  
      "$/*": ["./knot_packages/*"]     // If tsAlias is "$"
    }
  },
  "include": [
    "src/**/*",
    "knot_packages/**/*"  // Include linked packages
  ]
}
```

## Testing Alias Integration

Run the following to test that aliases work correctly:

```bash
# Link packages (generates tsconfig with aliases)
knot link

# Build with TypeScript (should resolve aliases)
npm run build

# Run tests that use aliased imports
npm test
```