use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(into, optional)] checked: Signal<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] required: Signal<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    // Event handlers
    #[prop(into, optional)] onchange: Option<Callback<ev::Event>>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <button
            node_ref=node_ref
            type="button"
            role="checkbox"
            aria-checked=move || checked.get().to_string()
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            disabled=disabled
            class=move || tw_merge!(
                "peer h-4 w-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground",
                class.get()
            )
            id=move || id.get()
            style=style
            on:click=move |ev| {
                if !disabled.get() {
                    if let Some(handler) = onchange.as_ref() {
                        handler.run(ev);
                    }
                }
            }
        >
            {move || if checked.get() {
                view! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="h-4 w-4"
                    >
                        <path d="M20 6 9 17l-5-5" />
                    </svg>
                }.into_any()
            } else {
                ().into_any()
            }}
        </button>
    }
}

// Usage Example
#[component]
pub fn CheckboxDemo() -> impl IntoView {
    let (checked, set_checked) = signal(false);

    view! {
        <div class="flex items-center space-x-2">
            <Checkbox
                id="terms"
                checked=checked
                onchange=Callback::new(move |_| {
                    set_checked.update(|c| *c = !*c);
                })
            />
            <label
                for="terms"
                class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >
                "Accept terms and conditions"
            </label>
        </div>
    }
}
