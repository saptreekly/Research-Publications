use leptos::*;
use leptos_router::*;
use crate::components::stack_matrix::StackMatrix;
use crate::projects::find_by_slug;
use crate::utils::{contact_href, curriculum_href, lab_href, project_href, report_href, situation_monitor_href, tidy_tuesday_href, tidy_tuesday_index_href};

fn project_tag(slug: &str) -> &'static str {
    find_by_slug(slug).expect("registered project").tag
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <header class="home-hero">
                <p class="home-eyebrow">"Dual US & NZ citizen · Wellington, New Zealand"</p>
                <h2 class="home-title">"All-source analysis with technical depth."</h2>
                <p class="home-lead">
                    "I'm Jack Weekly. My training is in strategic studies and Asia-Pacific security competition; "
                    "my technical work spans cyber forensics, security engineering, and applied cryptography. "
                    "I use cyber to demonstrate depth, but this portfolio is built for general intelligence work: "
                    "finished analytic writing, operational tooling, and research that connects policy to operational reality."
                </p>
                <ul class="home-highlights" aria-label="Core focus areas">
                    <li class="home-highlight">
                        <span class="home-highlight-label">"Analysis"</span>
                        <span class="home-highlight-text">"Regional security assessment, coercive statecraft, and intelligence-style reporting"</span>
                    </li>
                    <li class="home-highlight">
                        <span class="home-highlight-label">"Engineering"</span>
                        <span class="home-highlight-text">"Security tooling and data pipelines; cyber work as technical proof of tradecraft"</span>
                    </li>
                    <li class="home-highlight">
                        <span class="home-highlight-label">"Research"</span>
                        <span class="home-highlight-text">"Asia-Pacific competition, strategic studies, and cryptologic foundations"</span>
                    </li>
                </ul>
            </header>

            <section class="home-section home-start-section" id="start-here" aria-labelledby="start-here-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"For reviewers"</p>
                    <h2 id="start-here-heading" class="home-section-title">"Start here"</h2>
                    <p class="home-section-desc">
                        "Curated entry points for intelligence and national security roles. "
                        "Each link is a finished deliverable, not a project overview."
                    </p>
                </div>
                <ol class="home-start-list">
                    <li class="home-start-item">
                        <A href=report_href("lovely-malware") class="home-start-link">
                            <span class="home-start-track">"Cyber threat intelligence"</span>
                            <span class="home-start-title">"LovelyMalware forensic report"</span>
                            <span class="home-start-body">
                                "End-to-end CTI deliverable: static/dynamic forensics, network analysis, decryption, "
                                "Sigma detection rules, and IOC packages formatted for operational use."
                            </span>
                            <span class="home-start-cta">"Open CTI sample →"</span>
                        </A>
                    </li>
                    <li class="home-start-item">
                        <A href=report_href("thesis-summary") class="home-start-link">
                            <span class="home-start-track">"Strategic & all-source analysis"</span>
                            <span class="home-start-title">"5th-gen fighter exports & APAC competition"</span>
                            <span class="home-start-body">
                                "MStrat writing sample on coercive statecraft, export controls, and alignment pressure "
                                "on Indonesia, Malaysia, Pakistan, and other Asia-Pacific middle powers."
                            </span>
                            <span class="home-start-cta">"Open writing sample →"</span>
                        </A>
                    </li>
                    <li class="home-start-item">
                        <A href=project_href("geospatial-intel") class="home-start-link">
                            <span class="home-start-track">"Systems engineering"</span>
                            <span class="home-start-title">"Geospatial Intel Server"</span>
                            <span class="home-start-body">
                                "Real-time streaming architecture: OpenSky ingest, viewport-filtered WebSocket streaming, "
                                "H3 spatial indexing, and delta updates for sub-100k entity workloads."
                            </span>
                            <span class="home-start-cta">"Open engineering sample →"</span>
                        </A>
                    </li>
                    <li class="home-start-item">
                        <A href=lab_href("mod-01") class="home-start-link">
                            <span class="home-start-track">"Applied cryptography & tradecraft depth"</span>
                            <span class="home-start-title">"Interactive modular arithmetic lab"</span>
                            <span class="home-start-body">
                                "Browser-based Julia lab with automated verification. Demonstrates math foundations "
                                "relevant to cryptologic and cyber operations work (inverses, modular arithmetic)."
                            </span>
                            <span class="home-start-cta">"Run live lab →"</span>
                        </A>
                    </li>
                </ol>
                <p class="home-start-footer">
                    "Prefer a single PDF or clearance conversation? "
                    <A href=contact_href() class="home-start-footer-link">"Contact"</A>
                    " · "
                    <A href=curriculum_href() class="home-start-footer-link">"Full curriculum"</A>
                </p>
            </section>

            <section class="home-section" id="research" aria-labelledby="research-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Intelligence products"</p>
                    <h2 id="research-heading" class="home-section-title">"Technical analysis & reporting"</h2>
                    <p class="home-section-desc">
                        "Finished analytic work, from malware forensics to strategic studies writing samples."
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
                        "Systems work oriented toward ingestion, enrichment, isolation, and security operations at scale."
                    </p>
                </div>
                <div class="home-card-grid">
                    <A href=project_href("hlidskjalf") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">{project_tag("hlidskjalf")}</span>
                            <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                        </div>
                        <h3 class="home-card-title">"Project Hliðskjálf"</h3>
                        <p class="home-card-body">
                            "Bare-metal Type-1.5 hypervisor that virtualizes a live x86_64 host at Ring -1 for EPT-backed isolation, "
                            "kernel write-protection, and anti-evasion monitoring on legacy Windows systems."
                        </p>
                        <span class="home-card-cta">"VIEW PROJECT →"</span>
                    </A>
                    <A href=project_href("siem-ensemble") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">{project_tag("siem-ensemble")}</span>
                            <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                        </div>
                        <h3 class="home-card-title">"SIEM Ensemble"</h3>
                        <p class="home-card-body">
                            "Polyglot log pipeline with Rust ingestion, Zig shared-memory forwarding, Odin analytics, "
                            "and Elixir supervision for high-throughput detection workflows."
                        </p>
                        <span class="home-card-cta">"VIEW PROJECT →"</span>
                    </A>
                    <A href=project_href("geospatial-intel") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">{project_tag("geospatial-intel")}</span>
                            <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                        </div>
                        <h3 class="home-card-title">"Geospatial Intel Server"</h3>
                        <p class="home-card-body">
                            "WebSocket streaming server for OpenSky aircraft data with a Go backend, Rust spatial engine, "
                            "and Wasm frontend. Viewport filtering, H3 hex clustering, and delta updates for geospatial monitoring."
                        </p>
                        <span class="home-card-cta">"VIEW PROJECT →"</span>
                    </A>
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

            <section class="home-section" id="tidy-tuesday" aria-labelledby="tidy-tuesday-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Julia data analysis"</p>
                    <h2 id="tidy-tuesday-heading" class="home-section-title">"Tidy Tuesday"</h2>
                    <p class="home-section-desc">
                        "Weekly explorations of community datasets in Julia — data cleaning, visualization, and reproducible analysis."
                    </p>
                </div>
                <div class="home-card-grid">
                    <A href=tidy_tuesday_href("se4all-2026-05-26") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">"Data analysis"</span>
                            <time class="home-date" datetime="2026-05-26">"May 2026"</time>
                        </div>
                        <h3 class="home-card-title">"Sustainable Energy for All"</h3>
                        <p class="home-card-body">
                            "SE4ALL country-level energy metrics: renewable adoption rates, wind consumption in Nordic leaders, "
                            "and solar access gaps — analyzed with DataFrames.jl and Plots.jl."
                        </p>
                        <span class="home-card-cta">"READ ANALYSIS →"</span>
                    </A>
                </div>
                <A href=tidy_tuesday_index_href() class="home-cta">"Browse all Tidy Tuesday entries"</A>
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
