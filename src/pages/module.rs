use leptos::*;
use leptos_router::*;
use crate::lab::components::ModuleExercisePanels;
use crate::lab::modules::{find_by_slug, next_module, prev_module};
use crate::utils::{curriculum_href, module_href};

#[component]
pub fn ModulePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();

    view! {
        {move || match find_by_slug(&slug()) {
            Some(module) => {
                let prev = prev_module(module.slug);
                let next = next_module(module.slug);
                view! {
                    <section class="curriculum-module-page">
                        <header class="curriculum-module-topbar">
                            <A href=curriculum_href() class="social-link cta-link curriculum-back-link">
                                "← CURRICULUM"
                            </A>
                            <div class="curriculum-module-title">
                                <div class="row-tag">{module.section.to_uppercase()}</div>
                                <h2>{module.title}</h2>
                            </div>
                            <nav class="curriculum-module-pager" aria-label="Module navigation">
                                {prev.map(|prev_module| view! {
                                    <A href=module_href(prev_module.slug) class="social-link cta-link cta-link-secondary">
                                        "← PREV"
                                    </A>
                                })}
                                {next.map(|next_module| view! {
                                    <A href=module_href(next_module.slug) class="social-link cta-link cta-link-secondary">
                                        "NEXT →"
                                    </A>
                                })}
                            </nav>
                        </header>

                        <ModuleExercisePanels
                            module_id=module.id
                            _module_title=module.title
                            theory_src=module.theory_src
                        />
                    </section>
                }.into_view()
            }
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
