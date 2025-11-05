use leptos::*;
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::StructComponent;
use tailwind_fuse::{tw_join, tw_merge, AsTailwindClass, TwClass, TwVariant};
use wasm_bindgen::prelude::*;

/// Badge variant type matching shadcn/ui design system
#[derive(PartialEq, TwVariant, Clone, Copy)]
pub enum BadgeVariant {
    #[tw(
        default,
        class = "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80"
    )]
    Default,
    #[tw(class = "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80")]
    Destructive,
    #[tw(class = "text-foreground")]
    Outline,
}

impl Default for BadgeVariant {
    fn default() -> Self {
        Self::Default
    }
}

/// Badge component class structure using TwClass
#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
)]
pub struct BadgeClass {
    pub variant: BadgeVariant,
}

/// Props for Badge child components using StructComponent pattern
#[derive(StructComponent)]
#[component]
pub struct BadgeChildProps {
    #[component_prop(rename = "node_ref")]
    pub node_ref: AnyNodeRef,
    #[component_prop(rename = "attrs")]
    pub attrs: Vec<(&'static str, Attribute)>,
    #[component_prop]
    pub children: ViewFn,
}

/// A badge component for displaying tags, categories, or status
///
/// Displays a small badge with various visual styles.
///
/// # Examples
///
/// ```rust
/// use badge::{Badge, BadgeVariant};
/// use leptos::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         // Default badge
///         <Badge>"New"</Badge>
///
///         // With variant
///         <Badge variant=Signal::derive(|| BadgeVariant::Secondary)>
///             "Beta"
///         </Badge>
///
///         // Destructive variant
///         <Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
///             "Deprecated"
///         </Badge>
///     }
/// }
/// ```
#[component]
pub fn Badge(
    /// Badge visual variant
    #[prop(into, optional)]
    variant: Signal<BadgeVariant>,
    /// Additional CSS classes to merge
    #[prop(into, optional)]
    class: Signal<Option<String>>,
    /// Reference to the DOM node
    #[prop(into, optional)]
    node_ref: AnyNodeRef,
    /// Render as a child component for composition
    #[prop(optional)]
    as_child: Option<Callback<BadgeChildProps, AnyView>>,
    /// Badge content
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        BadgeClass {
            variant: variant.get(),
        }
        .with_class(class.get().unwrap_or_default())
    });

    let attrs = move || vec![];

    let children = StoredValue::new(children);
    let children_fn = ViewFn::new(move || {
        if let Some(children_fn) = children.get_value() {
            children_fn().into_any()
        } else {
            ().into_any()
        }
    });

    if let Some(as_child) = as_child {
        (as_child)(BadgeChildProps {
            node_ref,
            attrs: attrs(),
            children: children_fn,
        })
        .into_any()
    } else {
        view! {
            <div node_ref=node_ref class=class>
                {children_fn.run()}
            </div>
        }
        .into_any()
    }
}

/// Initialize the panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Mount a simple badge component to the body
#[wasm_bindgen]
pub fn mount_badge(text: &str) -> Result<(), JsValue> {
    let text = text.to_string();

    mount_to_body(move || {
        view! {
            <Badge>{text.clone()}</Badge>
        }
    });

    Ok(())
}

/// Mount a badge with a specific variant
#[wasm_bindgen]
pub fn mount_badge_variant(text: &str, variant: &str) -> Result<(), JsValue> {
    let text = text.to_string();
    let variant = match variant.to_lowercase().as_str() {
        "secondary" => BadgeVariant::Secondary,
        "destructive" => BadgeVariant::Destructive,
        "outline" => BadgeVariant::Outline,
        _ => BadgeVariant::Default,
    };

    mount_to_body(move || {
        view! {
            <Badge variant=Signal::derive(move || variant)>
                {text.clone()}
            </Badge>
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_variant_default() {
        let variant = BadgeVariant::default();
        assert_eq!(variant, BadgeVariant::Default);
    }

    #[test]
    fn test_badge_variant_equality() {
        assert_eq!(BadgeVariant::Default, BadgeVariant::Default);
        assert_eq!(BadgeVariant::Secondary, BadgeVariant::Secondary);
        assert_eq!(BadgeVariant::Destructive, BadgeVariant::Destructive);
        assert_eq!(BadgeVariant::Outline, BadgeVariant::Outline);
        assert_ne!(BadgeVariant::Default, BadgeVariant::Secondary);
    }

    #[test]
    fn test_all_variants_exist() {
        let variants = vec![
            BadgeVariant::Default,
            BadgeVariant::Secondary,
            BadgeVariant::Destructive,
            BadgeVariant::Outline,
        ];

        assert_eq!(variants.len(), 4, "Should have exactly 4 variants");
    }

    #[test]
    fn test_variant_clone() {
        let variant = BadgeVariant::Default;
        let cloned = variant.clone();
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_badge_class_creation() {
        let badge_class = BadgeClass {
            variant: BadgeVariant::Default,
        };
        // Test that the class structure can be created
        let _ = badge_class.as_class();
    }

    #[test]
    fn test_all_variant_values() {
        // Ensure all variants can be instantiated
        let _default = BadgeVariant::Default;
        let _secondary = BadgeVariant::Secondary;
        let _destructive = BadgeVariant::Destructive;
        let _outline = BadgeVariant::Outline;
    }

    #[test]
    fn test_variant_partial_eq() {
        assert!(BadgeVariant::Default == BadgeVariant::Default);
        assert!(BadgeVariant::Secondary != BadgeVariant::Default);
    }

    #[test]
    fn test_badge_class_with_variant() {
        let class_default = BadgeClass {
            variant: BadgeVariant::Default,
        };
        let class_secondary = BadgeClass {
            variant: BadgeVariant::Secondary,
        };

        // Verify classes can be generated
        let _default_str = class_default.as_class();
        let _secondary_str = class_secondary.as_class();
    }
}
