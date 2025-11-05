use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[component]
pub fn Textarea(
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] readonly: Signal<bool>,
    #[prop(into, optional)] required: Signal<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] rows: MaybeProp<i32>,
    #[prop(into, optional)] cols: MaybeProp<i32>,
    #[prop(into, optional)] maxlength: MaybeProp<i32>,
    #[prop(into, optional)] minlength: MaybeProp<i32>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    // Event handlers
    #[prop(into, optional)] oninput: Option<Callback<ev::Event>>,
    #[prop(into, optional)] onchange: Option<Callback<ev::Event>>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref
            placeholder=move || placeholder.get()
            disabled=disabled
            readonly=readonly
            required=required
            name=move || name.get()
            rows=move || rows.get()
            cols=move || cols.get()
            maxlength=move || maxlength.get()
            minlength=move || minlength.get()
            class=move || tw_merge!(
                "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
                class.get()
            )
            id=move || id.get()
            style=style
            on:input=move |ev| {
                if let Some(handler) = oninput.as_ref() {
                    handler.run(ev);
                }
            }
            on:change=move |ev| {
                if let Some(handler) = onchange.as_ref() {
                    handler.run(ev);
                }
            }
        >
            {move || value.get()}
        </textarea>
    }
}

// Usage Example
#[component]
pub fn TextareaDemo() -> impl IntoView {
    let (message, set_message) = signal(String::new());

    view! {
        <Textarea
            placeholder="Type your message here."
            value=message
            oninput=Callback::new(move |ev: ev::Event| {
                let value = event_target_value(&ev);
                set_message.set(value);
            })
        />
    }
}
