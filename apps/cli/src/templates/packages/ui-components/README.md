# UI Components Package

A React component library that demonstrates **single internal dependency** pattern. This package depends on the `utils` package for validation, formatting, and helper functions while providing reusable UI components.

## Overview

The `ui-components` package provides a collection of React components with built-in validation, formatting, and animations. It showcases how to create components that leverage shared utilities from another package.

## Dependency Chain Position

```
utils (base package)
 ↑
ui-components (depends on utils)
 ↑
└── Applications and other packages can depend on this
```

## Installation & Usage

```bash
# The package.yml automatically handles internal dependencies
knot add ui-components
```

```typescript
import { Button, Input, DataTable, Card } from 'ui-components';
import type { ButtonProps, InputProps } from 'ui-components';
```

## Components Overview

### 1. Button Component
Interactive button with loading states and animations using Framer Motion.

```typescript
import { Button } from 'ui-components';

<Button 
  variant="primary" 
  size="lg" 
  loading={isSubmitting}
  onClick={handleSubmit}
>
  Submit Form
</Button>
```

**Features:**
- Uses `generateId` from utils package for unique IDs
- Framer Motion animations
- Multiple variants and sizes
- Loading and disabled states

### 2. Input Component
Form input with built-in validation using utils package validators.

```typescript
import { Input } from 'ui-components';

<Input
  label="Email Address"
  value={email}
  onChange={setEmail}
  type="email"
  validation="email"
  required
  helperText="We'll never share your email"
/>
```

**Features:**
- Uses `validators` from utils package for real-time validation
- Uses `generateId` for accessibility
- Support for various validation types
- Error handling and display

### 3. DataTable Component
Advanced data table with sorting, formatting, and animations.

```typescript
import { DataTable } from 'ui-components';

const columns = [
  { key: 'name', title: 'Name', sortable: true },
  { key: 'price', title: 'Price', format: 'currency' },
  { key: 'createdAt', title: 'Created', format: 'date' }
];

<DataTable
  data={products}
  columns={columns}
  onRowClick={handleRowClick}
/>
```

**Features:**
- Uses `formatters` from utils package for data display
- Uses `helpers.cloneDeep` for safe data manipulation
- Uses `SortConfig` and `FilterConfig` types from utils
- Integrated sorting and filtering

## Dependency Usage Examples

### Validation Integration
```typescript
// Input component using utils validators
import { validators, ValidationError } from 'utils';

const validateInput = useCallback((inputValue: string): ValidationError | null => {
  if (validation === 'email') {
    return validators.validateEmail(inputValue, label || 'Email');
  }
  // More validation logic...
}, [validation, label]);
```

### Formatting Integration
```typescript
// DataTable using utils formatters
import { formatters } from 'utils';

const formatCellValue = (value: any, column: DataTableColumn<T>) => {
  switch (column.format) {
    case 'currency':
      return formatters.currency(Number(value));
    case 'date':
      return formatters.date(value);
    // More formatting options...
  }
};
```

### Helper Functions Integration
```typescript
// Button component using utils helpers
import { generateId } from 'utils';

const Button: React.FC<ButtonProps> = ({ id, ...props }) => {
  const buttonId = id || generateId('button');
  // Component implementation...
};
```

## External Dependencies

- **React & React-DOM** - Peer dependencies (not bundled)
- **clsx** - Conditional CSS class names
- **framer-motion** - Animations and transitions

## Development Tools

- **Storybook** - Component documentation and testing
- **Jest & Testing Library** - Unit testing
- **TypeScript** - Type safety
- **ESLint & Prettier** - Code quality

## Best Practices

### ✅ Do
- Import specific functions from utils package to optimize bundle size
- Use utils types for consistency with other packages
- Leverage utils validators for form validation
- Use utils formatters for data display

### ❌ Don't
- Duplicate validation or formatting logic that exists in utils
- Create circular dependencies with other packages
- Import heavy external libraries without considering impact on consumers

## Testing with Dependencies

```typescript
// Component tests can mock utils package
jest.mock('utils', () => ({
  generateId: jest.fn(() => 'test-id'),
  validators: {
    validateEmail: jest.fn(),
  },
  formatters: {
    currency: jest.fn((value) => `$${value}`),
  },
}));
```

## Storybook Integration

Components are documented in Storybook with examples showing:
- Different props and variations
- Validation behavior (using utils validators)
- Formatting examples (using utils formatters)
- Error states and edge cases

## Adding New Components

When creating new components:

1. **Use utils package** - Leverage existing validation, formatting, and helpers
2. **Export properly** - Add to `index.ts` with proper type exports
3. **Add Storybook stories** - Document usage and variations
4. **Write tests** - Include tests for utils package integration
5. **Follow patterns** - Use established patterns from existing components

This package demonstrates how to build a component library that effectively leverages shared utilities while maintaining clean separation of concerns.