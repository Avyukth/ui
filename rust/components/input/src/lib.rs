use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[component]
pub fn Input(
    // Input-specific attributes
    #[prop(into, optional)] r#type: MaybeProp<String>,
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] readonly: Signal<bool>,
    #[prop(into, optional)] required: Signal<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] autocomplete: MaybeProp<String>,
    #[prop(into, optional)] min: MaybeProp<String>,
    #[prop(into, optional)] max: MaybeProp<String>,
    #[prop(into, optional)] step: MaybeProp<String>,
    #[prop(into, optional)] pattern: MaybeProp<String>,
    #[prop(into, optional)] maxlength: MaybeProp<i32>,
    #[prop(into, optional)] minlength: MaybeProp<i32>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    // Event handlers
    #[prop(into, optional)] oninput: Option<Callback<ev::Event>>,
    #[prop(into, optional)] onchange: Option<Callback<ev::Event>>,
    #[prop(into, optional)] onfocus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] onblur: Option<Callback<ev::FocusEvent>>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type=move || r#type.get().unwrap_or_else(|| "text".to_string())
            value=value
            placeholder=move || placeholder.get()
            disabled=disabled
            readonly=readonly
            required=required
            name=move || name.get()
            autocomplete=move || autocomplete.get()
            min=move || min.get()
            max=move || max.get()
            step=move || step.get()
            pattern=move || pattern.get()
            maxlength=move || maxlength.get()
            minlength=move || minlength.get()
            class=move || tw_merge!(
                "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
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
            on:focus=move |ev| {
                if let Some(handler) = onfocus.as_ref() {
                    handler.run(ev);
                }
            }
            on:blur=move |ev| {
                if let Some(handler) = onblur.as_ref() {
                    handler.run(ev);
                }
            }
        />
    }
}

// Usage Example
#[component]
pub fn InputDemo() -> impl IntoView {
    let (email, set_email) = signal(String::new());

    view! {
        <Input
            r#type="email"
            placeholder="Email"
            value=email
            oninput=Callback::new(move |ev: ev::Event| {
                let value = event_target_value(&ev);
                set_email.set(value);
            })
        />
    }
}
