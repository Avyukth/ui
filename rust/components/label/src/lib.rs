use leptos::*;
use wasm_bindgen::prelude::*;

/// A form label component
///
/// # Examples
///
/// ```rust
/// use label::Label;
/// use leptos::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Label for_id="email" text="Email Address" />
///         <input id="email" type="email" />
///     }
/// }
/// ```
#[component]
pub fn Label(
    /// The text to display in the label
    #[prop(into)]
    text: String,

    /// The ID of the form element this label is for
    #[prop(optional, into)]
    for_id: Option<String>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

    let label_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <label
            for=for_id
            class=label_class
        >
            {text}
        </label>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn mount_label(text: &str, for_id: &str) -> Result<(), JsValue> {
    let text = text.to_string();
    let for_id = if for_id.is_empty() { None } else { Some(for_id.to_string()) };

    mount_to_body(move || {
        view! { <Label text=text.clone() for_id=for_id.clone() /> }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_compiles() {
        // Basic compilation test
        assert!(true);
    }
}
