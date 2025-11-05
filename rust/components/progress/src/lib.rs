use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[component]
pub fn Progress(
    #[prop(into, optional)] value: Signal<f64>,
    #[prop(into, optional)] max: Signal<f64>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let percentage = move || {
        let val = value.get();
        let max_val = max.get();
        if max_val > 0.0 {
            (val / max_val * 100.0).min(100.0).max(0.0)
        } else {
            0.0
        }
    };

    view! {
        <div
            node_ref=node_ref
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax=move || max.get().to_string()
            aria-valuenow=move || value.get().to_string()
            class=move || tw_merge!(
                "relative h-4 w-full overflow-hidden rounded-full bg-secondary",
                class.get()
            )
            id=move || id.get()
            style=style
        >
            <div
                class="h-full w-full flex-1 bg-primary transition-all"
                style:transform=move || format!("translateX(-{}%)", 100.0 - percentage())
            />
        </div>
    }
}

// Usage Example
#[component]
pub fn ProgressDemo() -> impl IntoView {
    let (progress, set_progress) = signal(13.0);

    // Simulate progress
    set_interval(
        move || {
            set_progress.update(|p| {
                *p = (*p + 10.0).min(100.0);
            });
        },
        std::time::Duration::from_millis(500),
    );

    view! {
        <Progress value=progress max=Signal::derive(|| 100.0) />
    }
}
