mod components;
mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::layout::RootLayout;
use pages::home::HomePage;
use pages::curriculum::CurriculumPage;
use web_sys::console;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    console::log_1(&"[WASM TELEMETRY] Initializing App Router Tree".into());

    view! {
        <Title text="JACK WEEKLY | CYBERSECURITY" />
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;700&family=IBM+Plex+Mono:wght@400&display=swap" rel="stylesheet" />

        <Router>
            <Routes>
                <Route path="/Research-Publications" view=RootLayout>
                    <Route path="" view=HomePage />
                    <Route path="curriculum" view=CurriculumPage />
                </Route>
                <Route path="/*any" view=|| {
                    let params = use_params_map();
                    let any_param = move || params.with(|p| p.get("any").cloned().unwrap_or_default());
                    let full_path = move || leptos::window().location().pathname().unwrap_or_default();

                    let log_msg = format!("[WASM TELEMETRY] Routing Warning: Unmatched path: {} at time: {:?}", any_param(), js_sys::Date::new_0().to_iso_string());
                    console::warn_1(&log_msg.into());

                    view! {
                        <div style="color: white; padding: 40px; font-family: monospace; z-index: 9999; position: fixed; background: rgba(255, 0, 0, 0.8);">
                            "ROUTING ERROR: Unmatched Path Locality"
                            <br />
                            "Unmatched Param: " {any_param}
                            <br />
                            "Absolute Location: " {full_path}
                        </div>
                    }
                }/>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    
    if let Ok(path) = leptos::window().location().pathname() {
        console::log_1(&format!("[WASM TELEMETRY] Absolute Browser Pathname: {}", path).into());
    }
    
    leptos::mount_to_body(|| view! { <App /> });
}
