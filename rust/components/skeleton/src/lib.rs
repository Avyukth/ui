use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[component]
pub fn Skeleton(
    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || tw_merge!(
                "animate-pulse rounded-md bg-muted",
                class.get()
            )
            id=move || id.get()
            style=style
        />
    }
}

// Usage Example
#[component]
pub fn SkeletonDemo() -> impl IntoView {
    view! {
        <div class="flex items-center space-x-4">
            <Skeleton class="h-12 w-12 rounded-full" />
            <div class="space-y-2">
                <Skeleton class="h-4 w-[250px]" />
                <Skeleton class="h-4 w-[200px]" />
            </div>
        </div>
    }
}
