use button::*;
use leptos::*;
use wasm_bindgen::prelude::*;

/// Basic example showing simple Button usage
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background p-8 flex items-center justify-center">
            <div class="flex gap-4">
                <Button>
                    "Click Me"
                </Button>

                <Button variant=Signal::derive(|| ButtonVariant::Secondary)>
                    "Secondary"
                </Button>

                <Button variant=Signal::derive(|| ButtonVariant::Outline)>
                    "Outline"
                </Button>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
