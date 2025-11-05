# Leptos shadcn/ui Components - Setup & Integration Guide

## Overview

This guide explains how to set up, build, and use the Leptos port of shadcn/ui components in your projects.

## Prerequisites

### Required Tools

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack for WebAssembly builds
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Trunk for serving Leptos apps (optional but recommended)
cargo install trunk

# Install cargo-leptos (alternative to Trunk)
cargo install cargo-leptos
```

### Node.js Dependencies (for Tailwind CSS)

```bash
# Install Node.js dependencies
npm install -D tailwindcss postcss autoprefixer tailwindcss-animate

# Initialize Tailwind
npx tailwindcss init -p
```

## Project Setup

### 1. Tailwind Configuration

Create or update `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: ['class'],
    content: [
        './index.html',
        './src/**/*.{rs,html}',
        './rust/components/*/src/**/*.rs'
    ],
    theme: {
        container: {
            center: true,
            padding: '2rem',
            screens: {
                '2xl': '1400px'
            }
        },
        extend: {
            colors: {
                border: 'hsl(var(--border))',
                input: 'hsl(var(--input))',
                ring: 'hsl(var(--ring))',
                background: 'hsl(var(--background))',
                foreground: 'hsl(var(--foreground))',
                primary: {
                    DEFAULT: 'hsl(var(--primary))',
                    foreground: 'hsl(var(--primary-foreground))'
                },
                secondary: {
                    DEFAULT: 'hsl(var(--secondary))',
                    foreground: 'hsl(var(--secondary-foreground))'
                },
                destructive: {
                    DEFAULT: 'hsl(var(--destructive))',
                    foreground: 'hsl(var(--destructive-foreground))'
                },
                muted: {
                    DEFAULT: 'hsl(var(--muted))',
                    foreground: 'hsl(var(--muted-foreground))'
                },
                accent: {
                    DEFAULT: 'hsl(var(--accent))',
                    foreground: 'hsl(var(--accent-foreground))'
                },
                popover: {
                    DEFAULT: 'hsl(var(--popover))',
                    foreground: 'hsl(var(--popover-foreground))'
                },
                card: {
                    DEFAULT: 'hsl(var(--card))',
                    foreground: 'hsl(var(--card-foreground))'
                }
            },
            borderRadius: {
                lg: 'var(--radius)',
                md: 'calc(var(--radius) - 2px)',
                sm: 'calc(var(--radius) - 4px)'
            },
            keyframes: {
                'accordion-down': {
                    from: { height: '0' },
                    to: { height: 'var(--radix-accordion-content-height)' }
                },
                'accordion-up': {
                    from: { height: 'var(--radix-accordion-content-height)' },
                    to: { height: '0' }
                }
            },
            animation: {
                'accordion-down': 'accordion-down 0.2s ease-out',
                'accordion-up': 'accordion-up 0.2s ease-out'
            }
        }
    },
    plugins: [require('tailwindcss-animate')]
};
```

### 2. CSS Variables

Create a CSS file (e.g., `style/app.css`) with theme variables:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 222.2 84% 4.9%;

    --card: 0 0% 100%;
    --card-foreground: 222.2 84% 4.9%;

    --popover: 0 0% 100%;
    --popover-foreground: 222.2 84% 4.9%;

    --primary: 222.2 47.4% 11.2%;
    --primary-foreground: 210 40% 98%;

    --secondary: 210 40% 96.1%;
    --secondary-foreground: 222.2 47.4% 11.2%;

    --muted: 210 40% 96.1%;
    --muted-foreground: 215.4 16.3% 46.9%;

    --accent: 210 40% 96.1%;
    --accent-foreground: 222.2 47.4% 11.2%;

    --destructive: 0 84.2% 60.2%;
    --destructive-foreground: 210 40% 98%;

    --border: 214.3 31.8% 91.4%;
    --input: 214.3 31.8% 91.4%;
    --ring: 222.2 84% 4.9%;

    --radius: 0.5rem;
  }

  .dark {
    --background: 222.2 84% 4.9%;
    --foreground: 210 40% 98%;

    --card: 222.2 84% 4.9%;
    --card-foreground: 210 40% 98%;

    --popover: 222.2 84% 4.9%;
    --popover-foreground: 210 40% 98%;

    --primary: 210 40% 98%;
    --primary-foreground: 222.2 47.4% 11.2%;

    --secondary: 217.2 32.6% 17.5%;
    --secondary-foreground: 210 40% 98%;

    --muted: 217.2 32.6% 17.5%;
    --muted-foreground: 215 20.2% 65.1%;

    --accent: 217.2 32.6% 17.5%;
    --accent-foreground: 210 40% 98%;

    --destructive: 0 62.8% 30.6%;
    --destructive-foreground: 210 40% 98%;

    --border: 217.2 32.6% 17.5%;
    --input: 217.2 32.6% 17.5%;
    --ring: 212.7 26.8% 83.9%;
  }
}

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground;
  }
}
```

