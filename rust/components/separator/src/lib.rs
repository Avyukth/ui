use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[derive(PartialEq, Clone, Copy)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

impl Default for SeparatorOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

#[component]
pub fn Separator(
    #[prop(into, optional)] orientation: Signal<SeparatorOrientation>,
    #[prop(into, optional)] decorative: Signal<bool>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let base_class = move || match orientation.get() {
        SeparatorOrientation::Horizontal => "h-[1px] w-full",
        SeparatorOrientation::Vertical => "h-full w-[1px]",
    };

    let role = move || {
        if decorative.get() {
            Some("none")
        } else {
            Some("separator")
        }
    };

    let aria_orientation = move || {
        if !decorative.get() {
            match orientation.get() {
                SeparatorOrientation::Horizontal => Some("horizontal"),
                SeparatorOrientation::Vertical => Some("vertical"),
            }
        } else {
            None
        }
    };

    view! {
        <div
            node_ref=node_ref
            role=role
            aria-orientation=aria_orientation
            class=move || tw_merge!(
                "shrink-0 bg-border",
                base_class(),
                class.get()
            )
            id=move || id.get()
            style=style
        />
    }
}

// Usage Example
#[component]
pub fn SeparatorDemo() -> impl IntoView {
    view! {
        <div>
            <div class="space-y-1">
                <h4 class="text-sm font-medium leading-none">"Radix Primitives"</h4>
                <p class="text-sm text-muted-foreground">
                    "An open-source UI component library."
                </p>
            </div>
            <Separator class="my-4" />
            <div class="flex h-5 items-center space-x-4 text-sm">
                <div>"Blog"</div>
                <Separator orientation=Signal::derive(|| SeparatorOrientation::Vertical) />
                <div>"Docs"</div>
                <Separator orientation=Signal::derive(|| SeparatorOrientation::Vertical) />
                <div>"Source"</div>
            </div>
        </div>
    }
}
