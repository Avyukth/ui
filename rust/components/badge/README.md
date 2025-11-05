# Badge Component

A small badge component built with Leptos and compiled to WebAssembly, following shadcn/ui design patterns.

## Features

- **4 Variants**: Default, Secondary, Destructive, Outline
- **Type-safe**: Using Rust's type system with TwClass/TwVariant macros
- **Accessible**: Built with ARIA standards
- **Performant**: Compiled to WASM
- **Reactive**: Full Leptos signal support
- **Customizable**: Support for custom classes and node refs
- **Composition**: AsChild pattern support
- **Small**: ~15-20KB gzipped

## Installation

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Build

```bash
cd rust/components/badge
wasm-pack build --target web --out-dir pkg
```

## Usage

### In Rust/Leptos

```rust
use badge::{Badge, BadgeVariant};
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        // Default badge
        <Badge>
            "New"
        </Badge>

        // With variant
        <Badge variant=Signal::derive(|| BadgeVariant::Secondary)>
            "Beta"
        </Badge>

        // Destructive variant
        <Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
            "Deprecated"
        </Badge>

        // Outline variant
        <Badge variant=Signal::derive(|| BadgeVariant::Outline)>
            "Draft"
        </Badge>

        // With custom class
        <Badge class=Signal::derive(|| Some("ml-2".to_string()))>
            "Custom"
        </Badge>
    }
}
```

### In HTML/JavaScript

```html
<!DOCTYPE html>
<html>
<head>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body>
    <script type="module">
        import init, { mount_badge, mount_badge_variant } from './pkg/badge.js';

        async function run() {
            await init();

            // Simple badge
            mount_badge('New');

            // Badge with variant
            mount_badge_variant('Deprecated', 'destructive');
        }

        run();
    </script>
</body>
</html>
```

## API Reference

### Component Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `Signal<BadgeVariant>` | `Default` | Visual style variant |
| `class` | `Signal<Option<String>>` | `None` | Additional CSS classes |
| `node_ref` | `AnyNodeRef` | Default | Reference to the DOM node |
| `as_child` | `Option<Callback<BadgeChildProps, AnyView>>` | `None` | Render as child component |
| `children` | `Option<Children>` | `None` | Badge content |

### BadgeVariant

```rust
#[derive(PartialEq, TwVariant, Clone, Copy)]
pub enum BadgeVariant {
    Default,      // border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80
    Secondary,    // border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80
    Destructive,  // border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80
    Outline,      // text-foreground
}
```

### JavaScript Functions

#### `mount_badge(text: string)`

Mount a simple badge with default variant.

```javascript
mount_badge('New');
```

#### `mount_badge_variant(text: string, variant: string)`

Mount a badge with a specific variant.

```javascript
mount_badge_variant('Error', 'destructive');
```

**Variants**: `"default"`, `"secondary"`, `"destructive"`, `"outline"`

## Examples

The component includes two WASM-based examples:

### Basic Example

Simple demonstration of badge variants:

```bash
# Build the example
wasm-pack build --target web --out-dir pkg --example basic

# Serve and view
cd examples
python3 -m http.server 8080
# Visit http://localhost:8080?example=basic
```

### Comprehensive Example

Full showcase of all features including:
- All 4 variants
- Status indicators
- Tags and categories
- Notifications
- Number badges
- Interactive demo with state management

```bash
# Build the example
wasm-pack build --target web --out-dir pkg --example comprehensive

# Serve and view
cd examples
python3 -m http.server 8080
# Visit http://localhost:8080?example=comprehensive
```

## Common Use Cases

### Status Badges

```rust
<Badge variant=Signal::derive(|| BadgeVariant::Default)>
    "Active"
</Badge>
<Badge variant=Signal::derive(|| BadgeVariant::Secondary)>
    "Pending"
</Badge>
<Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
    "Error"
</Badge>
```

### Category Tags

```rust
<Badge variant=Signal::derive(|| BadgeVariant::Outline)>
    "React"
</Badge>
<Badge variant=Signal::derive(|| BadgeVariant::Outline)>
    "TypeScript"
</Badge>
<Badge variant=Signal::derive(|| BadgeVariant::Outline)>
    "Rust"
</Badge>
```

### Notification Count

```rust
<Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
    "3"
</Badge>
<Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
    "99+"
</Badge>
```

### Feature Labels

```rust
<Badge variant=Signal::derive(|| BadgeVariant::Default)>
    "New"
</Badge>
<Badge variant=Signal::derive(|| BadgeVariant::Secondary)>
    "Beta"
</Badge>
<Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
    "Deprecated"
</Badge>
```

## Styling

The badge uses exact shadcn/ui Tailwind CSS classes via the `tailwind_fuse` crate. Ensure your project includes Tailwind CSS with shadcn/ui colors:

```javascript
tailwind.config = {
    theme: {
        extend: {
            colors: {
                border: "hsl(214.3 31.8% 91.4%)",
                ring: "hsl(221.2 83.2% 53.3%)",
                background: "hsl(0 0% 100%)",
                foreground: "hsl(222.2 84% 4.9%)",
                primary: {
                    DEFAULT: "hsl(221.2 83.2% 53.3%)",
                    foreground: "hsl(210 40% 98%)",
                },
                secondary: {
                    DEFAULT: "hsl(210 40% 96.1%)",
                    foreground: "hsl(222.2 47.4% 11.2%)",
                },
                destructive: {
                    DEFAULT: "hsl(0 84.2% 60.2%)",
                    foreground: "hsl(210 40% 98%)",
                },
            }
        }
    }
}
```

## Development

### Build for Development

```bash
wasm-pack build --target web --dev --out-dir pkg
```

### Build for Production

```bash
wasm-pack build --target web --release --out-dir pkg
```

### Build Examples

```bash
# Build specific example
wasm-pack build --target web --out-dir pkg --example basic
wasm-pack build --target web --out-dir pkg --example comprehensive
```

### Run Examples

```bash
cd examples
python3 -m http.server 8080
# Visit http://localhost:8080
```

## Testing

The component includes comprehensive unit tests covering all variants and functionality:

```bash
cargo test
```

Tests include:
- Variant default values
- Variant equality
- All variants enumeration
- Clone trait implementation
- BadgeClass creation and usage

## Bundle Size

The WASM bundle is optimized for size using `wasm-opt -O4`:

- WASM module: ~40-50KB (uncompressed)
- WASM module: ~15-20KB (gzipped)
- JavaScript glue: ~5KB

## Browser Support

Supports all modern browsers with WebAssembly support:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Architecture

The badge component uses modern Leptos patterns:

- **TwClass/TwVariant**: Type-safe Tailwind class generation
- **Signals**: Reactive updates
- **Memos**: Optimized class computation
- **StructComponent**: Composition patterns
- **NodeRef**: DOM access when needed

## Accessibility

- Uses semantic HTML elements
- Proper text contrast ratios (WCAG AA compliant)
- Focus styles for keyboard navigation
- Screen reader friendly

## License

MIT

## Related

- [shadcn/ui Badge](https://ui.shadcn.com/docs/components/badge)
- [Leptos Documentation](https://leptos.dev/)
- [RustForWeb/shadcn-ui](https://github.com/RustForWeb/shadcn-ui)
- [tailwind_fuse](https://crates.io/crates/tailwind_fuse)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
