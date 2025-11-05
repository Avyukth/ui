use leptos::*;
use wasm_bindgen::prelude::*;

/// A progress bar component
///
/// # Examples
///
/// ```rust
/// use progress::Progress;
/// use leptos::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Progress value=60.0 />
///         <Progress value=30.0 max=100.0 />
///     }
/// }
/// ```
#[component]
pub fn Progress(
    /// Current progress value
    #[prop(optional)]
    value: Option<f64>,

    /// Maximum value (default: 100)
    #[prop(optional)]
    max: Option<f64>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let value = value.unwrap_or(0.0);
    let max = max.unwrap_or(100.0);

    let percentage = if max > 0.0 {
        ((value / max) * 100.0).min(100.0).max(0.0)
    } else {
        0.0
    };

    let base_classes = "relative h-4 w-full overflow-hidden rounded-full bg-secondary";
    let progress_class = format!(
        "{}{}",
        base_classes,
        class.map(|c| format!(" {}", c)).unwrap_or_default()
    );

    view! {
        <div class=progress_class>
            <div
                class="h-full w-full flex-1 bg-primary transition-all"
                style:transform=format!("translateX(-{}%)", 100.0 - percentage)
            />
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn mount_progress(value: f64) -> Result<(), JsValue> {
    mount_to_body(move || {
        view! { <Progress value=Some(value) /> }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_calculation() {
        // Test will validate percentage calculation logic
        let value = 50.0;
        let max = 100.0;
        let percentage = ((value / max) * 100.0).min(100.0).max(0.0);
        assert_eq!(percentage, 50.0);
    }

    #[test]
    fn test_percentage_max_clamp() {
        let value = 150.0;
        let max = 100.0;
        let percentage = ((value / max) * 100.0).min(100.0).max(0.0);
        assert_eq!(percentage, 100.0);
    }
}
