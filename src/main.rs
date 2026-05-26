mod components;

use components::animated_background::AnimatedBackground;
use leptos::*;
use leptos_meta::*;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Jack Weekly | Cybersecurity Researcher" />
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;600&family=IBM+Plex+Mono:wght@400&display=swap" rel="stylesheet" />

        <AnimatedBackground />

        <header>
            <h1>"Jack Weekly"</h1>
            <div class="title-line">"Cybersecurity Researcher · Threat Intelligence · DFIR & Malware Analysis · Geopolitics & National Security"</div>
            <div class="location">"Wellington, New Zealand"</div>
            <div class="social-links">
                <a href="https://linkedin.com/in/jackweekly" title="LinkedIn">
                    <svg viewBox="0 0 24 24"><path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/></svg>
                </a>
                <a href="https://github.com/saptreekly" title="GitHub">
                    <svg viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
                </a>
                <a href="#" title="TryHackMe">
                    <svg viewBox="0 0 24 24"><path d="M12 24c6.627 0 12-5.373 12-12s-5.373-12-12-12-12 5.373-12 12 5.373 12 12 12zm-2.071-15.051l3.52 3.52-3.52 3.52c-.675.675-1.77.675-2.445 0s-.675-1.77 0-2.445l1.096-1.096-1.096-1.096c-.675-.675-.675-1.77 0-2.445s1.77-.675 2.445 0z"/></svg>
                </a>
                <a href="#" title="HackTheBox">
                    <svg viewBox="0 0 24 24"><path d="M12 0l-12 6.928v13.856l12 6.928 12-6.928v-13.856l-12-6.928zm0 2.309l9.6 5.543v11.085l-9.6 5.543-9.6-5.543v-11.085l9.6-5.543zm0 2.769l-6.4 3.695v7.39l6.4 3.695 6.4-3.695v-7.39l-6.4-3.695zm0 1.385l4.8 2.771v5.543l-4.8 2.771-4.8-2.771v-5.543l4.8-2.771z"/></svg>
                </a>
            </div>
        </header>

        <main>
            <section id="about">
                <p>"Most threat intelligence professionals come from either a policy background or a technical one. I work across both. My Master of Strategic Studies specialized in cyber warfare as an instrument of state power, which sits alongside hands-on experience in reverse engineering advanced malware, vulnerability assessment, and security architecture."</p>
                <p>"Currently deepening practical skills in malware analysis, threat hunting, and adversary tradecraft — working toward roles at the intersection of technical analysis and geopolitical context."</p>
            </section>

            <section id="publications">
                <h2>"Research & Publications"</h2>
                <div>
                    <h3>"LovelyMalware: Insane-Difficulty Malware Analysis Report"</h3>
                    <p>"Complete static, dynamic, and network forensic analysis of a PE32+ ransomware/stealer binary. Kill chain reconstruction, dual C2 architecture mapping, custom AES-256-CBC implementation analysis, and full file decryption."</p>
                    <p><span class="tag">"Malware Analysis"</span>" " <span class="tag">"Reverse Engineering"</span>" " <span class="tag">"DFIR"</span>" " <span class="tag">"Network Forensics"</span></p>
                    <a href="#">"View Report"</a>
                </div>
                <div style="margin-top:20px;">
                    <h3>"5th-Generation Fighter Exports as Strategic Competition"</h3>
                    <p>"How China is winning the middle-power game in Asia-Pacific through platform exports while US export controls create reverse leverage. FC-31 proliferation as a leading indicator for regional alignment shifts."</p>
                    <p><span class="tag">"Threat Intelligence"</span>" " <span class="tag">"Geopolitics"</span>" " <span class="tag">"APAC"</span>" " <span class="tag">"National Security"</span></p>
                    <a href="#">"Read Analysis"</a>
                </div>
                <div style="margin-top:20px;">
                    <h3>"Computational Mathematics & Cryptography Curriculum"</h3>
                    <p>"Open-source implementation of foundational cryptographic algorithms from first principles in Julia. Includes Chinese Remainder Theorem, Extended Euclidean Algorithm, and Håstad broadcast attack simulation."</p>
                    <p><span class="tag">"Cryptography"</span>" " <span class="tag">"Julia"</span>" " <span class="tag">"Open Source"</span></p>
                    <a href="#">"View on GitHub"</a>
                </div>
            </section>

            <section id="projects">
                <h2>"Active Projects"</h2>
                <div class="project-grid">
                    <div class="project-card">
                        <h3>"Project Hliðskjálf"</h3>
                        <p>"Bare-metal Type-1.5 Rust hypervisor for legacy x86_64 systems. Hardware-enforced security retrofit without reboots."</p>
                        <a href="#">"GitHub"</a>
                    </div>
                    <div class="project-card">
                        <h3>"SIEM Ensemble"</h3>
                        <p>"High-velocity polyglot log analytics engine. Rust core, Zig forwarder, Elixir orchestration. Nanosecond-level ingestion."</p>
                        <a href="#">"GitHub"</a>
                    </div>
                    <div class="project-card">
                        <h3>"Geospatial Intelligence Engine"</h3>
                        <p>"Real-time polyglot telemetry server streaming OpenSky and AIS maritime feeds via H3 hexagonal indexing."</p>
                        <a href="#">"GitHub"</a>
                    </div>
                    <div class="project-card">
                        <h3>"Julia Malware Classifier"</h3>
                        <p>"High-performance malware classification pipeline in Julia. 88.17% accuracy on EMBER 2018 dataset."</p>
                        <a href="#">"GitHub"</a>
                    </div>
                </div>
            </section>

            <section id="certs">
                <h2>"Certifications"</h2>
                <ul class="certs">
                    <li>"SOC Level 2 · TryHackMe · May 2026"</li>
                    <li>"SOC Level 1 · TryHackMe · Apr 2026"</li>
                    <li>"Google Cybersecurity Certificate · Apr 2024"</li>
                    <li>"CompTIA Security+ · In Progress"</li>
                </ul>
            </section>
        </main>

        <footer>
            "Jack Weekly"<br />
            "Wellington, New Zealand · Open to threat intelligence and cybersecurity roles"
        </footer>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
