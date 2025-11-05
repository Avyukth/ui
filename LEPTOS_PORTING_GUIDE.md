# Leptos shadcn/ui Porting Guide - Complete Documentation

## Executive Summary

This document provides a comprehensive analysis of porting shadcn/ui components from React to Leptos WASM. It includes:

1. **Pattern Analysis**: Dissection of existing Leptos ports
2. **Component Inventory**: List of completed vs. missing components
3. **Implementation Examples**: 8 new component implementations
4. **Setup & Integration**: Complete development guide

Based on analysis of [Avyukth/shadcn-ui](https://github.com/Avyukth/shadcn-ui/tree/main/book-examples/leptos).

---

## Part 1: Dissection of Porting Patterns

### Architecture Overview

The Leptos port follows these architectural principles:

```
Component Structure:
├── TwClass/TwVariant macros for type-safe styling
├── Signal-based props for reactivity
├── leptos_struct_component for composition patterns
├── Standard HTML attributes support
└── Event handlers via Callback<T>
```

### Pattern 1: Simple Variant-Based Components

**Example: Badge Component**

```rust
// 1. Define class structure
#[derive(TwClass)]
#[tw(class = "inline-flex items-center rounded-full border px-2.5 py-0.5...")]
pub struct BadgeClass {
    pub variant: BadgeVariant,
}

// 2. Define variants with Tailwind classes
#[derive(PartialEq, TwVariant)]
pub enum BadgeVariant {
    #[tw(default, class = "border-transparent bg-primary...")]
    Default,
    #[tw(class = "border-transparent bg-secondary...")]
    Secondary,
}

// 3. Component implementation
#[component]
pub fn Badge(
    #[prop(into, optional)] variant: Signal<BadgeVariant>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    // Memoize class computation
    let class = Memo::new(move |_| {
        BadgeClass { variant: variant.get() }
            .with_class(class.get().unwrap_or_default())
    });

    view! {
        <div node_ref=node_ref class=class>
            {children()}
        </div>
    }
}
```

**Key Insights:**
- `TwClass` macro generates type-safe Tailwind class builders
- `TwVariant` maps enums to CSS classes at compile time
- `Memo` optimizes class recalculation
- `with_class()` merges custom classes using `tw_merge!` logic

### Pattern 2: Interactive Components with AsChild

**Example: Button Component**

```rust
// 1. Define StructComponent for composition
#[derive(Clone, StructComponent)]
#[struct_component(tag = "button")]
pub struct ButtonChildProps {
    pub node_ref: AnyNodeRef,
    pub class: Signal<String>,
    pub disabled: Signal<bool>,
    pub onclick: Option<Callback<MouseEvent>>,
    // ... all button HTML attributes
}

// 2. Component with AsChild pattern
#[component]
pub fn Button(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] as_child: Option<Callback<ButtonChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
    // ... other props
) -> impl IntoView {
    let child_props = ButtonChildProps {
        class: class.into(),
        disabled,
        onclick,
        // ...
    };

    // Conditional rendering based on as_child
    if let Some(as_child) = as_child.as_ref() {
        as_child.run(child_props)  // Render prop pattern
    } else {
        child_props.render(children)  // Default rendering
    }
}
```

**Key Insights:**
- `StructComponent` enables flexible composition
- `as_child` allows rendering as different element types
- All HTML attributes are preserved
- Type-safe event handlers via `Callback<T>`

### Pattern 3: Compound Components

**Example: Card Component Family**

```rust
// Main container
#[component]
pub fn Card(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || tw_merge!(
                "rounded-lg border bg-card text-card-foreground shadow-sm",
                class.get()
            )
        >
            {children()}
        </div>
    }
}

// Related sub-components
#[component]
pub fn CardHeader(/* ... */) -> impl IntoView { /* ... */ }

#[component]
pub fn CardTitle(/* ... */) -> impl IntoView { /* ... */ }

#[component]
pub fn CardContent(/* ... */) -> impl IntoView { /* ... */ }

#[component]
pub fn CardFooter(/* ... */) -> impl IntoView { /* ... */ }
```

**Key Insights:**
- Each sub-component is independent
- Composition via `children`
- Consistent `tw_merge!` pattern for customization
- No parent-child communication needed (slots pattern)

### Common Implementation Patterns

#### 1. Props Handling

```rust
// Signal for reactive values
variant: Signal<ButtonVariant>

// MaybeProp for optional primitives
placeholder: MaybeProp<String>

// Signal for optional booleans
disabled: Signal<bool>

// Option<Callback> for event handlers
onclick: Option<Callback<MouseEvent>>

// AnyNodeRef for DOM access
node_ref: AnyNodeRef

// Children for content
children: Children
```

#### 2. Reactivity

```rust
// Create memoized computed values
let class = Memo::new(move |_| {
    ComponentClass {
        variant: variant.get(),
        size: size.get(),
    }
    .with_class(class.get().unwrap_or_default())
});

// Access reactive values in view
view! {
    <div class=class>  // Direct signal binding
        {move || some_signal.get()}  // Closure for derived values
    </div>
}
```

#### 3. Accessibility

```rust
view! {
    <button
        role="button"
        aria-label=move || label.get()
        aria-checked=move || checked.get().to_string()
        aria-disabled=move || disabled.get().to_string()
        tabindex="0"
    >
        {children()}
    </button>
}
```

#### 4. Event Handling

```rust
// Define callback prop
#[prop(into, optional)] onclick: Option<Callback<MouseEvent>>

// Use in view
view! {
    <button
        on:click=move |ev| {
            if let Some(handler) = onclick.as_ref() {
                handler.run(ev);
            }
        }
    >
        "Click me"
    </button>
}
```

### Dependencies

All components require:

```toml
[dependencies]
leptos = { version = "0.7", features = ["csr"] }
tailwind_fuse = "0.3"         # TwClass/TwVariant macros
leptos-node-ref = "0.1"       # AnyNodeRef for DOM access
leptos-struct-component = "0.1" # StructComponent for composition
leptos-style = "0.1"          # Style type for inline styles
wasm-bindgen = "0.2"
console_error_panic_hook = "0.1"
```

---

## Part 2: Component Inventory

### Completed Components (4 + 8 new = 12)

#### From Original Repository
1. ✅ **Alert** - Callout messages with variants
2. ✅ **Badge** - Status indicators
3. ✅ **Button** - Interactive button with 6 variants, AsChild pattern
4. ✅ **Card** - Container with Header/Title/Description/Content/Footer

#### Newly Implemented (in this guide)
5. ✅ **Checkbox** - Toggle checkbox with accessibility
6. ✅ **Input** - Text input field with full HTML support
7. ✅ **Label** - Form label with peer styling
8. ✅ **Progress** - Progress indicator with percentage
9. ✅ **Separator** - Horizontal/Vertical divider
10. ✅ **Skeleton** - Loading placeholder with animation
11. ✅ **Switch** - Toggle switch with smooth animation
12. ✅ **Textarea** - Multi-line text input

### Missing Components (44)

#### High Priority - Form & Input
- [ ] Radio Group
- [ ] Select
- [ ] Combobox
- [ ] Command
- [ ] Form
- [ ] Field
- [ ] Input Group
- [ ] Input OTP

#### High Priority - Layout & Navigation
- [ ] Accordion
- [ ] Tabs
- [ ] Navigation Menu
- [ ] Breadcrumb (exists but needs RustForWeb rewrite)
- [ ] Menubar
- [ ] Context Menu
- [ ] Dropdown Menu

#### High Priority - Overlays
- [ ] Dialog
- [ ] Alert Dialog
- [ ] Sheet
- [ ] Drawer
- [ ] Popover
- [ ] Tooltip
- [ ] Hover Card

#### Medium Priority - Data Display
- [ ] Table
- [ ] Data Table
- [ ] Avatar (exists but needs rewrite)
- [ ] Aspect Ratio
- [ ] Scroll Area
- [ ] Resizable

#### Medium Priority - Feedback
- [ ] Toast
- [ ] Sonner
- [ ] Spinner

#### Medium Priority - Advanced
- [ ] Calendar
- [ ] Date Picker
- [ ] Carousel
- [ ] Slider
- [ ] Toggle
- [ ] Toggle Group
- [ ] Collapsible
- [ ] Pagination

#### Lower Priority - Utility
- [ ] Button Group
- [ ] Empty State
- [ ] Item
- [ ] KBD

---

## Part 3: New Component Implementations

Complete implementations are available in:
- `/home/user/ui/rust/components/input/src/lib.rs`
- `/home/user/ui/rust/components/label/src/lib.rs`
- `/home/user/ui/rust/components/separator/src/lib.rs`
- `/home/user/ui/rust/components/checkbox/src/lib.rs`
- `/home/user/ui/rust/components/skeleton/src/lib.rs`
- `/home/user/ui/rust/components/textarea/src/lib.rs`
- `/home/user/ui/rust/components/switch/src/lib.rs`
- `/home/user/ui/rust/components/progress/src/lib.rs`

### Implementation Highlights

#### Input Component
```rust
// Supports all HTML input types
<Input
    r#type="email"
    placeholder="email@example.com"
    value=email_signal
    oninput=Callback::new(move |ev| { /* handle */ })
/>
```

#### Checkbox Component
```rust
// Accessible with ARIA attributes
<Checkbox
    checked=checked_signal
    onchange=Callback::new(move |_| {
        set_checked.update(|c| *c = !*c);
    })
/>
```

#### Switch Component
```rust
// Animated toggle with data-state
<Switch
    checked=enabled_signal
    onchange=Callback::new(move |_| {
        set_enabled.update(|e| *e = !*e);
    })
/>
```

#### Progress Component
```rust
// Percentage-based progress bar
<Progress
    value=progress_signal
    max=Signal::derive(|| 100.0)
/>
```

---

## Part 4: Setup & Integration

Complete setup guide available at `/home/user/ui/rust/SETUP_GUIDE.md`

### Quick Start

```bash
# 1. Install prerequisites
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install trunk
npm install -D tailwindcss tailwindcss-animate

# 2. Create project structure
mkdir my-leptos-app && cd my-leptos-app
cargo init --lib

# 3. Add dependencies to Cargo.toml
```

```toml
[dependencies]
leptos = { version = "0.7", features = ["csr"] }
tailwind_fuse = "0.3"
leptos-node-ref = "0.1"
leptos-struct-component = "0.1"
leptos-style = "0.1"

# Add components
button = { git = "https://github.com/Avyukth/ui", path = "rust/components/button" }
input = { git = "https://github.com/Avyukth/ui", path = "rust/components/input" }
```

### Tailwind Setup

1. Create `tailwind.config.js` with shadcn/ui theme
2. Create `style/app.css` with CSS variables for light/dark mode
3. Configure Trunk to process CSS

See full configuration in SETUP_GUIDE.md.

### Usage Example

```rust
use leptos::prelude::*;
use button::{Button, ButtonVariant};
use input::Input;
use card::{Card, CardHeader, CardTitle, CardContent};

#[component]
pub fn App() -> impl IntoView {
    let (email, set_email) = signal(String::new());

    view! {
        <Card class="w-96">
            <CardHeader>
                <CardTitle>"Welcome"</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4">
                <Input
                    placeholder="Enter email"
                    value=email
                    oninput=Callback::new(move |ev: ev::Event| {
                        set_email.set(event_target_value(&ev));
                    })
                />
                <Button class="w-full">
                    "Submit"
                </Button>
            </CardContent>
        </Card>
    }
}
```

---

## Part 5: Advanced Topics

### Theming

All components use CSS variables for theming:

```css
:root {
  --primary: 222.2 47.4% 11.2%;
  --secondary: 210 40% 96.1%;
  /* ... */
}

.dark {
  --primary: 210 40% 98%;
  --secondary: 217.2 32.6% 17.5%;
  /* ... */
}
```

Toggle theme by adding/removing `dark` class on `<html>`.

### Custom Styling

All components accept `class` prop:

```rust
<Button class="custom-class">
    "Styled Button"
</Button>
```

Classes merge intelligently via `tw_merge!`:
- Later classes override earlier ones
- Tailwind utility classes properly conflict-resolved

### Accessibility Checklist

✅ ARIA roles and attributes
✅ Keyboard navigation support
✅ Focus management
✅ Screen reader compatibility
✅ High contrast mode support
✅ Reduced motion preferences

### Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_default() {
        assert_eq!(ButtonVariant::default(), ButtonVariant::Default);
    }

    #[test]
    fn test_class_generation() {
        let class = ButtonClass {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Default,
        };
        let classes = class.as_class();
        assert!(classes.contains("bg-primary"));
    }
}
```

---

## Part 6: Component Porting Checklist

When porting a new component:

- [ ] Create component directory: `rust/components/{name}/`
- [ ] Create `Cargo.toml` with workspace dependencies
- [ ] Analyze React component props and variants
- [ ] Define Rust enums for variants with `TwVariant`
- [ ] Define class structure with `TwClass`
- [ ] Implement component with proper props
- [ ] Add ARIA attributes for accessibility
- [ ] Add event handlers as `Option<Callback<T>>`
- [ ] Support `class` prop customization
- [ ] Add `node_ref` for DOM access
- [ ] Implement `as_child` pattern if needed
- [ ] Write unit tests
- [ ] Create usage examples
- [ ] Add to workspace members
- [ ] Update documentation

---

## Part 7: Next Component Roadmap

### Priority 1: Form Components (Week 1-2)

1. **Select** - Dropdown selection
   - Complex: Requires portal/overlay positioning
   - Dependencies: Popover
   
2. **Radio Group** - Single selection from options
   - Medium complexity
   - Similar to Checkbox pattern

3. **Form** - Form container with validation
   - Requires context API for field coordination

### Priority 2: Overlay Components (Week 3-4)

4. **Dialog** - Modal dialog
   - Requires portal rendering
   - Focus trap management
   
5. **Popover** - Floating content
   - Positioning logic
   - Click-outside detection

6. **Tooltip** - Hover tooltips
   - Simpler than Popover
   - Accessibility considerations

### Priority 3: Layout Components (Week 5-6)

7. **Accordion** - Collapsible sections
   - Animation support needed
   - Multiple vs single expand modes

8. **Tabs** - Tabbed interface
   - State management for active tab
   - Keyboard navigation

9. **Table** - Data table
   - Complex with sorting/filtering
   - May need separate Data Table component

---

## Part 8: Resources

### Documentation
- [Leptos Book](https://leptos.dev/)
- [shadcn/ui Docs](https://ui.shadcn.com/)
- [Tailwind CSS](https://tailwindcss.com/)
- [tailwind_fuse](https://github.com/your-repo/tailwind-fuse)

### Community
- [Leptos Discord](https://discord.gg/leptos)
- [GitHub Discussions](https://github.com/Avyukth/ui/discussions)

### Tools
- [Trunk](https://trunkrs.dev/) - WASM bundler
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) - Build tool
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) - WASM packaging

---

## Conclusion

This guide provides a complete framework for porting shadcn/ui components to Leptos. Key takeaways:

1. **Pattern Consistency**: Follow TwClass/TwVariant pattern for all components
2. **Type Safety**: Leverage Rust's type system for props and variants
3. **Reactivity**: Use Signals and Memos appropriately
4. **Accessibility**: Maintain ARIA standards from React version
5. **Customization**: Always support `class` prop with `tw_merge!`

With 12 components complete and patterns established, the remaining 44 components can be systematically ported following these blueprints.

**Estimated completion**: 6-8 weeks with 1 developer, or 2-3 weeks with a small team.

