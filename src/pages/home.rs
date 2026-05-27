use leptos::*;
use leptos_router::*;
use crate::components::stack_matrix::StackMatrix;
use crate::utils::{curriculum_href, report_href};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <header class="home-hero">
                <p class="home-eyebrow">"Cybersecurity researcher · Wellington, New Zealand"</p>
                <h2 class="home-title">"Policy depth and technical execution in the same portfolio."</h2>
                <p class="home-lead">
                    "I'm Jack Weekly. I work across threat intelligence, malware analysis, and security engineering, "
                    "with graduate training in cyber warfare as statecraft and day-to-day experience in reverse engineering, "
                    "vulnerability assessment, and security architecture."
                </p>
                <ul class="home-highlights" aria-label="Core focus areas">
                    <li class="home-highlight">
                        <span class="home-highlight-label">"Analysis"</span>
                        <span class="home-highlight-text">"Malware reverse engineering and forensic reporting"</span>
                    </li>
                    <li class="home-highlight">
                        <span class="home-highlight-label">"Engineering"</span>
                        <span class="home-highlight-text">"Security tooling in Rust, Zig, and Elixir"</span>
                    </li>
                    <li class="home-highlight">
                        <span class="home-highlight-label">"Research"</span>
                        <span class="home-highlight-text">"Applied cryptography curriculum built in Julia"</span>
                    </li>
                </ul>
            </header>

            <section class="home-section" id="research" aria-labelledby="research-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Featured work"</p>
                    <h2 id="research-heading" class="home-section-title">"Research & analysis"</h2>
                    <p class="home-section-desc">
                        "Published investigative work demonstrating end-to-end technical depth, from binary triage to decryption and reporting."
                    </p>
                </div>
                <A href=report_href("lovely-malware") class="home-card home-card-link">
                    <div class="home-card-meta">
                        <span class="home-tag">"Malware analysis"</span>
                        <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                    </div>
                    <h3 class="home-card-title">"LovelyMalware analysis report"</h3>
                    <p class="home-card-body">
                        "Forensic write-up of a PE32+ ransomware sample: kill chain reconstruction, custom AES-256-CBC usage, "
                        "and successful file recovery. Shows practical reverse engineering and documentation skills."
                    </p>
                    <span class="home-card-cta">"READ REPORT →"</span>
                </A>
            </section>

            <section class="home-section" id="projects" aria-labelledby="projects-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Build portfolio"</p>
                    <h2 id="projects-heading" class="home-section-title">"Engineering projects"</h2>
                    <p class="home-section-desc">
                        "Longer-horizon systems work: low-level infrastructure and data pipelines relevant to security operations."
                    </p>
                </div>
                <div class="home-card-grid">
                    <article class="home-card">
                        <div class="home-card-meta">
                            <span class="home-tag">"Rust · Zig"</span>
                            <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                        </div>
                        <h3 class="home-card-title">"Project Hliðskjálf"</h3>
                        <p class="home-card-body">
                            "Bare-metal Type-1.5 hypervisor research. Focus on systems programming, isolation, and low-level security primitives."
                        </p>
                    </article>
                    <article class="home-card">
                        <div class="home-card-meta">
                            <span class="home-tag">"Rust · Elixir"</span>
                            <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                        </div>
                        <h3 class="home-card-title">"SIEM Ensemble"</h3>
                        <p class="home-card-body">
                            "High-throughput log analytics engine for security monitoring workflows: ingestion, enrichment, and query performance."
                        </p>
                    </article>
                </div>
            </section>

            <section class="home-section" id="credentials" aria-labelledby="credentials-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Verified training"</p>
                    <h2 id="credentials-heading" class="home-section-title">"Certifications & training"</h2>
                    <p class="home-section-desc">
                        "Completed credentials and training paths, plus certifications currently in progress. "
                        "Completed items link to verifiable records; in-progress items link to the official exam pages."
                    </p>
                </div>
                <ul class="home-cred-list">
                    <li>
                        <a
                            href="https://www.coursera.org/account/accomplishments/professional-cert/M8P6JVUJHCQ5"
                            class="home-cred home-cred-link"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <span class="home-cred-type">"Professional certificate"</span>
                            <span class="home-cred-provider">"Google · Coursera"</span>
                            <span class="home-cred-name">"Cybersecurity Professional Certificate"</span>
                        </a>
                    </li>
                    <li>
                        <a
                            href="https://tryhackme.com/certificate/THM-8VRTA9C6J9"
                            class="home-cred home-cred-link"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <span class="home-cred-type">"Training path"</span>
                            <span class="home-cred-provider">"TryHackMe"</span>
                            <span class="home-cred-name">"SOC Level 2"</span>
                        </a>
                    </li>
                    <li>
                        <a
                            href="https://tryhackme.com/certificate/THM-PKVJPYRSFS"
                            class="home-cred home-cred-link"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <span class="home-cred-type">"Training path"</span>
                            <span class="home-cred-provider">"TryHackMe"</span>
                            <span class="home-cred-name">"SOC Level 1"</span>
                        </a>
                    </li>
                    <li>
                        <a
                            href="https://www.comptia.org/certifications/security"
                            class="home-cred home-cred-link home-cred-in-progress"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <span class="home-cred-type">"In progress"</span>
                            <span class="home-cred-provider">"CompTIA"</span>
                            <span class="home-cred-name">"Security+"</span>
                        </a>
                    </li>
                    <li>
                        <a
                            href="https://www.cisco.com/site/us/en/learn/training-certifications/certifications/enterprise/ccna/index.html"
                            class="home-cred home-cred-link home-cred-in-progress"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <span class="home-cred-type">"In progress"</span>
                            <span class="home-cred-provider">"Cisco"</span>
                            <span class="home-cred-name">"CCNA"</span>
                        </a>
                    </li>
                </ul>
            </section>

            <section class="home-section" id="curriculum-link" aria-labelledby="curriculum-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Teaching material"</p>
                    <h2 id="curriculum-heading" class="home-section-title">"Julia cryptography curriculum"</h2>
                    <p class="home-section-desc">
                        "An eight-module track covering number theory, primes, and RSA. Each module includes theory notes and an interactive browser lab "
                        "with exercises and automated verification."
                    </p>
                </div>
                <A href=curriculum_href() class="home-cta">"Browse curriculum & labs"</A>
            </section>

            <section class="home-section home-section-muted" id="stack" aria-labelledby="stack-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Codebase snapshot"</p>
                    <h2 id="stack-heading" class="home-section-title">"Languages in active repos"</h2>
                    <p class="home-section-desc">
                        "Where my recent engineering and research time is concentrated across GitHub projects."
                    </p>
                </div>
                <StackMatrix />
            </section>
        </div>
    }
}
