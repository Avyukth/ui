use leptos::*;
use wasm_bindgen::prelude::*;

/// Button variants matching shadcn/ui exactly
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl ButtonVariant {
    /// Get the EXACT Tailwind CSS classes from shadcn/ui
    fn classes(&self) -> &'static str {
        match self {
            Self::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            Self::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            Self::Outline => "border border-input hover:bg-accent hover:text-accent-foreground",
            Self::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            Self::Ghost => "hover:bg-accent hover:text-accent-foreground",
            Self::Link => "underline-offset-4 hover:underline text-primary",
        }
    }
}

/// Button sizes matching shadcn/ui exactly
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Default
    }
}

impl ButtonSize {
    /// Get the EXACT size classes from shadcn/ui
    fn classes(&self) -> &'static str {
        match self {
            Self::Default => "h-10 px-4 py-2",
            Self::Sm => "h-9 rounded-md px-3",
            Self::Lg => "h-11 rounded-md px-8",
            Self::Icon => "h-10 w-10",
        }
    }
}

/// A button component matching shadcn/ui design exactly
///
/// # Examples
///
/// ```rust
/// use button::{Button, ButtonVariant, ButtonSize};
/// use leptos::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         // Default button
///         <Button>"Click me"</Button>
///
///         // With variant and size
///         <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
///             "Large Outline"
///         </Button>
///
///         // Icon button
///         <Button variant=ButtonVariant::Outline size=ButtonSize::Icon>
///             <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
///                 <path d="M12 19V5"/><path d="m5 12 7-7 7 7"/>
///             </svg>
///         </Button>
///
///         // With icon and text
///         <Button>
///             <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
///                 <path d="M12 19V5"/><path d="m5 12 7-7 7 7"/>
///             </svg>
///             "Upload"
///         </Button>
///     }
/// }
/// ```
#[component]
pub fn Button(
    /// Button variant (default: Default)
    #[prop(optional)]
    variant: Option<ButtonVariant>,

    /// Button size (default: Default)
    #[prop(optional)]
    size: Option<ButtonSize>,

    /// Whether the button is disabled
    #[prop(optional)]
    disabled: Option<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,

    /// Optional click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn()>>,

    /// Button content (text, icons, or both)
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);

    // EXACT base classes from shadcn/ui
    let base_classes = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0";

    // Combine all classes exactly as shadcn/ui does
    let button_class = format!(
        "{} {} {}{}",
        base_classes,
        variant.classes(),
        size.classes(),
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    let handle_click = move |_| {
        if !disabled {
            if let Some(ref callback) = on_click {
                callback();
            }
        }
    };

    view! {
        <button
            class=button_class
            disabled=disabled
            on:click=handle_click
        >
            {children()}
        </button>
    }
}

/// Initialize the panic hook
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Mount a simple button (for JavaScript usage)
#[wasm_bindgen]
pub fn mount_button(text: &str) -> Result<(), JsValue> {
    let text = text.to_string();

    mount_to_body(move || {
        view! {
            <Button>{text.clone()}</Button>
        }
    });

    Ok(())
}

/// Mount a button with variant and size
#[wasm_bindgen]
pub fn mount_button_full(text: &str, variant: &str, size: &str) -> Result<(), JsValue> {
    let text = text.to_string();

    let variant = match variant.to_lowercase().as_str() {
        "destructive" => ButtonVariant::Destructive,
        "outline" => ButtonVariant::Outline,
        "secondary" => ButtonVariant::Secondary,
        "ghost" => ButtonVariant::Ghost,
        "link" => ButtonVariant::Link,
        _ => ButtonVariant::Default,
    };

    let size = match size.to_lowercase().as_str() {
        "sm" => ButtonSize::Sm,
        "lg" => ButtonSize::Lg,
        "icon" => ButtonSize::Icon,
        _ => ButtonSize::Default,
    };

    mount_to_body(move || {
        view! {
            <Button variant=variant size=size>{text.clone()}</Button>
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variant_default() {
        let variant = ButtonVariant::default();
        assert_eq!(variant, ButtonVariant::Default);
    }

    #[test]
    fn test_button_size_default() {
        let size = ButtonSize::default();
        assert_eq!(size, ButtonSize::Default);
    }

    #[test]
    fn test_variant_classes() {
        assert_eq!(ButtonVariant::Default.classes(), "bg-primary text-primary-foreground hover:bg-primary/90");
        assert_eq!(ButtonVariant::Outline.classes(), "border border-input hover:bg-accent hover:text-accent-foreground");
    }

    #[test]
    fn test_size_classes() {
        assert_eq!(ButtonSize::Default.classes(), "h-10 px-4 py-2");
        assert_eq!(ButtonSize::Icon.classes(), "h-10 w-10");
    }
}
