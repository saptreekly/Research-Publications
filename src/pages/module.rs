use leptos::*;
use leptos_router::*;
use crate::components::technical_document::TechnicalDocument;
use crate::lab::modules::find_by_slug;
use crate::utils::{curriculum_href, lab_href};

#[component]
pub fn ModulePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();

    view! {
        <section id="module-nav">
            <A href=curriculum_href() class="social-link cta-link">"← BACK TO CURRICULUM"</A>
        </section>

        {move || match find_by_slug(&slug()) {
            Some(module) => view! {
                <section id="module">
                    <div class="curriculum-module-header">
                        <div>
                            <div class="row-tag">{module.section.to_uppercase()}</div>
                            <h2>{module.title}</h2>
                        </div>
                        <A href=lab_href(module.slug) class="social-link cta-link">
                            "OPEN INTERACTIVE LAB"
                        </A>
                    </div>
                    <TechnicalDocument src=module.theory_src />
                </section>
            }.into_view(),
            None => view! {
                <section id="module">
                    <h2>"MODULE NOT FOUND"</h2>
                    <p class="doc-error">"Unknown curriculum module."</p>
                    <A href=curriculum_href() class="social-link cta-link">"BACK TO CURRICULUM"</A>
                </section>
            }.into_view(),
        }}
    }
}
