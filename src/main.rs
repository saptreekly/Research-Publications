mod components;
mod lab;
mod malware_traffic;
mod pages;
mod projects;
mod reports;
mod seo;
mod situation_monitor;
mod theme;
mod tidy_tuesday;
mod utils;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::animated_background::AnimatedBackground;
use crate::components::seo_head::SeoHead;
use crate::theme::provide_theme;
use pages::layout::RootLayout;
use pages::home::HomePage;
use pages::contact::ContactPage;
use pages::curriculum::CurriculumPage;
use pages::lab::LabPage;
use pages::module::ModulePage;
use pages::report::ReportPage;
use pages::project::ProjectPage;
use pages::malware_traffic::{MalwareTrafficIndexPage, MalwareTrafficPage};
use pages::situation_monitor::SituationMonitorPage;
use pages::tidy_tuesday::{TidyTuesdayIndexPage, TidyTuesdayPage};

pub const APP_BASE: &str = "/Research-Publications";
const ROUTE_HOME: &str = "/Research-Publications/";
const ROUTE_CURRICULUM: &str = "/Research-Publications/curriculum";
const ROUTE_MODULE: &str = "/Research-Publications/curriculum/:slug";
const ROUTE_LAB: &str = "/Research-Publications/curriculum/lab/:slug";
const ROUTE_CONTACT: &str = "/Research-Publications/contact";
const ROUTE_REPORT: &str = "/Research-Publications/research/:slug";
const ROUTE_PROJECT: &str = "/Research-Publications/projects/:slug";
const ROUTE_TIDY_TUESDAY: &str = "/Research-Publications/tidy-tuesday";
const ROUTE_TIDY_TUESDAY_ENTRY: &str = "/Research-Publications/tidy-tuesday/:slug";
const ROUTE_SITUATION_MONITOR: &str = "/Research-Publications/situation-monitor";
const ROUTE_MALWARE_TRAFFIC: &str = "/Research-Publications/malware-traffic";
const ROUTE_MALWARE_TRAFFIC_ENTRY: &str = "/Research-Publications/malware-traffic/:slug";

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    provide_theme();

    view! {
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;700&family=IBM+Plex+Mono:wght@400&display=swap" rel="stylesheet" />

        <Router base=APP_BASE trailing_slash=TrailingSlash::Redirect>
            <AnimatedBackground />
            <SeoHead />
            <Routes>
                <Route path=ROUTE_HOME view=move || view! {
                    <RootLayout><HomePage /></RootLayout>
                } />
                <Route path=ROUTE_LAB view=move || view! {
                    <RootLayout><LabPage /></RootLayout>
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
                <Route path=ROUTE_TIDY_TUESDAY_ENTRY view=move || view! {
                    <RootLayout><TidyTuesdayPage /></RootLayout>
                } />
                <Route path=ROUTE_TIDY_TUESDAY view=move || view! {
                    <RootLayout><TidyTuesdayIndexPage /></RootLayout>
                } />
                <Route path=ROUTE_SITUATION_MONITOR view=move || view! {
                    <RootLayout><SituationMonitorPage /></RootLayout>
                } />
                <Route path=ROUTE_MALWARE_TRAFFIC_ENTRY view=move || view! {
                    <RootLayout><MalwareTrafficPage /></RootLayout>
                } />
                <Route path=ROUTE_MALWARE_TRAFFIC view=move || view! {
                    <RootLayout><MalwareTrafficIndexPage /></RootLayout>
                } />
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    #[cfg(debug_assertions)]
    if let Ok(path) = leptos::window().location().pathname() {
        web_sys::console::log_1(
            &format!("[WASM TELEMETRY] Absolute Browser Pathname: {}", path).into(),
        );
    }

    leptos::mount_to_body(|| view! { <App /> });
}
