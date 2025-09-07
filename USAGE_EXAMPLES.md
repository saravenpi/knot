# TypeScript Alias Integration Usage Examples

## Package-Specific Aliases

With the new TypeScript alias integration, imports are now more specific and intuitive:

### Before (Generic alias)
```typescript
// Old generic import - less clear which package is being used
import { User } from '#/user';
import { formatDate } from '#/date';
```

### After (Package-specific aliases)
```typescript
// New package-specific imports - crystal clear package source
import { User } from '#types/user';
import { formatDate } from '#utils/date';
```

## Configuration Examples

### Default '#' prefix (knot.yml):
```yaml
apps:
  my-app:
    tsAlias: true  # or '#'
    packages: [types, utils, validators]
```

**Generated tsconfig.json paths:**
```json
{
  "compilerOptions": {
    "paths": {
      "#types/*": ["./knot_packages/types/*"],
      "#utils/*": ["./knot_packages/utils/*"],
      "#validators/*": ["./knot_packages/validators/*"]
    }
  }
}
```

### Custom prefix:
```yaml
apps:
  my-app:
    tsAlias: "@knot/"
    packages: [types, utils]
```

**Generated tsconfig.json paths:**
```json
{
  "compilerOptions": {
    "paths": {
      "@knot/types/*": ["./knot_packages/types/*"],
      "@knot/utils/*": ["./knot_packages/utils/*"]
    }
  }
}
```

## Import Examples

### Types Package:
```typescript
// Import types
import type { ApiResponse, User, Product } from '#types/api';
import type { DatabaseConfig } from '#types/config';

// Use in code
const user: User = { id: 1, name: "John" };
```

### Utils Package:
```typescript
// Import utilities
import { formatCurrency, validateEmail } from '#utils/formatting';
import { debounce, throttle } from '#utils/performance';

// Use in code
const price = formatCurrency(29.99, 'USD');
const isValid = validateEmail('user@example.com');
```

## Error Handling

The system validates alias names to ensure they're valid TypeScript identifiers:

### Valid aliases:
- `#types` ✅
- `@pkg/utils` ✅
- `$helpers` ✅
- `_internal` ✅

### Invalid aliases (will cause errors):
- `#interface` ❌ (TypeScript reserved word)
- `#123pkg` ❌ (can't start with number)
- `#pkg-name` ❌ (hyphens not allowed in identifiers)

### Reserved word conflicts:
If a package name combined with the prefix creates a reserved word, the system will error:
- Package "interface" with prefix "#" → "#interface" ❌ (reserved word)
- Package "type" with prefix "#" → "#type" ❌ (reserved word)

## Benefits

1. **Clear package source**: Know exactly which package provides each import
2. **Better IDE support**: IDEs can provide better autocomplete and navigation
3. **Reduced naming conflicts**: Package-specific aliases prevent name collisions
4. **Type safety**: TypeScript can better track types across packages
5. **Maintainable imports**: Easier to refactor and reorganize code

## Migration from Generic Alias

Old imports using the generic pattern:
```typescript
import { helper } from '#/some-utility';
```

New imports using package-specific aliases:
```typescript
import { helper } from '#utils/some-utility';
```

The migration is straightforward - just specify which package the import comes from.