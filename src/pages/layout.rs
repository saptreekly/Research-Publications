use leptos::*;
use leptos_router::*;
use crate::components::theme_toggle::ThemeToggle;
use crate::utils::{
    contact_href, curriculum_href, home_href, malware_reports_index_href, situation_monitor_href,
    tidy_tuesday_index_href,
};

#[component]
pub fn RootLayout(children: Children) -> impl IntoView {
    let location = use_location();
    let path = move || location.pathname.get();
    let mobile_nav_open = create_rw_signal(false);

    create_effect(move |_| {
        let _ = path();
        mobile_nav_open.set(false);
        if let Some(window) = web_sys::window() {
            let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
        }
    });

    view! {
        <div id="app-container">
            <aside class="site-sidebar">
                <div class="site-sidebar-header">
                    <div class="site-brand">
                        <h1><A href=home_href()>"JACK WEEKLY"</A></h1>
                        <div class="row-tag">"STRATEGIC & TECHNICAL ANALYSIS"</div>
                    </div>
                    <button
                        type="button"
                        class="site-nav-toggle"
                        aria-label="Toggle navigation"
                        aria-expanded=move || mobile_nav_open.get()
                        on:click=move |_| mobile_nav_open.update(|open| *open = !*open)
                    >
                        {move || if mobile_nav_open.get() { "CLOSE" } else { "MENU" }}
                    </button>
                </div>

                <div
                    class="site-sidebar-panel"
                    class:site-sidebar-panel-open=move || mobile_nav_open.get()
                >
                    <div class="site-sidebar-panel-inner">
                        <div class="site-sidebar-top">
                        <nav class="site-nav" aria-label="Primary">
                            <div class="site-nav-label">"NAVIGATION"</div>
                            <ul class="site-nav-list">
                                <li><A href=home_href() class="nav-link">"HOME"</A></li>
                                <li><A href=curriculum_href() class="nav-link">"CURRICULUM"</A></li>
                                <li><A href=tidy_tuesday_index_href() class="nav-link">"TIDY TUESDAY"</A></li>
                                {#[cfg(feature = "malware-traffic")]
                                {
                                    view! {
                                        <li>
                                            <A href=malware_reports_index_href() class="nav-link">
                                                "MALWARE REPORTS"
                                            </A>
                                        </li>
                                    }.into_view()
                                }}
                                <li><A href=situation_monitor_href() class="nav-link">"SITUATION MONITOR"</A></li>
                                <li><A href=contact_href() class="nav-link">"CONTACT"</A></li>
                                <li>
                                    <a
                                        href="https://github.com/saptreekly/Computational-Mathematics-for-Cybersecurity-with-Julia"
                                        class="nav-link nav-link-external"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                    >
                                        "JULIA CRYPTO REPO"
                                    </a>
                                </li>
                            </ul>
                        </nav>

                        <A href=contact_href() class="site-availability">
                            <span class="site-availability-status">"Currently open"</span>
                            <span class="site-availability-title">"Open to opportunities"</span>
                            <span class="site-availability-desc">
                                "Dual US & NZ citizen (US-born), Wellington. Not currently cleared. "
                                "Open to NZ and international roles, including NZSIS or equivalent vetting."
                            </span>
                            <span class="site-availability-cta">"Get in touch →"</span>
                        </A>
                    </div>

                    <div class="site-sidebar-bottom">
                        <div class="site-theme-row">
                            <ThemeToggle />
                        </div>
                        <div class="site-nav-label">"CONNECT"</div>
                        <div class="social-links">
                            <a href="https://x.com/weeklyjack1" class="social-link">
                                <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
                                "TWITTER"
                            </a>
                            <a href="https://linkedin.com/in/jack-weekly/" class="social-link">
                                <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/></svg>
                                "LINKEDIN"
                            </a>
                            <a href="https://github.com/saptreekly" class="social-link">
                                <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
                                "GITHUB"
                            </a>
                        </div>
                        <div class="site-meta">
                            <div class="row-date">"WELLINGTON · DUAL US/NZ CITIZEN"</div>
                        </div>
                    </div>
                    </div>
                </div>
            </aside>

            <main class="site-main">
                <div class="page-transition" key=path>
                    {children()}
                </div>
            </main>
        </div>
    }
}
