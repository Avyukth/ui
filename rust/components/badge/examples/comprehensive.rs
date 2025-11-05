use badge::*;
use leptos::*;
use wasm_bindgen::prelude::*;

/// Comprehensive example showcasing all Badge variants and features
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background p-8">
            <div class="max-w-6xl mx-auto space-y-12">
                // Header
                <header>
                    <h1 class="text-4xl font-bold mb-2">"Badge Component"</h1>
                    <p class="text-muted-foreground text-lg">
                        "Displays a badge or label for status, categories, and tags."
                    </p>
                </header>

                // Variants Section
                <section class="space-y-4">
                    <div>
                        <h2 class="text-2xl font-semibold mb-2">"Variants"</h2>
                        <p class="text-muted-foreground mb-4">
                            "Four badge variants for different use cases."
                        </p>
                    </div>

                    <div class="border rounded-lg p-6">
                        <div class="flex flex-wrap gap-3">
                            <Badge variant=Signal::derive(|| BadgeVariant::Default)>
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
                </section>

                // Use Cases Section
                <section class="space-y-4">
                    <div>
                        <h2 class="text-2xl font-semibold mb-2">"Common Use Cases"</h2>
                        <p class="text-muted-foreground mb-4">
                            "Badges in real-world scenarios."
                        </p>
                    </div>

                    <div class="border rounded-lg p-6 space-y-6">
                        // Status indicators
                        <div>
                            <h3 class="text-sm font-medium mb-2">"Status"</h3>
                            <div class="flex flex-wrap gap-2">
                                <Badge variant=Signal::derive(|| BadgeVariant::Default)>
                                    "Active"
                                </Badge>
                                <Badge variant=Signal::derive(|| BadgeVariant::Secondary)>
                                    "Pending"
                                </Badge>
                                <Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
                                    "Inactive"
                                </Badge>
                            </div>
                        </div>

                        // Tags
                        <div>
                            <h3 class="text-sm font-medium mb-2">"Tags"</h3>
                            <div class="flex flex-wrap gap-2">
                                <Badge variant=Signal::derive(|| BadgeVariant::Outline)>
                                    "React"
                                </Badge>
                                <Badge variant=Signal::derive(|| BadgeVariant::Outline)>
                                    "TypeScript"
                                </Badge>
                                <Badge variant=Signal::derive(|| BadgeVariant::Outline)>
                                    "Tailwind"
                                </Badge>
                                <Badge variant=Signal::derive(|| BadgeVariant::Outline)>
                                    "Rust"
                                </Badge>
                            </div>
                        </div>

                        // Notifications
                        <div>
                            <h3 class="text-sm font-medium mb-2">"Notifications"</h3>
                            <div class="flex flex-wrap gap-2">
                                <Badge variant=Signal::derive(|| BadgeVariant::Default)>
                                    "New"
                                </Badge>
                                <Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
                                    "Urgent"
                                </Badge>
                                <Badge variant=Signal::derive(|| BadgeVariant::Secondary)>
                                    "Beta"
                                </Badge>
                            </div>
                        </div>
                    </div>
                </section>

                // With Numbers Section
                <section class="space-y-4">
                    <div>
                        <h2 class="text-2xl font-semibold mb-2">"With Numbers"</h2>
                        <p class="text-muted-foreground mb-4">
                            "Badges can display counts and numbers."
                        </p>
                    </div>

                    <div class="border rounded-lg p-6">
                        <div class="flex flex-wrap gap-3">
                            <Badge variant=Signal::derive(|| BadgeVariant::Default)>
                                "99+"
                            </Badge>

                            <Badge variant=Signal::derive(|| BadgeVariant::Secondary)>
                                "42"
                            </Badge>

                            <Badge variant=Signal::derive(|| BadgeVariant::Destructive)>
                                "3"
                            </Badge>

                            <Badge variant=Signal::derive(|| BadgeVariant::Outline)>
                                "New 5"
                            </Badge>
                        </div>
                    </div>
                </section>

                // Interactive Section
                <section class="space-y-4">
                    <div>
                        <h2 class="text-2xl font-semibold mb-2">"Interactive Demo"</h2>
                        <p class="text-muted-foreground mb-4">
                            "Toggle between different badge variants."
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
    let (variant, set_variant) = create_signal(BadgeVariant::Default);

    let variant_name = move || match variant.get() {
        BadgeVariant::Default => "Default",
        BadgeVariant::Secondary => "Secondary",
        BadgeVariant::Destructive => "Destructive",
        BadgeVariant::Outline => "Outline",
    };

    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap gap-2">
                <button
                    class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-3"
                    on:click=move |_| set_variant.set(BadgeVariant::Default)
                >
                    "Default"
                </button>

                <button
                    class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-3"
                    on:click=move |_| set_variant.set(BadgeVariant::Secondary)
                >
                    "Secondary"
                </button>

                <button
                    class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-3"
                    on:click=move |_| set_variant.set(BadgeVariant::Destructive)
                >
                    "Destructive"
                </button>

                <button
                    class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-3"
                    on:click=move |_| set_variant.set(BadgeVariant::Outline)
                >
                    "Outline"
                </button>
            </div>

            <div class="flex items-center gap-4">
                <Badge variant=Signal::derive(move || variant.get())>
                    {variant_name}
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
