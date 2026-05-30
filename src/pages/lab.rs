use leptos::*;
use crate::pages::module::ModulePage;

/// Legacy lab route — labs are merged into the module page.
#[component]
pub fn LabPage() -> impl IntoView {
    view! { <ModulePage /> }
}
