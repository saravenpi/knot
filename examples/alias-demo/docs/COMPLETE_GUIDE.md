# Complete Guide to Knot Package Aliases

This comprehensive guide demonstrates every aspect of Knot's package alias system through practical examples and real-world scenarios.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Alias Configuration](#alias-configuration)
3. [Practical Scenarios](#practical-scenarios)
4. [Advanced Usage](#advanced-usage)
5. [Best Practices](#best-practices)
6. [Troubleshooting](#troubleshooting)
7. [Migration Guide](#migration-guide)

## Quick Start

### Basic Setup

```yaml
# knot.yml
name: my-project
tsAlias: "#"  # Global alias for all apps

apps:
  web-app:
    packages: [utils, ui-components]
    
  mobile-app:
    tsAlias: "@mobile"  # Override for this app
    packages: [utils, ui-components]
```

After running `knot link`, your apps can import packages using aliases:

```typescript
// In web-app (uses global "#")
import { Button } from '#/ui-components';
import { formatDate } from '#/utils';

// In mobile-app (uses "@mobile")
import { Button } from '@mobile/ui-components';
import { formatDate } from '@mobile/utils';
```

## Alias Configuration

### Global Aliases

Set a default alias for all apps in `knot.yml`:

```yaml
name: global-alias-project
tsAlias: "#"  # Short and universal
# or
tsAlias: "@pkg"  # More descriptive
# or  
tsAlias: "$"  # Ultra-short for heavy usage
```

### App-Specific Aliases

Override the global alias for specific apps:

```yaml
name: mixed-alias-project
tsAlias: "#"  # Global default

apps:
  frontend:
    tsAlias: "@ui"     # UI-focused alias
    packages: [components, design-tokens]
    
  backend:
    tsAlias: "@core"   # Core functionality alias
    packages: [database, auth, utils]
    
  mobile:
    # Inherits global "#"
    packages: [components, utils]
    
  analytics:
    tsAlias: "$"       # Short for frequent imports
    packages: [utils, charts, data-processing]
```

### Conditional Aliases

Configure aliases based on app context:

```yaml
# knot.yml
name: conditional-aliases
tsAlias: "#"

apps:
  # Frontend applications use @ui
  admin-dashboard:
    tsAlias: "@ui"
    packages: [admin-components, utils, auth]
    
  customer-portal:
    tsAlias: "@ui"
    packages: [customer-components, utils, payments]
    
  # Backend services use @core
  api-gateway:
    tsAlias: "@core"
    packages: [gateway-utils, auth, rate-limiting]
    
  user-service:
    tsAlias: "@core"
    packages: [user-management, database, utils]
    
  # Mobile apps inherit global
  ios-app:
    packages: [mobile-components, utils]
    
  android-app:
    packages: [mobile-components, utils]
```

## Practical Scenarios

### Scenario 1: E-commerce Platform

```yaml
name: ecommerce-platform
description: Multi-app e-commerce platform with organized aliases
tsAlias: "@shop"  # Global default

apps:
  # Customer-facing applications
  storefront:
    tsAlias: "@store"
    packages:
      - product-catalog
      - shopping-cart
      - checkout-flow
      - ui-components
      - utils
      
  mobile-store:
    tsAlias: "@store"
    packages:
      - product-catalog
      - shopping-cart
      - checkout-flow
      - mobile-components
      - utils
      
  # Admin applications  
  admin-panel:
    tsAlias: "@admin"
    packages:
      - admin-components
      - inventory-management
      - analytics-dashboard
      - auth
      - utils
      
  vendor-portal:
    tsAlias: "@vendor"
    packages:
      - vendor-components
      - product-management
      - sales-analytics
      - auth
      - utils
      
  # Backend services
  api-server:
    tsAlias: "@api"
    packages:
      - product-service
      - user-service
      - payment-service
      - database
      - auth
      - utils
      
  # Internal tools
  data-processor:
    tsAlias: "@data"
    packages:
      - etl-pipeline
      - analytics-engine
      - report-generator
      - database
      - utils
```

Usage examples:

```typescript
// storefront/src/components/ProductList.tsx
import { ProductCard, Button } from '@store/ui-components';
import { formatPrice, debounce } from '@store/utils';
import { Product } from '@store/product-catalog';

// admin-panel/src/pages/Inventory.tsx
import { DataTable, Modal } from '@admin/admin-components';
import { InventoryItem } from '@admin/inventory-management';
import { formatDate } from '@admin/utils';

// api-server/src/routes/products.ts
import { ProductService } from '@api/product-service';
import { authenticate } from '@api/auth';
import { logger } from '@api/utils';
```

### Scenario 2: Development Tools Suite

```yaml
name: dev-tools-suite
description: Developer productivity tools with semantic aliases
tsAlias: "@dev"

apps:
  # Code editors and IDEs
  code-editor:
    tsAlias: "@editor"
    packages:
      - syntax-highlighting
      - auto-complete
      - file-explorer
      - editor-core
      - utils
      
  # Build and deployment tools
  build-pipeline:
    tsAlias: "@build"
    packages:
      - bundler
      - transpiler
      - optimizer
      - file-system
      - utils
      
  deployment-manager:
    tsAlias: "@deploy"
    packages:
      - docker-utils
      - kubernetes-client
      - cloud-providers
      - monitoring
      - utils
      
  # Testing frameworks
  test-runner:
    tsAlias: "@test"
    packages:
      - test-framework
      - assertions
      - mocking
      - coverage
      - utils
      
  # Documentation tools
  doc-generator:
    tsAlias: "@docs"
    packages:
      - markdown-parser
      - template-engine
      - static-site-generator
      - utils
```

### Scenario 3: Team-Based Development

```yaml
name: team-project
description: Multi-team project with team-based aliases

# Global packages available to all teams
packages:
  - shared-utils
  - common-types
  - design-system

apps:
  # Frontend Team
  web-dashboard:
    tsAlias: "@frontend"
    packages:
      - dashboard-components
      - data-visualization
      - shared-utils
      - design-system
      
  admin-interface:
    tsAlias: "@frontend"
    packages:
      - admin-components
      - form-validation
      - shared-utils
      - design-system
      
  # Backend Team
  user-api:
    tsAlias: "@backend"
    packages:
      - user-management
      - authentication
      - database-orm
      - shared-utils
      
  notification-service:
    tsAlias: "@backend"
    packages:
      - email-templates
      - push-notifications
      - queue-management
      - shared-utils
      
  # Mobile Team
  ios-app:
    tsAlias: "@mobile"
    packages:
      - native-components
      - offline-storage
      - shared-utils
      - common-types
      
  android-app:
    tsAlias: "@mobile"
    packages:
      - native-components
      - offline-storage
      - shared-utils
      - common-types
      
  # DevOps Team
  monitoring-dashboard:
    tsAlias: "@devops"
    packages:
      - metrics-collector
      - log-analyzer
      - alert-manager
      - shared-utils
```

## Advanced Usage

### Dynamic Alias Resolution

```typescript
// utils/alias-resolver.ts
export const resolveAlias = (packageName: string): string => {
  const aliasMap = {
    'ui-components': process.env.UI_ALIAS || '@ui',
    'utils': process.env.UTILS_ALIAS || '@utils',
    'api-client': process.env.API_ALIAS || '@api'
  };
  
  return aliasMap[packageName] || '@pkg';
};

// Usage in build tools
import { resolveAlias } from './alias-resolver';

const importStatement = `import { Component } from '${resolveAlias('ui-components')}/components';`;
```

### Alias Validation

```typescript
// scripts/validate-aliases.ts
import { glob } from 'glob';
import { readFileSync } from 'fs';
import * as yaml from 'js-yaml';

interface AliasConfig {
  global?: string;
  apps: Record<string, { alias?: string; packages: string[] }>;
}

export const validateAliases = async (): Promise<void> => {
  const knotConfig = yaml.load(readFileSync('knot.yml', 'utf8')) as any;
  const globalAlias = knotConfig.tsAlias;
  
  // Find all TypeScript files
  const tsFiles = await glob('apps/*/src/**/*.{ts,tsx}');
  
  for (const file of tsFiles) {
    const content = readFileSync(file, 'utf8');
    const appName = file.split('/')[1];
    const appConfig = knotConfig.apps[appName];
    const expectedAlias = appConfig?.tsAlias || globalAlias;
    
    // Check for incorrect alias usage
    const importPattern = /import.*from\s+['"]([^'"]+)['"];?/g;
    let match;
    
    while ((match = importPattern.exec(content)) !== null) {
      const importPath = match[1];
      
      if (importPath.startsWith('@') || importPath.startsWith('#') || importPath.startsWith('$')) {
        if (!importPath.startsWith(expectedAlias)) {
          console.warn(`Incorrect alias in ${file}: expected ${expectedAlias}, found ${importPath}`);
        }
      }
    }
  }
};
```

### Monorepo Integration

```yaml
# knot.yml for monorepo
name: monorepo-project
description: Large monorepo with organized aliases

# Global packages
packages:
  # Core utilities
  - core-utils
  - shared-types
  - config-manager
  
  # UI packages
  - design-tokens
  - ui-primitives
  - component-library
  
  # Business logic
  - user-domain
  - product-domain
  - order-domain

apps:
  # Web applications
  customer-app:
    tsAlias: "@customer"
    packages:
      - component-library
      - user-domain
      - product-domain
      - order-domain
      - core-utils
      
  admin-app:
    tsAlias: "@admin"
    packages:
      - component-library
      - user-domain
      - product-domain
      - order-domain
      - admin-components
      - core-utils
      
  # Mobile applications
  mobile-customer:
    tsAlias: "@mobile"
    packages:
      - mobile-components
      - user-domain
      - product-domain
      - order-domain
      - core-utils
      
  # Microservices
  user-service:
    tsAlias: "@service"
    packages:
      - user-domain
      - database-adapters
      - message-queue
      - core-utils
      
  product-service:
    tsAlias: "@service"
    packages:
      - product-domain
      - database-adapters
      - message-queue
      - core-utils
      
  # Tools and utilities
  data-migration:
    tsAlias: "@tools"
    packages:
      - migration-utils
      - database-adapters
      - all-domains
      - core-utils
```

## Best Practices

### 1. Alias Naming Conventions

```yaml
# ✅ Good: Semantic and consistent
frontend-app:
  tsAlias: "@ui"      # Clear purpose
  
backend-service:
  tsAlias: "@api"     # Short and clear
  
mobile-app:
  tsAlias: "@mobile"  # Descriptive

# ✅ Good: Team-based
team-alpha-app:
  tsAlias: "@alpha"
  
team-beta-app:
  tsAlias: "@beta"

# ❌ Avoid: Confusing or inconsistent
confusing-app:
  tsAlias: "@x1y2z3"  # Meaningless
  
inconsistent-app:
  tsAlias: "randomString"  # No pattern
```

### 2. Import Organization

```typescript
// ✅ Good: Group imports by alias
// External dependencies
import React from 'react';
import { Router } from 'express';

// Knot packages via alias
import { Button, Input } from '@ui/components';
import { formatDate, debounce } from '@ui/utils';
import { User, ApiResponse } from '@ui/types';

// ✅ Good: Use consistent alias patterns
import { 
  UserService,
  ProductService 
} from '@api/services';

import { 
  validateEmail,
  sanitizeInput 
} from '@api/validators';

// ❌ Avoid: Mixed alias usage
import { Button } from '@ui/components';
import { formatDate } from '#/utils';  // Inconsistent alias
import { User } from '@different/types';  // Another different alias
```

### 3. Package Design for Aliases

```typescript
// ✅ Good: Clear exports structure
// packages/ui-components/src/index.ts
export { Button } from './Button';
export { Input } from './Input';
export { Modal } from './Modal';

export type { ButtonProps } from './Button';
export type { InputProps } from './Input';
export type { ModalProps } from './Modal';

// packages/utils/src/index.ts
export * from './strings';
export * from './dates';
export * from './validation';

// Sub-path exports
export { debounce, throttle } from './performance';
```

### 4. Documentation Standards

```typescript
/**
 * @fileoverview User management utilities
 * 
 * Import examples:
 * ```typescript
 * // If your app uses @api alias:
 * import { createUser, validateUser } from '@api/user-utils';
 * 
 * // If your app uses # alias:
 * import { createUser, validateUser } from '#/user-utils';
 * ```
 */

export interface User {
  id: string;
  email: string;
  name: string;
}

/**
 * Creates a new user with validation
 * 
 * @example
 * ```typescript
 * import { createUser } from '@api/user-utils';
 * 
 * const user = await createUser({
 *   email: 'user@example.com',
 *   name: 'John Doe'
 * });
 * ```
 */
export const createUser = async (userData: Omit<User, 'id'>): Promise<User> => {
  // Implementation
};
```

## Troubleshooting

### Common Issues and Solutions

#### 1. TypeScript Cannot Resolve Alias

**Problem:**
```
Cannot find module '@ui/components' or its corresponding type declarations.
```

**Solution:**
```bash
# Re-run knot link to regenerate tsconfig.json
knot link

# Check that tsconfig.json has correct paths
cat apps/your-app/tsconfig.json
```

#### 2. Alias Not Working in Tests

**Problem:**
```typescript
// Test file
import { Button } from '@ui/components'; // Module not found
```

**Solution:**
```json
// jest.config.js
module.exports = {
  moduleNameMapper: {
    '^@ui/(.*)$': '<rootDir>/knot_packages/$1'
  }
};
```

#### 3. Build Tools Don't Recognize Aliases

**Problem:**
```
Webpack can't resolve '@ui/components'
```

**Solution:**
```javascript
// webpack.config.js
module.exports = {
  resolve: {
    alias: {
      '@ui': path.resolve(__dirname, 'knot_packages')
    }
  }
};
```

#### 4. Mixed Alias Configurations

**Problem:**
Different parts of the codebase use different aliases.

**Solution:**
```bash
# Use validation script
npm run validate-aliases

# Standardize on one alias per app
# Update imports using find/replace or codemod
```

### Debug Commands

```bash
# Check current alias configuration
knot config show

# Validate alias usage across project
knot validate aliases

# List all linked packages with their aliases
knot list --aliases

# Show TypeScript configuration
knot tsconfig show app-name
```

## Migration Guide

### Migrating from Direct Imports

**Before:**
```typescript
// Direct relative imports
import { Button } from '../../knot_packages/ui-components';
import { formatDate } from '../../knot_packages/utils/dates';
```

**After:**
```typescript
// Using aliases
import { Button } from '@ui/ui-components';
import { formatDate } from '@ui/utils/dates';
```

**Migration Steps:**

1. Configure aliases in `knot.yml`:
```yaml
name: existing-project
tsAlias: "@ui"  # Choose your preferred alias
```

2. Run knot link to generate TypeScript configuration:
```bash
knot link
```

3. Update imports using find/replace or automated tools:
```bash
# Example sed command for mass replacement
find apps -name "*.ts" -o -name "*.tsx" | xargs sed -i 's|../../knot_packages/|@ui/|g'
```

### Updating Existing Projects

1. **Assess Current State:**
```bash
# Find all current import patterns
grep -r "from.*knot_packages" apps/
```

2. **Plan Alias Strategy:**
```yaml
# Document your alias strategy
# knot.yml
name: legacy-project
tsAlias: "@pkg"  # Start simple

apps:
  main-app:
    packages: [utils, components]
```

3. **Gradual Migration:**
```typescript
// Phase 1: Add alias support alongside existing imports
import { Button } from '../../knot_packages/ui-components'; // Old
import { Button } from '@pkg/ui-components'; // New

// Phase 2: Remove old imports once all are migrated
import { Button } from '@pkg/ui-components'; // Final
```

4. **Validate Migration:**
```bash
# Ensure no old import patterns remain
grep -r "\.\./.*knot_packages" apps/ || echo "Migration complete!"
```

## Real-World Examples

This directory contains complete, working examples:

- `examples/alias-demo/` - Full demonstration project
- `examples/monorepo-aliases/` - Large monorepo example
- `examples/team-workflows/` - Team-based development
- `examples/migration-example/` - Before/after migration

Each example includes:
- Complete project structure
- Working code with alias usage
- Test suites
- Build configurations
- Documentation

Run any example:

```bash
cd examples/alias-demo
knot link
npm run build
npm test
```

This guide covers all aspects of Knot's alias system. For additional help, see the test files in `tests/` or refer to the official Knot documentation.