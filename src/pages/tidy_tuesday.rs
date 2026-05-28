use leptos::*;
use leptos_router::*;
use crate::components::technical_document::TechnicalDocument;
use crate::tidy_tuesday::se4all::Se4AllExplorer;
use crate::tidy_tuesday::{find_by_slug, ENTRIES};
use crate::utils::{home_href, tidy_tuesday_href};

#[component]
pub fn TidyTuesdayIndexPage() -> impl IntoView {
    view! {
        <section id="tidy-tuesday">
            <h2>"05 / TIDY TUESDAY"</h2>
            <p class="section-intro">
                "Weekly data explorations in Julia using datasets from the "
                <a
                    href="https://github.com/rfordatascience/tidytuesday"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    "TidyTuesday"
                </a>
                " community. Each entry includes reproducible Julia code and interactive browser visualizations."
            </p>

            <ul class="curriculum-module-list">
                {ENTRIES.iter().map(|entry| view! {
                    <li class="curriculum-module-item">
                        <div class="curriculum-module-card">
                            <div class="home-card-meta">
                                <span class="home-tag">{entry.tag}</span>
                                <time class="home-date" datetime=entry.date>{entry.date}</time>
                            </div>
                            <div class="cert-title">{entry.title}</div>
                            <p class="home-card-body">{entry.subtitle}</p>
                            <div class="curriculum-module-actions">
                                <A
                                    href=tidy_tuesday_href(entry.slug)
                                    class="social-link cta-link"
                                >
                                    "READ ANALYSIS"
                                </A>
                            </div>
                        </div>
                    </li>
                }).collect_view()}
            </ul>
        </section>
    }
}

#[component]
pub fn TidyTuesdayPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();

    view! {
        <section id="tidy-tuesday-nav">
            <A href=home_href() class="social-link cta-link">"← BACK TO HOME"</A>
        </section>

        {move || match find_by_slug(&slug()) {
            Some(entry) => view! {
                <section class="report-page">
                    <header class="report-header">
                        <div class="report-header-meta">
                            <span class="home-tag">{entry.tag}</span>
                            <time class="home-date" datetime=entry.dataset_date>{entry.dataset_date}</time>
                        </div>
                        <h2 class="report-title">{entry.title}</h2>
                        <p class="report-subtitle">{entry.subtitle}</p>
                        <a
                            href=format!("https://github.com/saptreekly/Research-Publications/blob/main/{}", entry.julia_src)
                            class="home-cta project-repo-link"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "VIEW JULIA SOURCE →"
                        </a>
                    </header>
                    {entry.explore_data.map(|data_url| view! {
                        <Se4AllExplorer data_url=data_url />
                    })}
                    <TechnicalDocument src=entry.src />
                </section>
            }.into_view(),
            None => view! {
                <section class="report-page">
                    <h2>"ANALYSIS NOT FOUND"</h2>
                    <p class="doc-error">"Unknown Tidy Tuesday entry."</p>
                    <A href=home_href() class="social-link cta-link">"BACK TO HOME"</A>
                </section>
            }.into_view(),
        }}
    }
}
