use leptos::*;
use wasm_bindgen::prelude::*;

/// Main card container
#[component]
pub fn Card(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,

    /// Card content
    children: Children,
) -> impl IntoView {
    let base_classes = "rounded-lg border bg-card text-card-foreground shadow-sm";
    let card_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <div class=card_class>
            {children()}
        </div>
    }
}

/// Card header section
#[component]
pub fn CardHeader(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,

    /// Header content
    children: Children,
) -> impl IntoView {
    let base_classes = "flex flex-col space-y-1.5 p-6";
    let header_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <div class=header_class>
            {children()}
        </div>
    }
}

/// Card title
#[component]
pub fn CardTitle(
    /// Title text
    #[prop(into)]
    text: String,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = "text-2xl font-semibold leading-none tracking-tight";
    let title_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <h3 class=title_class>{text}</h3>
    }
}

/// Card description
#[component]
pub fn CardDescription(
    /// Description text
    #[prop(into)]
    text: String,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = "text-sm text-muted-foreground";
    let desc_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <p class=desc_class>{text}</p>
    }
}

/// Card content section
#[component]
pub fn CardContent(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,

    /// Content
    children: Children,
) -> impl IntoView {
    let base_classes = "p-6 pt-0";
    let content_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <div class=content_class>
            {children()}
        </div>
    }
}

/// Card footer section
#[component]
pub fn CardFooter(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,

    /// Footer content
    children: Children,
) -> impl IntoView {
    let base_classes = "flex items-center p-6 pt-0";
    let footer_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <div class=footer_class>
            {children()}
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn mount_card_simple(title: &str, description: &str, content: &str) -> Result<(), JsValue> {
    let title = title.to_string();
    let description = description.to_string();
    let content = content.to_string();

    mount_to_body(move || {
        view! {
            <Card>
                <CardHeader>
                    <CardTitle text=title.clone() />
                    <CardDescription text=description.clone() />
                </CardHeader>
                <CardContent>
                    <p>{content.clone()}</p>
                </CardContent>
            </Card>
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_components_compile() {
        assert!(true);
    }
}
