use leptos::*;
use leptos_router::*;
use crate::projects::find_by_slug;
use crate::utils::{
    contact_href, curriculum_href, malware_reports_href, project_href, projects_index_href,
    report_href, start_here_href, tidy_tuesday_href, tidy_tuesday_index_href, malware_reports_index_href,
};

fn project_tag(slug: &str) -> &'static str {
    find_by_slug(slug).expect("registered project").tag
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <header class="home-flagship" id="flagship" aria-labelledby="flagship-title">
                <p class="home-flagship-kicker">"Flagship working paper · WIP"</p>
                <h2 id="flagship-title" class="home-flagship-title">
                    "Cyber, neo-medievalism, and panoptic rails"
                </h2>
                <p class="home-flagship-deck">
                    "Overlapping authority in cyberspace. Bull, Rid, and Farrell and Newman on weaponized interdependence, "
                    "with SWIFT as a comparative hub and New Zealand as a middle-power assurance problem."
                </p>
                <div class="home-hero-actions">
                    <A href=report_href("cyber-neomedievalism") class="home-hero-action home-hero-action-primary">
                        "Read the working paper"
                    </A>
                    <a href=start_here_href() class="home-hero-action">"Then the supporting samples"</a>
                </div>
                <p class="home-flagship-note">
                    "Unfinished on purpose. This is the centre of the portfolio — the analytic lane at full stretch, "
                    "not a final judgment."
                </p>
            </header>

            <section class="home-section home-about" aria-labelledby="about-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Who this is"</p>
                    <h2 id="about-heading" class="home-section-title">"Jack Weekly"</h2>
                    <p class="home-section-desc">
                        "Dual US & NZ citizen · Wellington. Master of Strategic Studies (with Merit). "
                        "Technical work in cyber forensics, security engineering, and applied cryptography as proof of method. "
                        "Early career. I ship work to be scrutinised, not to claim seniority."
                    </p>
                </div>
            </section>

            <section class="home-section home-start-section" id="start-here" aria-labelledby="start-here-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"For reviewers"</p>
                    <h2 id="start-here-heading" class="home-section-title">"After the paper"</h2>
                    <p class="home-section-desc">
                        "Three samples that show technical method and tooling beside the flagship analytic piece."
                    </p>
                </div>
                <ol class="home-start-list">
                    <li class="home-start-item">
                        <A href=report_href("wannacry") class="home-start-link">
                            <span class="home-start-track">"Technical analysis"</span>
                            <span class="home-start-title">"WannaCry · Vanguard stress test"</span>
                            <span class="home-start-body">
                                "Static malware analysis with my own triage CLI: embedded ZIP unlock, onion/BTC indicators, "
                                "and an honest account of where the tool still fails."
                            </span>
                            <span class="home-start-cta">"Open analysis sample →"</span>
                        </A>
                    </li>
                    <li class="home-start-item">
                        <A href=project_href("vanguard-re") class="home-start-link">
                            <span class="home-start-track">"Defensive tooling"</span>
                            <span class="home-start-title">"Vanguard-RE · static malware triage CLI"</span>
                            <span class="home-start-body">
                                "Memory-safe Rust CLI for PE/ELF/Mach-O triage: ImpHash clustering, delay-load http_client "
                                "scoring, weak XOR recovery, bomb-bounded in-memory ZIP quarantine, and recursive unpacking."
                            </span>
                            <span class="home-start-cta">"Open tooling sample →"</span>
                        </A>
                    </li>
                    <li class="home-start-item">
                        <A href=project_href("casre") class="home-start-link">
                            <span class="home-start-track">"Systems engineering"</span>
                            <span class="home-start-title">"CASRE · recon & phishing campaign graphs"</span>
                            <span class="home-start-body">
                                "High-speed Go CLI that follows ESP → cloaker → lander chains with verdict scoring, "
                                "MITRE tags, and IOC export — a concrete engineering sample of operational tooling."
                            </span>
                            <span class="home-start-cta">"Open engineering sample →"</span>
                        </A>
                    </li>
                </ol>
                <p class="home-start-footer">
                    "Prefer a single PDF or a conversation about role fit? "
                    <A href=contact_href() class="home-start-footer-link">"Contact"</A>
                    " · "
                    <A href=report_href("conti-locker") class="home-start-footer-link">"Conti analysis"</A>
                    " · "
                    <A href=report_href("thesis-summary") class="home-start-footer-link">"MStrat writing sample"</A>
                    " · "
                    <A href=curriculum_href() class="home-start-footer-link">"Crypto curriculum"</A>
                </p>
            </section>

            <section class="home-section" id="research" aria-labelledby="research-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Finished products"</p>
                    <h2 id="research-heading" class="home-section-title">"Technical analysis & reporting"</h2>
                    <p class="home-section-desc">
                        "Analytic writing with evidence, caveats, and defensive outputs. "
                        "Raccoon is the current stealer / Vanguard stress test (WIP). WannaCry and Conti are the finished ransomware baselines."
                    </p>
                </div>
                <div class="home-card-grid">
                    <A href=report_href("raccoon-stealer") class="home-card home-card-link home-card-wip">
                        <div class="home-card-meta">
                            <span class="home-tag">"Malware analysis · WIP"</span>
                            <time class="home-date" datetime="2026-07">"Jul 2026"</time>
                        </div>
                        <h3 class="home-card-title">"Raccoon Stealer v2 · Vanguard stress test"</h3>
                        <p class="home-card-body">
                            "Static pass on a 21-member stealer pack: ImpHash clustering, thin IAT / delay-load WinINet, "
                            "and a Vanguard scorecard that now promotes string-resolved http_client evidence."
                        </p>
                        <span class="home-card-cta">"READ WIP REPORT →"</span>
                    </A>
                    <A href=report_href("wannacry") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">"Malware analysis"</span>
                            <time class="home-date" datetime="2026-07">"Jul 2026"</time>
                        </div>
                        <h3 class="home-card-title">"WannaCry Vanguard stress test"</h3>
                        <p class="home-card-body">
                            "Deep static pass on WannaCry with Vanguard-RE: in-memory embedded ZIP unlock, WNcry@2ol7 recovery, "
                            "Tor onion / BTC harvest, plus honest failure modes on .NET and ELF scoring."
                        </p>
                        <span class="home-card-cta">"READ REPORT →"</span>
                    </A>
                    <A href=report_href("conti-locker") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">"Malware analysis"</span>
                            <time class="home-date" datetime="2026-07">"Jul 2026"</time>
                        </div>
                        <h3 class="home-card-title">"Conti Locker v2 analysis"</h3>
                        <p class="home-card-body">
                            "Static reverse-engineering of the leaked Conti ransomware build tree with Vanguard-RE: "
                            "hybrid ChaCha20 + RSA crypto, SMB spreading, and static inferences checked against leaked source."
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
                    <A href=report_href("lovely-malware") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">"Malware analysis"</span>
                            <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                        </div>
                        <h3 class="home-card-title">"LovelyMalware analysis report"</h3>
                        <p class="home-card-body">
                            "Forensic report on a PE32+ ransomware sample: static and dynamic analysis, PCAP review, "
                            "decryption, Sigma rules, and IOC packages suitable for operational use."
                        </p>
                        <span class="home-card-cta">"READ REPORT →"</span>
                    </A>
                    <A href=malware_reports_href("smartapesg-2026-05-22") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">"Malware traffic"</span>
                            <time class="home-date" datetime="2026-05-22">"May 2026"</time>
                        </div>
                        <h3 class="home-card-title">"SmartApeSG ClickFix → NetSupport RAT"</h3>
                        <p class="home-card-body">
                            "Interactive PCAP analysis of a ClickFix infection chain: compromised WordPress site, PowerShell staging, "
                            "33 MB ZIP payload, unidentified RAT C2, and NetSupport persistence."
                        </p>
                        <span class="home-card-cta">"OPEN ANALYSIS →"</span>
                    </A>
                </div>
                <p class="home-section-footer">
                    <A href=malware_reports_index_href() class="home-start-footer-link">"All malware reports →"</A>
                </p>
            </section>

            <section class="home-section home-section-featured" id="projects" aria-labelledby="projects-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Selected engineering"</p>
                    <h2 id="projects-heading" class="home-section-title">"Tools built to be evaluated"</h2>
                    <p class="home-section-desc">
                        "Operational tooling with clear scope and reviewable outputs. These are the engineering samples "
                        "I want judged on method and containment, not on ambition alone."
                    </p>
                </div>
                <div class="home-card-grid">
                    <A href=project_href("casre") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">{project_tag("casre")}</span>
                            <time class="home-date" datetime="2026-07">"Jul 2026"</time>
                        </div>
                        <h3 class="home-card-title">"CASRE"</h3>
                        <p class="home-card-body">
                            "Concurrent Go CLI for host recon and phishing URL campaigns: DNS/TLS/HTTP enrichment, "
                            "hop graphs with role classification, MITRE tags, verdict scoring, and IOC export."
                        </p>
                        <span class="home-card-cta">"VIEW PROJECT →"</span>
                    </A>
                    <A href=project_href("vanguard-re") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">{project_tag("vanguard-re")}</span>
                            <time class="home-date" datetime="2026-07">"Jul 2026"</time>
                        </div>
                        <h3 class="home-card-title">"Vanguard-RE"</h3>
                        <p class="home-card-body">
                            "Rust CLI for static malware triage: formal PE/ELF/Mach-O parsing, ImpHash clusters, "
                            "thin-IAT stealer scoring, weak XOR recovery, and bomb-bounded in-memory quarantine."
                        </p>
                        <span class="home-card-cta">"VIEW PROJECT →"</span>
                    </A>
                    <A href=project_href("net-honeynet") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">{project_tag("net-honeynet")}</span>
                            <time class="home-date" datetime="2026-07">"Jul 2026"</time>
                        </div>
                        <h3 class="home-card-title">"Net Honeynet"</h3>
                        <p class="home-card-body">
                            "Medium-interaction Rust honeynet emulating SSH, HTTP, SMTP, and SMB with a collector "
                            "pipeline to file or S3 and an intel-export CLI for malicious IP scoring."
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

            <section class="home-section" id="exploratory" aria-labelledby="exploratory-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Exploratory"</p>
                    <h2 id="exploratory-heading" class="home-section-title">"Ongoing systems experiments"</h2>
                    <p class="home-section-desc">
                        "Ambitious work still under construction. Interesting for range, but not the primary evidence "
                        "for how I perform under review. Treat these as learning projects, not finished claims."
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
                            "Exploratory Type-1.5 hypervisor research for EPT-backed isolation and host hardening on legacy x86_64. "
                            "Active learning project, not a production claim."
                        </p>
                        <span class="home-card-cta">"VIEW NOTES →"</span>
                    </A>
                    <A href=project_href("siem-ensemble") class="home-card home-card-link">
                        <div class="home-card-meta">
                            <span class="home-tag">{project_tag("siem-ensemble")}</span>
                            <time class="home-date" datetime="2026-04">"Apr 2026"</time>
                        </div>
                        <h3 class="home-card-title">"SIEM Ensemble"</h3>
                        <p class="home-card-body">
                            "Polyglot log-pipeline experiment across Rust, Zig, Odin, and Elixir. Useful for systems practice; "
                            "still exploratory rather than a finished detection product."
                        </p>
                        <span class="home-card-cta">"VIEW NOTES →"</span>
                    </A>
                </div>
                <p class="home-section-footer">
                    <A href=projects_index_href() class="home-start-footer-link">"All projects →"</A>
                </p>
            </section>

            <section class="home-section" id="education" aria-labelledby="education-heading">
                <div class="home-section-header">
                    <p class="home-section-kicker">"Analytic foundation"</p>
                    <h2 id="education-heading" class="home-section-title">"Education"</h2>
                    <p class="home-section-desc">
                        "A Master of Strategic Studies with Merit in strategic competition and statecraft, paired with undergraduate work in international business."
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
                    <p class="home-section-kicker">"Foundations"</p>
                    <h2 id="curriculum-heading" class="home-section-title">"Julia cryptography curriculum"</h2>
                    <p class="home-section-desc">
                        "An eight-module track covering number theory, primes, and RSA. Each module includes theory notes and an interactive browser lab "
                        "with exercises and automated verification. Useful background, not the primary review sample."
                    </p>
                </div>
                <A href=curriculum_href() class="home-cta">"Browse curriculum & labs"</A>
            </section>
        </div>
    }
}
