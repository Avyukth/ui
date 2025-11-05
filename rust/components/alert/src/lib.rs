use leptos::*;
use wasm_bindgen::prelude::*;

/// Alert variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertVariant {
    Default,
    Destructive,
}

impl Default for AlertVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl AlertVariant {
    fn classes(&self) -> &'static str {
        match self {
            Self::Default => "bg-background text-foreground",
            Self::Destructive => "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive",
        }
    }
}

/// Alert component for displaying important messages
#[component]
pub fn Alert(
    /// Alert variant
    #[prop(optional)]
    variant: Option<AlertVariant>,

    /// Alert content
    children: Children,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();

    let base_classes = "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground";
    let alert_class = format!(
        "{} {}{}",
        base_classes,
        variant.classes(),
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <div class=alert_class role="alert">
            {children()}
        </div>
    }
}

/// Alert title
#[component]
pub fn AlertTitle(
    /// Title text
    #[prop(into)]
    text: String,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = "mb-1 font-medium leading-none tracking-tight";
    let title_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <h5 class=title_class>{text}</h5>
    }
}

/// Alert description
#[component]
pub fn AlertDescription(
    /// Description text
    #[prop(into)]
    text: String,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = "text-sm [&_p]:leading-relaxed";
    let desc_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <div class=desc_class>{text}</div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn mount_alert(title: &str, description: &str, variant: &str) -> Result<(), JsValue> {
    let title = title.to_string();
    let description = description.to_string();
    let variant = match variant.to_lowercase().as_str() {
        "destructive" => AlertVariant::Destructive,
        _ => AlertVariant::Default,
    };

    mount_to_body(move || {
        view! {
            <Alert variant=variant>
                <AlertTitle text=title.clone() />
                <AlertDescription text=description.clone() />
            </Alert>
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_default() {
        assert_eq!(AlertVariant::default(), AlertVariant::Default);
    }

    #[test]
    fn test_variant_classes() {
        assert!(AlertVariant::Default.classes().contains("bg-background"));
        assert!(AlertVariant::Destructive.classes().contains("destructive"));
    }
}
