mod routes;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use research_publications::components::animated_background::AnimatedBackground;
use research_publications::components::seo_head::SeoHead;
use research_publications::pages::contact::ContactPage;
use research_publications::pages::curriculum::CurriculumPage;
use research_publications::pages::home::HomePage;
use research_publications::pages::layout::RootLayout;
use research_publications::pages::module::ModulePage;
use research_publications::pages::project::ProjectPage;
use research_publications::pages::report::ReportPage;
use research_publications::theme::provide_theme;

use routes::{
    LabRoute, MalwareTrafficEntryRoute, MalwareTrafficIndexRoute, SituationMonitorRoute,
    TidyTuesdayEntryRoute, TidyTuesdayIndexRoute,
};

const ROUTE_HOME: &str = "/Research-Publications/";
const ROUTE_CURRICULUM: &str = "/Research-Publications/curriculum";
const ROUTE_MODULE: &str = "/Research-Publications/curriculum/:slug";
const ROUTE_CONTACT: &str = "/Research-Publications/contact";
const ROUTE_REPORT: &str = "/Research-Publications/research/:slug";
const ROUTE_PROJECT: &str = "/Research-Publications/projects/:slug";

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    provide_theme();

    view! {
        <Link href="https://fonts.googleapis.com/css2?family=Source+Sans+3:ital,wght@0,400;0,600;0,700;1,400&family=Source+Serif+4:ital,wght@0,400;0,600;0,700;1,400&family=IBM+Plex+Mono:wght@400;500&display=swap" rel="stylesheet" />

        // Route paths already include APP_BASE. Do not also set Router `base=APP_BASE`:
        // TrailingSlash::Redirect resolves through that base and doubles the prefix
        // (e.g. /Research-Publications/Research-Publications/...), which leaves only
        // the animated background after a deep-link refresh on GitHub Pages.
        <Router trailing_slash=TrailingSlash::Redirect>
            <AnimatedBackground />
            <SeoHead />
            <Routes>
                <Route path=ROUTE_HOME view=move || view! {
                    <RootLayout><HomePage /></RootLayout>
                } />
                <Route path=ROUTE_REPORT view=move || view! {
                    <RootLayout><ReportPage /></RootLayout>
                } />
                <Route path=ROUTE_PROJECT view=move || view! {
                    <RootLayout><ProjectPage /></RootLayout>
                } />
                <Route path=ROUTE_MODULE view=move || view! {
                    <RootLayout><ModulePage /></RootLayout>
                } />
                <Route path=ROUTE_CURRICULUM view=move || view! {
                    <RootLayout><CurriculumPage /></RootLayout>
                } />
                <Route path=ROUTE_CONTACT view=move || view! {
                    <RootLayout><ContactPage /></RootLayout>
                } />
                <LabRoute />
                <TidyTuesdayEntryRoute />
                <TidyTuesdayIndexRoute />
                <SituationMonitorRoute />
                <MalwareTrafficEntryRoute />
                <MalwareTrafficIndexRoute />
            </Routes>
        </Router>
    }
}

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    #[cfg(debug_assertions)]
    if let Ok(path) = leptos::window().location().pathname() {
        web_sys::console::log_1(
            &format!("[WASM TELEMETRY] Absolute Browser Pathname: {}", path).into(),
        );
    }

    leptos::mount_to_body(|| view! { <App /> });
}
