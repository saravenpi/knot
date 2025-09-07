# Utils Package

A foundational utility package providing common functions, types, and helpers that other packages in your Knot project can depend on. This package serves as the base layer in the dependency hierarchy.

## Overview

The `utils` package is designed to be a **base dependency** that other packages can safely import without creating circular dependencies. It provides:

- 📝 **Validation utilities** - Email, URL, password strength validation
- 🎨 **Formatting functions** - Currency, dates, numbers, file sizes
- 🛠️ **Helper functions** - ID generation, retries, deep merging, debouncing
- 📊 **Common types** - API responses, pagination, filtering, sorting
- 🔄 **Re-exported dependencies** - Commonly used functions from lodash and date-fns

## Dependency Chain Position

```
utils (base package - no internal dependencies)
 ↑
 └── Other packages depend on this
```

## Installation & Usage

```typescript
// Import specific functions
import { formatCurrency, validateEmail, generateId } from 'utils';

// Import default objects
import { formatters, validators, helpers } from 'utils';

// Import types
import { ApiResponse, ValidationError, SortConfig } from 'utils';

// Use re-exported external dependencies
import { formatDate, debounce } from 'utils';
```

## Key Features

### 1. Validation Functions
```typescript
import { validators, validateEmail } from 'utils';

// Email validation
const emailError = validateEmail('user@example.com');
if (emailError) {
  console.log(emailError.message);
}

// Using validator object
const isValidPhone = validators.isPhoneNumber('+1-555-123-4567');
```

### 2. Formatting Utilities
```typescript
import { formatters, formatCurrency } from 'utils';

// Currency formatting
const price = formatCurrency(29.99, 'USD'); // "$29.99"

// Using formatter object
const fileSize = formatters.fileSize(1024000); // "1 MB"
const slug = formatters.slugify('My Blog Post Title'); // "my-blog-post-title"
```

### 3. Helper Functions
```typescript
import { helpers, generateId, retry } from 'utils';

// Generate unique IDs
const uniqueId = generateId('user'); // "user_1a2b3c4d_xy9z8"

// Retry with backoff
const result = await retry(async () => {
  return fetch('/api/data');
}, 3, 1000);

// Deep merge objects
const merged = helpers.deepMerge(obj1, obj2);
```

### 4. Common Types
```typescript
import { ApiResponse, PaginatedResponse, ValidationError } from 'utils';

interface UserResponse extends ApiResponse<User> {
  // API response structure is already defined
}

function handleErrors(errors: ValidationError[]) {
  errors.forEach(error => {
    console.log(`${error.field}: ${error.message}`);
  });
}
```

## External Dependencies

The utils package includes these external dependencies:
- **date-fns** - Date manipulation and formatting
- **lodash** - Utility functions (debounce, throttle, cloneDeep)

These are re-exported for convenience, so other packages don't need to install them separately if they only need basic functionality.

## Best Practices

### ✅ Do
- Use this package for common utilities that multiple packages need
- Import specific functions to keep bundle size small
- Add new utilities here if they don't have external dependencies
- Use the provided types for consistency across packages

### ❌ Don't
- Add dependencies on other internal packages (creates circular dependencies)
- Include React-specific utilities (use ui-components package instead)
- Add heavy external dependencies without careful consideration
- Create utilities that are specific to one use case

## Extending the Package

When adding new utilities to this package:

1. **Keep it generic** - Utilities should be reusable across different contexts
2. **Avoid external dependencies** - Only add dependencies that provide significant value
3. **Export everything** - Make sure new utilities are exported from `index.ts`
4. **Add types** - Include TypeScript types for better developer experience
5. **Document usage** - Add examples in this README

## Example: Adding a New Utility

```typescript
// src/crypto.ts
export function hashString(input: string): string {
  // Implementation
}

// src/index.ts
export * from './crypto';
```

This foundational package enables clean architecture by providing a stable base that other packages can depend on without worrying about circular dependencies or version conflicts.