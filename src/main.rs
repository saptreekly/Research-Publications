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

    // The Router component MUST be wrapped in a way that provides integration context 
    // for CSR (Client Side Rendering).
    view! {
        <Title text="JACK WEEKLY | CYBERSECURITY" />
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;700&family=IBM+Plex+Mono:wght@400&display=swap" rel="stylesheet" />

        <Router>
            <Routes>
                <Route path="/" view=RootLayout>
                    <Route path="" view=HomePage />
                    <Route path="curriculum" view=CurriculumPage />
                </Route>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    // In CSR, leptos_router::provide_browser_router_integration must be called
    // or mount_to_body must be used in a way that provides it.
    leptos::mount_to_body(|| {
        view! { <App /> }
    });
}
