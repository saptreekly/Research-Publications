use leptos::*;
use leptos_router::*;
use crate::lab::modules::{modules_in_section, sections, ModuleMeta};
use crate::utils::{lab_href, module_href};

#[component]
pub fn CurriculumPage() -> impl IntoView {
    view! {
        <section id="curriculum">
            <h2>"04 / JULIA CRYPTOGRAPHY"</h2>
            <p class="section-intro">
                "Computational mathematics for cybersecurity: theory and interactive labs. Select a module to read the curriculum or open its browser-based lab."
            </p>

            {sections()
                .iter()
                .map(|section| view! {
                    <CurriculumSection section=section />
                })
                .collect_view()}
        </section>
    }
}

#[component]
fn CurriculumSection(section: &'static str) -> impl IntoView {
    let modules: Vec<&'static ModuleMeta> = modules_in_section(section).collect();

    view! {
        <div class="curriculum-section">
            <h3>{format!("{} // MODULES", section.to_uppercase())}</h3>
            <ul class="curriculum-module-list">
                {modules.into_iter().map(|module| view! {
                    <li class="curriculum-module-item">
                        <div class="curriculum-module-card">
                            <div class="cert-title">{module.title}</div>
                            <div class="curriculum-module-actions">
                                <A
                                    href=module_href(module.slug)
                                    class="social-link cta-link"
                                >
                                    "READ MODULE"
                                </A>
                                <A
                                    href=lab_href(module.slug)
                                    class="social-link cta-link cta-link-secondary"
                                >
                                    "OPEN LAB"
                                </A>
                            </div>
                        </div>
                    </li>
                }).collect_view()}
            </ul>
        </div>
    }
}
