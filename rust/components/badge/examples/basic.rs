use badge::*;
use leptos::*;
use wasm_bindgen::prelude::*;

/// Basic example showing simple Badge usage
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background p-8 flex items-center justify-center">
            <div class="flex flex-wrap gap-3">
                <Badge>
                    "Default"
                </Badge>

                <Badge variant=Signal::derive(|| BadgeVariant::Secondary)>
                    "Secondary"
                </Badge>

                <Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
                    "Destructive"
                </Badge>

                <Badge variant=Signal::derive(|| BadgeVariant::Outline)>
                    "Outline"
                </Badge>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
