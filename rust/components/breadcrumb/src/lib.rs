use leptos::*;
use wasm_bindgen::prelude::*;

/// Breadcrumb navigation container
#[component]
pub fn Breadcrumb(
    /// Breadcrumb items
    children: Children,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    view! {
        <nav aria-label="breadcrumb" class=class>
            {children()}
        </nav>
    }
}

/// Breadcrumb list
#[component]
pub fn BreadcrumbList(
    /// List items
    children: Children,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5";
    let list_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <ol class=list_class>
            {children()}
        </ol>
    }
}

/// Breadcrumb item
#[component]
pub fn BreadcrumbItem(
    /// Item content
    children: Children,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = "inline-flex items-center gap-1.5";
    let item_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <li class=item_class>
            {children()}
        </li>
    }
}

/// Breadcrumb link
#[component]
pub fn BreadcrumbLink(
    /// Link text
    #[prop(into)]
    text: String,

    /// Link href
    #[prop(optional, into)]
    href: Option<String>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = "transition-colors hover:text-foreground";
    let link_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <a href=href class=link_class>
            {text}
        </a>
    }
}

/// Breadcrumb separator
#[component]
pub fn BreadcrumbSeparator(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,

    /// Children (optional, defaults to /)
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let base_classes = "select-none";
    let sep_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <li role="presentation" aria-hidden="true" class=sep_class>
            {move || children.as_ref().map(|c| c()).unwrap_or_else(|| view! { "/" }.into_view())}
        </li>
    }
}

/// Current/active breadcrumb page
#[component]
pub fn BreadcrumbPage(
    /// Page text
    #[prop(into)]
    text: String,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = "font-normal text-foreground";
    let page_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <span role="link" aria-disabled="true" aria-current="page" class=page_class>
            {text}
        </span>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn mount_breadcrumb_simple() -> Result<(), JsValue> {
    mount_to_body(|| {
        view! {
            <Breadcrumb>
                <BreadcrumbList>
                    <BreadcrumbItem>
                        <BreadcrumbLink text="Home" href=Some("/".to_string()) />
                    </BreadcrumbItem>
                    <BreadcrumbSeparator children=None />
                    <BreadcrumbItem>
                        <BreadcrumbLink text="Components" href=Some("/components".to_string()) />
                    </BreadcrumbItem>
                    <BreadcrumbSeparator children=None />
                    <BreadcrumbItem>
                        <BreadcrumbPage text="Breadcrumb" />
                    </BreadcrumbItem>
                </BreadcrumbList>
            </Breadcrumb>
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_compiles() {
        assert!(true);
    }
}
