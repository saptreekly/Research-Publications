mod components;

use components::animated_background::AnimatedBackground;
use leptos::*;
use leptos_meta::*;

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
                </div>
                <div>
                    <div class="row-date">"WELLINGTON, NZ"</div>
                    <div class="row-date">"LINKEDIN · GITHUB"</div>
                </div>
            </aside>

            <main>
                <section id="about">
                    <h2>"01 / ABOUT"</h2>
                    <p>"Most threat intelligence professionals come from either a policy background or a technical one. I work across both. My Master of Strategic Studies specialized in cyber warfare as an instrument of state power, which sits alongside hands-on experience in reverse engineering advanced malware, vulnerability assessment, and security architecture."</p>
                </section>

                <section id="publications">
                    <h2>"02 / INVESTIGATIVE RESEARCH"</h2>
                    <div class="row">
                        <div class="row-tag">"[TYPE: REVERSE_ENG]"</div>
                        <div class="row-date">"2026.05 //"</div>
                        <div>
                            <h3>"LovelyMalware Analysis Report"</h3>
                            <p>"Full forensic analysis of a PE32+ ransomware binary. Kill chain reconstruction, custom AES-256-CBC analysis, and file decryption."</p>
                        </div>
                    </div>
                </section>

                <section id="projects">
                    <h2>"03 / CORE ENGINE ARCHITECTURE"</h2>
                    <div class="row">
                        <div class="row-tag">"[LANG: RUST/ZIG]"</div>
                        <div class="row-date">"2026.04 //"</div>
                        <div>
                            <h3>"Project Hliðskjálf"</h3>
                            <p>"Bare-metal Type-1.5 Rust hypervisor."</p>
                        </div>
                    </div>
                    <div class="row">
                        <div class="row-tag">"[LANG: RUST/ELIXIR]"</div>
                        <div class="row-date">"2026.04 //"</div>
                        <div>
                            <h3>"SIEM Ensemble"</h3>
                            <p>"High-velocity log analytics engine."</p>
                        </div>
                    </div>
                </section>

                <section id="certs">
                    <h2>"04 / VALIDATED METRICS"</h2>
                    <ul class="certs">
                        <li class="cert-item">
                            <div class="cert-label">"STATUS: VERIFIED"</div>
                            <div class="cert-title">"SOC Level 2 · TryHackMe"</div>
                        </li>
                        <li class="cert-item">
                            <div class="cert-label">"STATUS: VERIFIED"</div>
                            <div class="cert-title">"Google Cybersecurity"</div>
                        </li>
                    </ul>
                </section>
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App /> });
}
