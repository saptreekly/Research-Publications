use leptos::*;
use leptos_router::*;
use crate::lab::components::LabWorkspace;

#[component]
pub fn LabPage() -> impl IntoView {
    view! {
        <section id="lab-nav">
            <A href="curriculum" class="social-link cta-link">"← BACK TO CURRICULUM"</A>
        </section>

        <LabWorkspace
            module_src="research-docs/julia-crypto/mod_01_lab.md"
            module_id="mod_01"
            module_title="01.1 / Modular Foundations"
        />
    }
}
