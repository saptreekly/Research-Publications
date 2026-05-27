mod components;
mod lab;
mod pages;
mod reports;
mod utils;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::animated_background::AnimatedBackground;
use pages::layout::RootLayout;
use pages::home::HomePage;
use pages::contact::ContactPage;
use pages::curriculum::CurriculumPage;
use pages::lab::LabPage;
use pages::module::ModulePage;
use pages::report::ReportPage;
use web_sys::console;

pub const APP_BASE: &str = "/Research-Publications";
const ROUTE_HOME: &str = "/Research-Publications/";
const ROUTE_CURRICULUM: &str = "/Research-Publications/curriculum";
const ROUTE_MODULE: &str = "/Research-Publications/curriculum/:slug";
const ROUTE_LAB: &str = "/Research-Publications/curriculum/lab/:slug";
const ROUTE_CONTACT: &str = "/Research-Publications/contact";
const ROUTE_REPORT: &str = "/Research-Publications/research/:slug";

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    console::log_1(&"[WASM TELEMETRY] Initializing App Router Tree".into());

    view! {
        <Title text="JACK WEEKLY | CYBERSECURITY" />
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;700&family=IBM+Plex+Mono:wght@400&display=swap" rel="stylesheet" />

        <AnimatedBackground />

        <Router base=APP_BASE trailing_slash=TrailingSlash::Redirect>
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
                <Route path=ROUTE_MODULE view=move || view! {
                    <RootLayout><ModulePage /></RootLayout>
                } />
                <Route path=ROUTE_CURRICULUM view=move || view! {
                    <RootLayout><CurriculumPage /></RootLayout>
                } />
                <Route path=ROUTE_CONTACT view=move || view! {
                    <RootLayout><ContactPage /></RootLayout>
                } />
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
