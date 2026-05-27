mod components;
mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::layout::RootLayout;
use pages::home::HomePage;
use pages::curriculum::CurriculumPage;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="JACK WEEKLY | CYBERSECURITY" />
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;700&family=IBM+Plex+Mono:wght@400&display=swap" rel="stylesheet" />

        // Router base must match the subfolder on GitHub Pages
        <Router>
            <Routes>
                // This route matches /Research-Publications/
                <Route path="/" view=RootLayout>
                    <Route path="" view=HomePage />
                    <Route path="curriculum" view=CurriculumPage />
                </Route>
                <Route path="/*any" view=|| view! { <div style="color: white; padding: 40px; font-family: monospace; z-index: 9999; position: fixed;">"ROUTING ERROR: Unmatched Path Locality"</div> }/>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App /> });
}
