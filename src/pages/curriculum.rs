use leptos::*;
use leptos_router::*;
use crate::lab::modules::{modules_in_section, sections, ModuleMeta};
use crate::utils::module_href;

#[component]
pub fn CurriculumPage() -> impl IntoView {
    view! {
        <section id="curriculum">
            <h2>"04 / JULIA CRYPTOGRAPHY"</h2>
            <p class="section-intro">
                "Computational mathematics for cybersecurity. Each module pairs theory with an interactive browser lab — "
                "read the concept on the left, run and verify code on the right."
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
            <ul class="curriculum-problem-list">
                {modules.into_iter().enumerate().map(|(index, module)| view! {
                    <li>
                        <A href=module_href(module.slug) class="curriculum-problem-row">
                            <span class="curriculum-problem-index">{format!("{:02}", index + 1)}</span>
                            <span class="curriculum-problem-title">{module.title}</span>
                            <span class="curriculum-problem-tag">{module.section}</span>
                        </A>
                    </li>
                }).collect_view()}
            </ul>
        </div>
    }
}
