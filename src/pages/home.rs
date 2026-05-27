use leptos::*;
use leptos_router::*;
use crate::components::stack_matrix::StackMatrix;
use crate::utils::{curriculum_href, report_href};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <header class="home-hero">
                <p class="home-eyebrow">"Dual US & NZ citizen · Wellington, New Zealand"</p>
                <h2 class="home-title">"Strategic analysis and technical collection in one analyst."</h2>
                <p class="home-lead">
                    "I'm Jack Weekly. I work across malware forensics, threat reporting, and security engineering, "
                    "with graduate training in coercive statecraft and Asia-Pacific security competition. "
                    "This site is a sample of analytic writing, operational technical work, and applied cryptography research."
                </p>
                <ul class="home-highlights" aria-label="Core focus areas">
                    <li class="home-highlight">
                        <span class="home-highlight-label">"Analysis"</span>
                        <span class="home-highlight-text">"Malware forensics, kill-chain reconstruction, and CTI-style reporting"</span>
                    </li>
                    <li class="home-highlight">
                        <span class="home-highlight-label">"Engineering"</span>
                        <span class="home-highlight-text">"Collection pipelines and security tooling in Rust, Zig, and Elixir"</span>
                    </li>
                    <li class="home-highlight">
                        <span class="home-highlight-label">"Research"</span>
                        <span class="home-highlight-text">"Strategic studies and applied cryptography foundations"</span>
                    </li>
                </ul>
            </header>

            <section class="home-section" id="research" aria-labelledby="research-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Intelligence products"</p>
                    <h2 id="research-heading" class="home-section-title">"Technical analysis & reporting"</h2>
                    <p class="home-section-desc">
                        "Finished analytic work—from malware forensics to strategic studies writing samples."
                    </p>
                </div>
                <div class="home-card-grid">
                    <A href=report_href("lovely-malware") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">"Malware analysis"</span>
                            <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                        </div>
                        <h3 class="home-card-title">"LovelyMalware analysis report"</h3>
                        <p class="home-card-body">
                            "Intelligence-style forensic report on a PE32+ ransomware sample: static and dynamic analysis, PCAP review, "
                            "decryption, Sigma rules, and IOC packages suitable for operational use."
                        </p>
                        <span class="home-card-cta">"READ REPORT →"</span>
                    </A>
                    <A href=report_href("thesis-summary") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">"Strategic analysis"</span>
                            <time class="home-date" datetime="2024">"2024"</time>
                        </div>
                        <h3 class="home-card-title">"5th-gen fighter exports & APAC middle-power competition"</h3>
                        <p class="home-card-body">
                            "Condensed writing sample from MStrat thesis work: how US export controls and China's FC-31 programme "
                            "reshape alignment choices for Indonesia, Malaysia, Pakistan, and other APAC middle powers."
                        </p>
                        <span class="home-card-cta">"READ ESSAY →"</span>
                    </A>
                </div>
            </section>

            <section class="home-section" id="projects" aria-labelledby="projects-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Operational tooling"</p>
                    <h2 id="projects-heading" class="home-section-title">"Engineering projects"</h2>
                    <p class="home-section-desc">
                        "Systems work oriented toward collection, enrichment, isolation, and security operations at scale."
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
                            "Bare-metal Type-1.5 hypervisor research focused on isolation, trusted execution boundaries, and low-level systems security."
                        </p>
                    </article>
                    <article class="home-card">
                        <div class="home-card-meta">
                            <span class="home-tag">"Rust · Elixir"</span>
                            <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                        </div>
                        <h3 class="home-card-title">"SIEM Ensemble"</h3>
                        <p class="home-card-body">
                            "High-throughput log analytics for detection workflows: ingestion, enrichment, correlation, and query performance."
                        </p>
                    </article>
                </div>
            </section>

            <section class="home-section" id="education" aria-labelledby="education-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Analytic foundation"</p>
                    <h2 id="education-heading" class="home-section-title">"Education"</h2>
                    <p class="home-section-desc">
                        "Graduate training in strategic competition and statecraft, paired with undergraduate work in international business."
                    </p>
                </div>
                <ul class="home-cred-list">
                    <li>
                        <a href=report_href("thesis-summary") class="home-cred home-cred-link">
                            <span class="home-cred-type">"Graduate degree"</span>
                            <span class="home-cred-provider">"Victoria University of Wellington"</span>
                            <span class="home-cred-name">"Master of Strategic Studies"</span>
                            <span class="home-cred-detail">"Awarded with Merit · Thesis: Evaluating US and Chinese 5th-Generation Aerospace Capabilities as Instruments of Coercive Statecraft in Asia-Pacific"</span>
                        </a>
                    </li>
                    <li>
                        <div class="home-cred">
                            <span class="home-cred-type">"Undergraduate degree"</span>
                            <span class="home-cred-provider">"University of Auckland"</span>
                            <span class="home-cred-name">"Bachelor of Commerce (BCom)"</span>
                            <span class="home-cred-detail">"Double major: International Business · Innovation & Entrepreneurship"</span>
                        </div>
                    </li>
                </ul>
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
