# UI Components

Reusable UI component library for CasaEats applications.

## Installation

This package is automatically linked by Knot when running `knot link`.

## Usage

```typescript
import { Button, Card, Modal } from '@ui-components';

function MyComponent() {
  return (
    <Card>
      <Button variant="primary" onClick={handleClick}>
        Click me
      </Button>
    </Card>
  );
}
```

## Development

```bash
# Start Storybook for component development
knot run dev

# Run tests
knot run test

# Build components
knot run build
```

## Components

- **Button** - Primary UI button with variants
- **Card** - Container component with shadow and padding
- **Modal** - Overlay component for dialogs
- **Input** - Form input with validation states
- **Avatar** - User profile picture component
- **Badge** - Small status or count indicator

## Scripts

- `dev` - Start Storybook development server
- `build` - Build component library
- `test` - Run component tests
- `lint` - Lint component code
- `format` - Format code with Prettier