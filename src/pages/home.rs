use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section id="about">
            <h2>"01 / ABOUT"</h2>
            <p>"Most threat intelligence professionals come from either a policy background or a technical one. I work across both. My Master of Strategic Studies specialized in cyber warfare as an instrument of state power, which sits alongside hands-on experience in reverse engineering advanced malware, vulnerability assessment, and security architecture."</p>
        </section>

        <section id="publications">
            <h2>"02 / INVESTIGATIVE RESEARCH"</h2>
            <div class="dashboard-section">
                <div class="row">
                    <div class="row-meta">
                        <span class="row-tag">"[TYPE: REVERSE_ENG]"</span>
                        <span class="row-date">"2026.05 //"</span>
                    </div>
                    <div class="row-content">
                        <h3>"LovelyMalware Analysis Report"</h3>
                        <p>"Full forensic analysis of a PE32+ ransomware binary. Kill chain reconstruction, custom AES-256-CBC analysis, and file decryption."</p>
                    </div>
                </div>
            </div>
        </section>

        <section id="projects">
            <h2>"03 / CORE ENGINE ARCHITECTURE"</h2>
            <div class="dashboard-section">
                <div class="row">
                    <div class="row-meta">
                        <span class="row-tag">"[LANG: RUST/ZIG]"</span>
                        <span class="row-date">"2026.04 //"</span>
                    </div>
                    <div class="row-content">
                        <h3>"Project Hliðskjálf"</h3>
                        <p>"Bare-metal Type-1.5 Rust hypervisor."</p>
                    </div>
                </div>
                <div class="row">
                    <div class="row-meta">
                        <span class="row-tag">"[LANG: RUST/ELIXIR]"</span>
                        <span class="row-date">"2026.04 //"</span>
                    </div>
                    <div class="row-content">
                        <h3>"SIEM Ensemble"</h3>
                        <p>"High-velocity log analytics engine."</p>
                    </div>
                </div>
            </div>
        </section>

        <section id="curriculum-link">
            <h2>"04 / ACADEMIC CURRICULUM"</h2>
            <p class="section-intro">"Explore my structured technical curriculum in Julia for cryptographic systems."</p>
            <A href="curriculum" class="social-link cta-link">"VIEW CURRICULUM"</A>
        </section>
    }
}
