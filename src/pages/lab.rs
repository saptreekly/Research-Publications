use leptos::*;
use leptos_router::*;
use crate::lab::components::LabWorkspace;
use crate::lab::modules::find_by_slug;
use crate::utils::{curriculum_href, module_href};

#[component]
pub fn LabPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();

    view! {
        <section id="lab-nav" class="curriculum-module-actions">
            <A href=curriculum_href() class="social-link cta-link">"← CURRICULUM"</A>
            {move || find_by_slug(&slug()).map(|module| view! {
                <A href=module_href(module.slug) class="social-link cta-link cta-link-secondary">
                    "← MODULE"
                </A>
            })}
        </section>

        {move || match find_by_slug(&slug()) {
            Some(module) => view! {
                <LabWorkspace
                    module_src=module.lab_src
                    module_id=module.id
                    module_title=module.title
                />
            }.into_view(),
            None => view! {
                <section class="lab-workspace">
                    <h2>"LAB // MODULE NOT FOUND"</h2>
                    <p class="doc-error">"Unknown module slug. Return to the curriculum index."</p>
                    <A href=curriculum_href() class="social-link cta-link">"BACK TO CURRICULUM"</A>
                </section>
            }.into_view(),
        }}
    }
}