### 3. Cargo Configuration

Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
leptos = { version = "0.7", features = ["csr"] }
tailwind_fuse = "0.3"
leptos-node-ref = "0.1"
leptos-struct-component = "0.1"
leptos-style = "0.1"
wasm-bindgen = "0.2"
console_error_panic_hook = "0.1"

# Import components as needed
button = { path = "./rust/components/button" }
badge = { path = "./rust/components/badge" }
input = { path = "./rust/components/input" }
# ... etc
```

Or use git dependencies:

```toml
[dependencies]
button = { git = "https://github.com/Avyukth/ui", path = "rust/components/button" }
```

### 4. HTML Template

Create `index.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Leptos shadcn/ui</title>
    <link data-trunk rel="css" href="style/app.css" />
</head>
<body>
    <div id="app"></div>
</body>
</html>
```

### 5. Build Configuration

#### Using Trunk

Create `Trunk.toml`:

```toml
[build]
target = "index.html"

[watch]
ignore = ["./target"]

[serve]
address = "127.0.0.1"
port = 8080
open = false
```

#### Using cargo-leptos

Create `Cargo.toml` configuration:

```toml
[package.metadata.leptos]
output-name = "app"
site-root = "target/site"
site-pkg-dir = "pkg"
style-file = "style/app.css"
tailwind-input-file = "style/app.css"
assets-dir = "public"
site-addr = "127.0.0.1:3000"
reload-port = 3001
browserquery = "defaults"
env = "DEV"
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
```

## Usage Examples

### Basic Component Usage

```rust
use leptos::prelude::*;
use button::{Button, ButtonVariant, ButtonSize};
use input::Input;
use label::Label;

#[component]
pub fn LoginForm() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    view! {
        <div class="max-w-md mx-auto p-6">
            <div class="space-y-4">
                <div class="space-y-2">
                    <Label r#for="email">"Email"</Label>
                    <Input
                        id="email"
                        r#type="email"
                        placeholder="email@example.com"
                        value=email
                        oninput=Callback::new(move |ev: ev::Event| {
                            set_email.set(event_target_value(&ev));
                        })
                    />
                </div>

                <div class="space-y-2">
                    <Label r#for="password">"Password"</Label>
                    <Input
                        id="password"
                        r#type="password"
                        value=password
                        oninput=Callback::new(move |ev: ev::Event| {
                            set_password.set(event_target_value(&ev));
                        })
                    />
                </div>

                <Button
                    class="w-full"
                    onclick=Callback::new(move |_| {
                        // Handle login
                    })
                >
                    "Sign In"
                </Button>
            </div>
        </div>
    }
}
```

### Using Compound Components

```rust
use card::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
use button::{Button, ButtonVariant};

#[component]
pub fn ProfileCard() -> impl IntoView {
    view! {
        <Card class="w-[350px]">
            <CardHeader>
                <CardTitle>"User Profile"</CardTitle>
                <CardDescription>"Manage your account settings"</CardDescription>
            </CardHeader>
            <CardContent>
                <p>"Profile content goes here"</p>
            </CardContent>
            <CardFooter class="flex justify-between">
                <Button variant=Signal::derive(|| ButtonVariant::Outline)>
                    "Cancel"
                </Button>
                <Button>"Save Changes"</Button>
            </CardFooter>
        </Card>
    }
}
```

### Working with State

```rust
use switch::Switch;
use checkbox::Checkbox;

