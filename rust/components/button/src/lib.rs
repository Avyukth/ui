use leptos::{ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::{StructComponent, struct_component};
use leptos_style::Style;
use tailwind_fuse::*;
use wasm_bindgen::prelude::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(PartialEq, TwVariant, Clone, Copy)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(PartialEq, TwVariant, Clone, Copy)]
pub enum ButtonSize {
    #[tw(default, class = "h-10 px-4 py-2")]
    Default,
    #[tw(class = "h-9 rounded-md px-3")]
    Sm,
    #[tw(class = "h-11 rounded-md px-8")]
    Lg,
    #[tw(class = "h-10 w-10")]
    Icon,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, StructComponent)]
#[struct_component(tag = "button")]
pub struct ButtonChildProps {
    pub node_ref: AnyNodeRef,

    // Global attributes
    pub autofocus: Signal<bool>,
    pub class: Signal<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,

    // Attributes from `button`
    pub disabled: Signal<bool>,
    pub form: MaybeProp<String>,
    pub formaction: MaybeProp<String>,
    pub formenctype: MaybeProp<String>,
    pub formmethod: MaybeProp<String>,
    pub formnovalidate: Signal<bool>,
    pub formtarget: MaybeProp<String>,
    pub name: MaybeProp<String>,
    pub r#type: MaybeProp<String>,
    pub value: MaybeProp<String>,

    // Event handler attributes
    pub onclick: Option<Callback<MouseEvent>>,
}

/// Button component matching shadcn/ui design exactly
///
/// # Examples
///
/// ```rust
/// use button::{Button, ButtonVariant, ButtonSize};
/// use leptos::prelude::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         // Default button
///         <Button>"Click me"</Button>
///
///         // With variant and size
///         <Button
///             variant=Signal::derive(move || ButtonVariant::Outline)
///             size=Signal::derive(move || ButtonSize::Lg)
///         >
///             "Large Outline"
///         </Button>
///
///         // Icon button
///         <Button
///             variant=Signal::derive(move || ButtonVariant::Outline)
///             size=Signal::derive(move || ButtonSize::Icon)
///         >
///             <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24">
///                 <path d="M12 19V5"/><path d="m5 12 7-7 7 7"/>
///             </svg>
///         </Button>
///     }
/// }
/// ```
#[component]
pub fn Button(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] size: Signal<ButtonSize>,

    // Global attributes
    #[prop(into, optional)] autofocus: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    // Attributes from `button`
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] formaction: MaybeProp<String>,
    #[prop(into, optional)] formenctype: MaybeProp<String>,
    #[prop(into, optional)] formmethod: MaybeProp<String>,
    #[prop(into, optional)] formnovalidate: Signal<bool>,
    #[prop(into, optional)] formtarget: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] r#type: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,

    // Event handler attributes
    #[prop(into, optional)] onclick: Option<Callback<MouseEvent>>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: Option<Callback<ButtonChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        ButtonClass {
            variant: variant.get(),
            size: size.get(),
        }
        .with_class(class.get().unwrap_or_default())
    });

    let child_props = ButtonChildProps {
        node_ref,

        // Global attributes
        autofocus,
        class: class.into(),
        id,
        style,

        // Attributes from `button`
        disabled,
        form,
        formaction,
        formenctype,
        formmethod,
        formnovalidate,
        formtarget,
        name,
        r#type,
        value,

        // Event handler attributes
        onclick,
    };

    if let Some(as_child) = as_child.as_ref() {
        as_child.run(child_props)
    } else {
        child_props.render(children)
    }
}

/// Initialize the panic hook
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Mount a simple button for JavaScript usage
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

/// Mount a button with variant and size for JavaScript usage
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
            <Button
                variant=Signal::derive(move || variant)
                size=Signal::derive(move || size)
            >
                {text.clone()}
            </Button>
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
    fn test_button_variant_equality() {
        assert_eq!(ButtonVariant::Default, ButtonVariant::Default);
        assert_ne!(ButtonVariant::Default, ButtonVariant::Destructive);
        assert_ne!(ButtonVariant::Outline, ButtonVariant::Secondary);
    }

    #[test]
    fn test_button_size_equality() {
        assert_eq!(ButtonSize::Default, ButtonSize::Default);
        assert_ne!(ButtonSize::Sm, ButtonSize::Lg);
        assert_ne!(ButtonSize::Icon, ButtonSize::Default);
    }

    #[test]
    fn test_all_variants_exist() {
        let variants = vec![
            ButtonVariant::Default,
            ButtonVariant::Destructive,
            ButtonVariant::Outline,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
        ];
        assert_eq!(variants.len(), 6);
    }

    #[test]
    fn test_all_sizes_exist() {
        let sizes = vec![
            ButtonSize::Default,
            ButtonSize::Sm,
            ButtonSize::Lg,
            ButtonSize::Icon,
        ];
        assert_eq!(sizes.len(), 4);
    }

    #[test]
    fn test_variant_clone() {
        let variant = ButtonVariant::Outline;
        let cloned = variant;
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_size_clone() {
        let size = ButtonSize::Lg;
        let cloned = size;
        assert_eq!(size, cloned);
    }
}
