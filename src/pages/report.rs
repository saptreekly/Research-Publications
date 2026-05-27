use leptos::*;
use leptos_router::*;
use crate::components::technical_document::TechnicalDocument;
use crate::reports::find_by_slug;
use crate::utils::home_href;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReportTab {
    Report,
    Iocs,
    Sigma,
}

#[component]
pub fn ReportPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();
    let active_tab = create_rw_signal(ReportTab::Report);

    view! {
        <section id="report-nav">
            <A href=home_href() class="social-link cta-link">"← BACK TO HOME"</A>
        </section>

        {move || match find_by_slug(&slug()) {
            Some(report) => {
                let has_tabs = report.sigma_src.is_some() || report.ioc_src.is_some();
                view! {
                    <section class="report-page">
                        <header class="report-header">
                            <div class="report-header-meta">
                                <span class="home-tag">{report.tag}</span>
                                <time class="home-date" datetime=report.date>{report.date}</time>
                            </div>
                            <h2 class="report-title">{report.title}</h2>
                            <p class="report-subtitle">"Malware analysis report · HackTheBox Insane"</p>
                        </header>

                        {has_tabs.then(|| view! {
                            <div class="report-tabs" role="tablist" aria-label="Report sections">
                                <button
                                    type="button"
                                    role="tab"
                                    class="report-tab"
                                    class:report-tab-active=move || active_tab.get() == ReportTab::Report
                                    aria-selected=move || active_tab.get() == ReportTab::Report
                                    on:click=move |_| active_tab.set(ReportTab::Report)
                                >
                                    "ANALYSIS REPORT"
                                </button>
                                {report.ioc_src.map(|_| view! {
                                    <button
                                        type="button"
                                        role="tab"
                                        class="report-tab"
                                        class:report-tab-active=move || active_tab.get() == ReportTab::Iocs
                                        aria-selected=move || active_tab.get() == ReportTab::Iocs
                                        on:click=move |_| active_tab.set(ReportTab::Iocs)
                                    >
                                        "IOCS"
                                    </button>
                                })}
                                {report.sigma_src.map(|_| view! {
                                    <button
                                        type="button"
                                        role="tab"
                                        class="report-tab"
                                        class:report-tab-active=move || active_tab.get() == ReportTab::Sigma
                                        aria-selected=move || active_tab.get() == ReportTab::Sigma
                                        on:click=move |_| active_tab.set(ReportTab::Sigma)
                                    >
                                        "SIGMA RULES"
                                    </button>
                                })}
                            </div>
                        })}

                        {move || {
                            if !has_tabs {
                                return view! {
                                    <TechnicalDocument src=report.src />
                                }.into_view();
                            }

                            match active_tab.get() {
                                ReportTab::Report => view! {
                                    <div role="tabpanel" class="report-tab-panel">
                                        <TechnicalDocument src=report.src />
                                    </div>
                                }.into_view(),
                                ReportTab::Iocs => view! {
                                    <div role="tabpanel" class="report-tab-panel">
                                        <TechnicalDocument src=report.ioc_src.unwrap_or(report.src) />
                                    </div>
                                }.into_view(),
                                ReportTab::Sigma => view! {
                                    <div role="tabpanel" class="report-tab-panel">
                                        <TechnicalDocument src=report.sigma_src.unwrap_or(report.src) />
                                    </div>
                                }.into_view(),
                            }
                        }}
                    </section>
                }.into_view()
            }
            None => view! {
                <section class="report-page">
                    <h2>"REPORT NOT FOUND"</h2>
                    <p class="doc-error">"Unknown research report."</p>
                    <A href=home_href() class="social-link cta-link">"BACK TO HOME"</A>
                </section>
            }.into_view(),
        }}
    }
}
