use leptos::*;
use wasm_bindgen::prelude::*;

/// Skeleton variant shapes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkeletonVariant {
    Default,
    Circle,
    Text,
}

impl Default for SkeletonVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl SkeletonVariant {
    fn classes(&self) -> &'static str {
        match self {
            Self::Default => "rounded-md",
            Self::Circle => "rounded-full",
            Self::Text => "rounded-md h-4",
        }
    }
}

/// A skeleton loading placeholder
///
/// # Examples
///
/// ```rust
/// use skeleton::{Skeleton, SkeletonVariant};
/// use leptos::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Skeleton class="w-full h-12" />
///         <Skeleton variant=SkeletonVariant::Circle class="w-12 h-12" />
///         <Skeleton variant=SkeletonVariant::Text class="w-3/4" />
///     }
/// }
/// ```
#[component]
pub fn Skeleton(
    /// Skeleton variant
    #[prop(optional)]
    variant: Option<SkeletonVariant>,

    /// Additional CSS classes (use for width/height)
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();

    let base_classes = "animate-pulse bg-muted";
    let skeleton_class = format!(
        "{} {}{}",
        base_classes,
        variant.classes(),
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <div class=skeleton_class />
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn mount_skeleton(width: &str, height: &str) -> Result<(), JsValue> {
    let class = Some(format!("{} {}", width, height));

    mount_to_body(move || {
        view! { <Skeleton class=class.clone() /> }
    });

    Ok(())
}

#[wasm_bindgen]
pub fn mount_skeleton_circle(size: &str) -> Result<(), JsValue> {
    let class = Some(format!("{} {}", size, size));

    mount_to_body(move || {
        view! { <Skeleton variant=SkeletonVariant::Circle class=class.clone() /> }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_default() {
        assert_eq!(SkeletonVariant::default(), SkeletonVariant::Default);
    }

    #[test]
    fn test_variant_classes() {
        assert!(SkeletonVariant::Default.classes().contains("rounded-md"));
        assert!(SkeletonVariant::Circle.classes().contains("rounded-full"));
        assert!(SkeletonVariant::Text.classes().contains("h-4"));
    }
}
