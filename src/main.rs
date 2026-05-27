mod components;

use components::animated_background::AnimatedBackground;
use components::technical_document::TechnicalDocument;
use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
struct StackItem { language: String, bytes: u64 }

#[derive(Clone, Deserialize, Serialize)]
struct StackData { updated_at: String, languages: Vec<StackItem> }

#[component]
fn StackMatrix() -> impl IntoView {
    let stack = create_resource(|| (), |_| async move {
        gloo_net::http::Request::get("static/stack.json")
            .send()
            .await
            .unwrap()
            .json::<StackData>()
            .await
            .unwrap_or_else(|_| StackData { updated_at: "N/A".to_string(), languages: vec![] })
    });

    view! {
        <div class="stack-matrix">
            <h2 style="margin-bottom: 15px; border: none; padding: 0;">"LANGUAGE DISTRIBUTION"</h2>
            <Suspense fallback=move || view! { <div class="stack-label">"Loading..."</div> }>
                {move || stack.get().map(|data| view! {
                    {data.languages.into_iter().map(|item| {
                        let max_bytes = 100000.0;
                        let percentage = ((item.bytes as f64 / max_bytes) * 100.0).min(100.0);
                        view! {
                            <div class="stack-row">
                                <div class="stack-label">{item.language}</div>
                                <div class="bar-container">
                                    <div class="bar" style=format!("width: {}%;", percentage)></div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                    <div class="row-date" style="margin-top: 15px; font-size: 0.55rem;">
                        "LAST AUDIT: " {data.updated_at}
                    </div>
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="JACK WEEKLY | CYBERSECURITY" />
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;700&family=IBM+Plex+Mono:wght@400&display=swap" rel="stylesheet" />

        <AnimatedBackground />

        <div id="app-container">
            <aside>
                <div>
                    <h1>"JACK WEEKLY"</h1>
                    <div class="row-tag">"CYBERSECURITY RESEARCHER"</div>
                    <StackMatrix />
                    <div class="social-links">
                        <a href="https://x.com/weeklyjack1" class="social-link">
                            <svg viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
                            "TWITTER"
                        </a>
                        <a href="https://linkedin.com/in/jack-weekly/" class="social-link">
                            <svg viewBox="0 0 24 24"><path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/></svg>
                            "LINKEDIN"
                        </a>
                        <a href="https://github.com/saptreekly" class="social-link">
                            <svg viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
                            "GITHUB"
                        </a>
                    </div>
                </div>
            </aside>

            <main>
                <section id="about">
                    <h2>"ABOUT"</h2>
                    <p>"Most threat intelligence professionals come from either a policy background or a technical one. I work across both. My Master of Strategic Studies specialized in cyber warfare as an instrument of state power, which sits alongside hands-on experience in reverse engineering advanced malware, vulnerability assessment, and security architecture."</p>
                </section>

                <section id="curriculum">
                    <h2>"TECHNICAL CURRICULUM"</h2>
                    <TechnicalDocument content=r#"
### Cryptographic Implementation
To implement the Extended Euclidean Algorithm for $ax + by = \gcd(a, b)$, we must define the iterative process.
```julia
function extended_gcd(a, b)
    if a == 0
        return (b, 0, 1)
    else
        g, y, x = extended_gcd(b % a, a)
        return (g, x - (b ÷ a) * y, y)
    end
end
```
"# />
                </section>

                <section id="projects">
                    <h2>"PROJECTS"</h2>
                    <div class="project-card">
                        <h3>"Project Hliðskjálf"</h3>
                        <p>"Bare-metal Type-1.5 Rust hypervisor."</p>
                    </div>
                </section>
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App /> });
}
