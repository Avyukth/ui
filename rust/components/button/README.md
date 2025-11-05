# Button Component

A flexible, accessible button component built with Leptos and compiled to WebAssembly, following shadcn/ui design patterns.

## Features

- **6 Variants**: Default, Destructive, Outline, Secondary, Ghost, Link
- **4 Sizes**: Small, Default, Large, Icon
- **Type-safe**: Using Rust's type system with TwClass/TwVariant macros
- **Accessible**: Built with ARIA standards
- **Performant**: Compiled to WASM
- **Reactive**: Full Leptos signal support
- **Customizable**: Support for custom classes and node refs
- **Composition**: AsChild pattern support

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
cd rust/components/button
wasm-pack build --target web --out-dir pkg
```

## Usage

### In Rust/Leptos

```rust
use button::{Button, ButtonVariant, ButtonSize};
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        // Basic button
        <Button>
            "Click me"
        </Button>

        // With variant
        <Button variant=Signal::derive(|| ButtonVariant::Destructive)>
            "Delete"
        </Button>

        // With size
        <Button size=Signal::derive(|| ButtonSize::Sm)>
            "Small"
        </Button>

        // With click handler
        <Button onclick=Callback::new(|_| {
            web_sys::console::log_1(&"Clicked!".into());
        })>
            "Interactive"
        </Button>

        // Disabled
        <Button disabled=Signal::derive(|| true)>
            "Disabled"
        </Button>

        // With custom class
        <Button class=Signal::derive(|| Some("my-custom-class".to_string()))>
            "Custom"
        </Button>

        // Icon button
        <Button size=Signal::derive(|| ButtonSize::Icon)>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M5 12h14"/>
                <path d="m12 5 7 7-7 7"/>
            </svg>
        </Button>
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
        import init, { mount_button, mount_button_full } from './pkg/button.js';

        async function run() {
            await init();

            // Simple button
            mount_button('Click Me');

            // Button with variant and size
            mount_button_full('Submit', 'outline', 'lg');
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
| `variant` | `Signal<ButtonVariant>` | `Default` | Visual style variant |
| `size` | `Signal<ButtonSize>` | `Default` | Size variant |
| `disabled` | `Signal<bool>` | `false` | Whether the button is disabled |
| `onclick` | `Option<Callback<MouseEvent>>` | `None` | Click event handler |
| `class` | `Signal<Option<String>>` | `None` | Additional CSS classes |
| `node_ref` | `AnyNodeRef` | Default | Reference to the DOM node |
| `as_child` | `Option<Callback<ButtonChildProps, AnyView>>` | `None` | Render as child component |
| `children` | `Option<Children>` | `None` | Button content |

### ButtonVariant

```rust
#[derive(PartialEq, TwVariant, Clone, Copy)]
pub enum ButtonVariant {
    Default,      // bg-primary text-primary-foreground hover:bg-primary/90
    Destructive,  // bg-destructive text-destructive-foreground hover:bg-destructive/90
    Outline,      // border border-input bg-background hover:bg-accent
    Secondary,    // bg-secondary text-secondary-foreground hover:bg-secondary/80
    Ghost,        // hover:bg-accent hover:text-accent-foreground
    Link,         // text-primary underline-offset-4 hover:underline
}
```

### ButtonSize

```rust
#[derive(PartialEq, TwVariant, Clone, Copy)]
pub enum ButtonSize {
    Default,  // h-10 px-4 py-2
    Sm,       // h-9 px-3 rounded-md
    Lg,       // h-11 px-8 rounded-md
    Icon,     // h-10 w-10
}
```

### JavaScript Functions

#### `mount_button(label: string)`

Mount a simple button with default variant and size.

```javascript
mount_button('Click Me');
```

#### `mount_button_full(label: string, variant: string, size: string)`

Mount a button with specific variant and size.

```javascript
mount_button_full('Submit', 'outline', 'lg');
```

**Variants**: `"default"`, `"destructive"`, `"outline"`, `"secondary"`, `"ghost"`, `"link"`

**Sizes**: `"default"`, `"sm"`, `"lg"`, `"icon"`

## Examples

The component includes two WASM-based examples:

### Basic Example

Simple demonstration of button variants:

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
- All 6 variants
- All 4 sizes
- Icon buttons
- Disabled states
- Interactive demo with state management

```bash
# Build the example
wasm-pack build --target web --out-dir pkg --example comprehensive

# Serve and view
cd examples
python3 -m http.server 8080
# Visit http://localhost:8080?example=comprehensive
```

## Styling

The button uses exact shadcn/ui Tailwind CSS classes via the `tailwind_fuse` crate. Ensure your project includes Tailwind CSS with shadcn/ui colors:

```javascript
tailwind.config = {
    theme: {
        extend: {
            colors: {
                border: "hsl(214.3 31.8% 91.4%)",
                input: "hsl(214.3 31.8% 91.4%)",
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
                muted: {
                    DEFAULT: "hsl(210 40% 96.1%)",
                    foreground: "hsl(215.4 16.3% 46.9%)",
                },
                accent: {
                    DEFAULT: "hsl(210 40% 96.1%)",
                    foreground: "hsl(222.2 47.4% 11.2%)",
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

The component includes comprehensive unit tests covering all variants, sizes, and functionality:

```bash
cargo test
```

Tests include:
- Variant default values
- Size default values
- Variant equality
- Size equality
- All variants enumeration
- All sizes enumeration
- Clone trait implementation

## Bundle Size

The WASM bundle is optimized for size using `wasm-opt -O4`:

- WASM module: ~50-80KB (uncompressed)
- WASM module: ~20-35KB (gzipped)
- JavaScript glue: ~5-10KB

## Browser Support

Supports all modern browsers with WebAssembly support:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Architecture

The button component uses modern Leptos patterns:

- **TwClass/TwVariant**: Type-safe Tailwind class generation
- **Signals**: Reactive updates
- **Memos**: Optimized class computation
- **StructComponent**: Composition patterns
- **NodeRef**: DOM access when needed

## License

MIT

## Related

- [shadcn/ui](https://ui.shadcn.com/)
- [Leptos Documentation](https://leptos.dev/)
- [RustForWeb/shadcn-ui](https://github.com/RustForWeb/shadcn-ui)
- [tailwind_fuse](https://crates.io/crates/tailwind_fuse)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
