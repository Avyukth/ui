use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[component]
pub fn Switch(
    #[prop(into, optional)] checked: Signal<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    // Event handlers
    #[prop(into, optional)] onchange: Option<Callback<ev::MouseEvent>>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <button
            node_ref=node_ref
            type="button"
            role="switch"
            aria-checked=move || checked.get().to_string()
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            disabled=disabled
            class=move || tw_merge!(
                "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
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
            <span
                data-state=move || if checked.get() { "checked" } else { "unchecked" }
                class="pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
            />
        </button>
    }
}

// Usage Example
#[component]
pub fn SwitchDemo() -> impl IntoView {
    let (enabled, set_enabled) = signal(false);

    view! {
        <div class="flex items-center space-x-2">
            <Switch
                id="airplane-mode"
                checked=enabled
                onchange=Callback::new(move |_| {
                    set_enabled.update(|e| *e = !*e);
                })
            />
            <label
                for="airplane-mode"
                class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >
                "Airplane Mode"
            </label>
        </div>
    }
}