#[component]
pub fn Settings() -> impl IntoView {
    let (notifications, set_notifications) = signal(false);
    let (dark_mode, set_dark_mode) = signal(false);
    let (terms, set_terms) = signal(false);

    view! {
        <div class="space-y-4">
            <div class="flex items-center justify-between">
                <Label r#for="notifications">"Enable Notifications"</Label>
                <Switch
                    id="notifications"
                    checked=notifications
                    onchange=Callback::new(move |_| {
                        set_notifications.update(|n| *n = !*n);
                    })
                />
            </div>

            <div class="flex items-center space-x-2">
                <Checkbox
                    id="terms"
                    checked=terms
                    onchange=Callback::new(move |_| {
                        set_terms.update(|t| *t = !*t);
                    })
                />
                <Label r#for="terms">"Accept terms and conditions"</Label>
            </div>
        </div>
    }
}
```

## Building Components

### Development Build

```bash
# Using Trunk
trunk serve

# Using cargo-leptos
cargo leptos serve

# Or build individual components
cd rust/components/button
wasm-pack build --target web --dev
```

### Production Build

```bash
# Using Trunk
trunk build --release

# Using cargo-leptos
cargo leptos build --release

# Individual components
cd rust/components/button
wasm-pack build --target web --release
```

## Testing

```bash
# Test all components
cd rust
cargo test --all

# Test specific component
cd rust/components/button
cargo test
```

## Component Customization

All components support class customization via the `class` prop:

```rust
<Button class="my-custom-class another-class">
    "Customized Button"
</Button>
```

Classes are merged using `tw_merge!`, so Tailwind utility classes will override base styles appropriately.

## Dark Mode

Toggle dark mode by adding/removing the `dark` class on the `<html>` or `<body>` element:

```rust
use leptos_dom::helpers::document;

fn toggle_dark_mode() {
    let doc = document();
    let html = doc.document_element().unwrap();
    let class_list = html.class_list();
    
    if class_list.contains("dark") {
        class_list.remove_1("dark").unwrap();
    } else {
        class_list.add_1("dark").unwrap();
    }
}
```

## Common Issues & Solutions

### 1. Tailwind classes not applying

- Ensure `content` paths in `tailwind.config.js` include all Rust files
- Rebuild Tailwind: `npx tailwindcss -i ./style/app.css -o ./dist/app.css`
- Check that CSS is properly linked in `index.html`

### 2. Component not found

- Verify component is listed in workspace `Cargo.toml` members
- Check import paths in your application
- Run `cargo clean && cargo build`

### 3. WASM build errors

- Update `wasm-bindgen`: `cargo install wasm-bindgen-cli`
- Ensure Rust toolchain is up to date: `rustup update`
- Check that `wasm32-unknown-unknown` target is installed: `rustup target add wasm32-unknown-unknown`

### 4. CSS variables not working

- Ensure CSS file with `:root` variables is loaded before Tailwind
- Check that variables are defined for both light and dark modes
- Verify `@layer base` directive is present

## Component Reference

### Available Components

| Component | Description | Key Features |
|-----------|-------------|--------------|
| Alert | Callout messages | Default, Destructive variants |
| Badge | Small status indicators | 4 variants, fully themeable |
| Button | Interactive button | 6 variants, 4 sizes, AsChild pattern |
| Card | Container with sections | Header, Title, Description, Content, Footer |
| Checkbox | Toggle checkbox | Accessible, animated |
| Input | Text input field | All HTML input types supported |
| Label | Form label | Accessibility enhanced |
| Progress | Progress indicator | Animated, percentage-based |
| Separator | Visual divider | Horizontal/Vertical |
| Skeleton | Loading placeholder | Pulse animation |
| Switch | Toggle switch | Accessible, smooth animation |
| Textarea | Multi-line text input | Auto-resize support |

### Component Patterns

All components follow these patterns:

1. **Signal-based props** for reactivity
2. **MaybeProp** for optional string/primitive props
3. **node_ref** for DOM access
4. **class** prop for customization
5. **Standard HTML attributes** where applicable
6. **Callback** for event handlers

## Next Steps

- Explore component examples in `rust/components/*/examples/`
- Check out the [Leptos documentation](https://leptos.dev/)
- Review [shadcn/ui documentation](https://ui.shadcn.com/) for design patterns
- Join the Leptos Discord for community support

## Contributing

To add new components:

1. Create component directory: `mkdir -p rust/components/newcomponent/src`
2. Follow existing patterns from Button/Badge
3. Add to workspace members in root `Cargo.toml`
4. Create examples and tests
5. Submit PR with documentation

## License

MIT - Same as shadcn/ui
