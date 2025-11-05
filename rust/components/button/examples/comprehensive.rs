use button::*;
use leptos::*;
use wasm_bindgen::prelude::*;

/// Comprehensive example showcasing all Button variants and features
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background p-8">
            <div class="max-w-6xl mx-auto space-y-12">
                // Header
                <header>
                    <h1 class="text-4xl font-bold mb-2">"Button Component"</h1>
                    <p class="text-muted-foreground text-lg">
                        "Displays a button or a component that looks like a button."
                    </p>
                </header>

                // Variants Section
                <section class="space-y-4">
                    <div>
                        <h2 class="text-2xl font-semibold mb-2">"Variants"</h2>
                        <p class="text-muted-foreground mb-4">
                            "Six button variants for different use cases."
                        </p>
                    </div>

                    <div class="border rounded-lg p-6">
                        <div class="flex flex-wrap gap-4">
                            <Button variant=Signal::derive(|| ButtonVariant::Default)>
                                "Default"
                            </Button>

                            <Button variant=Signal::derive(|| ButtonVariant::Secondary)>
                                "Secondary"
                            </Button>

                            <Button variant=Signal::derive(|| ButtonVariant::Destructive)>
                                "Destructive"
                            </Button>

                            <Button variant=Signal::derive(|| ButtonVariant::Outline)>
                                "Outline"
                            </Button>

                            <Button variant=Signal::derive(|| ButtonVariant::Ghost)>
                                "Ghost"
                            </Button>

                            <Button variant=Signal::derive(|| ButtonVariant::Link)>
                                "Link"
                            </Button>
                        </div>
                    </div>
                </section>

                // Sizes Section
                <section class="space-y-4">
                    <div>
                        <h2 class="text-2xl font-semibold mb-2">"Sizes"</h2>
                        <p class="text-muted-foreground mb-4">
                            "Four size options to fit your design needs."
                        </p>
                    </div>

                    <div class="border rounded-lg p-6">
                        <div class="flex flex-wrap items-center gap-4">
                            <Button size=Signal::derive(|| ButtonSize::Sm)>
                                "Small"
                            </Button>

                            <Button size=Signal::derive(|| ButtonSize::Default)>
                                "Default"
                            </Button>

                            <Button size=Signal::derive(|| ButtonSize::Lg)>
                                "Large"
                            </Button>

                            <Button size=Signal::derive(|| ButtonSize::Icon)>
                                "🔍"
                            </Button>
                        </div>
                    </div>
                </section>

                // Icon Buttons Section
                <section class="space-y-4">
                    <div>
                        <h2 class="text-2xl font-semibold mb-2">"With Icons"</h2>
                        <p class="text-muted-foreground mb-4">
                            "Buttons can contain SVG icons that are automatically sized."
                        </p>
                    </div>

                    <div class="border rounded-lg p-6">
                        <div class="flex flex-wrap gap-4">
                            <Button>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="10"/>
                                    <path d="M12 16v-4"/>
                                    <path d="M12 8h.01"/>
                                </svg>
                                "With Icon"
                            </Button>

                            <Button variant=Signal::derive(|| ButtonVariant::Outline) size=Signal::derive(|| ButtonSize::Icon)>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M5 12h14"/>
                                    <path d="m12 5 7 7-7 7"/>
                                </svg>
                            </Button>
                        </div>
                    </div>
                </section>

                // Disabled State Section
                <section class="space-y-4">
                    <div>
                        <h2 class="text-2xl font-semibold mb-2">"Disabled"</h2>
                        <p class="text-muted-foreground mb-4">
                            "Buttons can be disabled with reduced opacity."
                        </p>
                    </div>

                    <div class="border rounded-lg p-6">
                        <div class="flex flex-wrap gap-4">
                            <Button disabled=Signal::derive(|| true)>
                                "Disabled"
                            </Button>

                            <Button
                                variant=Signal::derive(|| ButtonVariant::Outline)
                                disabled=Signal::derive(|| true)
                            >
                                "Disabled Outline"
                            </Button>
                        </div>
                    </div>
                </section>

                // Interactive Section
                <section class="space-y-4">
                    <div>
                        <h2 class="text-2xl font-semibold mb-2">"Interactive"</h2>
                        <p class="text-muted-foreground mb-4">
                            "Buttons with click handlers."
                        </p>
                    </div>

                    <div class="border rounded-lg p-6">
                        <InteractiveDemo />
                    </div>
                </section>

                // Footer
                <footer class="pt-8 border-t text-center text-muted-foreground text-sm">
                    <p>"Built with Leptos and Rust WASM • Styled with Tailwind CSS"</p>
                </footer>
            </div>
        </div>
    }
}

#[component]
fn InteractiveDemo() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let increment = move |_| {
        set_count.update(|n| *n += 1);
    };

    let reset = move |_| {
        set_count.set(0);
    };

    view! {
        <div class="flex flex-wrap items-center gap-4">
            <Button onclick=Callback::new(increment)>
                "Click Me"
            </Button>

            <Button
                variant=Signal::derive(|| ButtonVariant::Secondary)
                onclick=Callback::new(reset)
            >
                "Reset"
            </Button>

            <div class="text-sm text-muted-foreground">
                "Clicked " {count} " times"
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
