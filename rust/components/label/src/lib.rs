use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[component]
pub fn Label(
    // Label-specific attributes
    #[prop(into, optional)] r#for: MaybeProp<String>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <label
            node_ref=node_ref
            r#for=move || r#for.get()
            class=move || tw_merge!(
                "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
                class.get()
            )
            id=move || id.get()
            style=style
        >
            {children()}
        </label>
    }
}

// Usage Example
#[component]
pub fn LabelDemo() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-2">
            <Label r#for="email">"Your email"</Label>
            <input
                id="email"
                type="email"
                class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2"
            />
        </div>
    }
}
