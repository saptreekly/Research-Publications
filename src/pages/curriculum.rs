use leptos::*;
use leptos_router::*;
use crate::components::technical_document::TechnicalDocument;

#[component]
pub fn CurriculumPage() -> impl IntoView {
    view! {
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

        <section id="curriculum">
            <h2>"05 / JULIA CRYPTOGRAPHY"</h2>
            <p class="section-intro">"Module 01 — modular foundations. Open the interactive lab to run Julia code in the browser."</p>
            <A href="curriculum/lab/mod-01" class="social-link cta-link">"OPEN INTERACTIVE LAB"</A>
            <TechnicalDocument src="research-docs/julia-crypto/mod_01.md" />
        </section>
    }
}
