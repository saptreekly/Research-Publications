use leptos::*;
use leptos_router::*;
use crate::components::technical_document::TechnicalDocument;
use crate::projects::find_by_slug;
use crate::utils::home_href;

#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();

    view! {
        <section id="project-nav">
            <A href=home_href() class="social-link cta-link">"← BACK TO HOME"</A>
        </section>

        {move || match find_by_slug(&slug()) {
            Some(project) => view! {
                <section class="report-page">
                    <header class="report-header">
                        <div class="report-header-meta">
                            <span class="home-tag">{project.tag}</span>
                            <time class="home-date" datetime=project.date>{project.date}</time>
                        </div>
                        <h2 class="report-title">{project.title}</h2>
                        <p class="report-subtitle">{project.subtitle}</p>
                        <a
                            href=project.repo_url
                            class="home-cta project-repo-link"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "VIEW REPOSITORY →"
                        </a>
                    </header>
                    <TechnicalDocument src=project.src />
                </section>
            }.into_view(),
            None => view! {
                <section class="report-page">
                    <h2>"PROJECT NOT FOUND"</h2>
                    <p class="doc-error">"Unknown engineering project."</p>
                    <A href=home_href() class="social-link cta-link">"BACK TO HOME"</A>
                </section>
            }.into_view(),
        }}
    }
}
