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
    console::log_1(&"App component rendering...".into());
    provide_meta_context();

    view! {
        <Title text="JACK WEEKLY | CYBERSECURITY" />
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;700&family=IBM+Plex+Mono:wght@400&display=swap" rel="stylesheet" />

        <Router>
            <Routes>
                <Route path="/Research-Publications" view=RootLayout>
                    <Route path="" view=HomePage />
                    <Route path="curriculum" view=CurriculumPage />
                </Route>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"Main function executing...".into());
    leptos::mount_to_body(|| {
        console::log_1(&"Mounting App...".into());
        view! { <App /> }
    });
}
