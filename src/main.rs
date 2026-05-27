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
    leptos::mount_to_body(|| view! { <App /> });
}
