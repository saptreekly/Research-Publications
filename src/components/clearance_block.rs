use leptos::*;

/// Brief eligibility summary for recruiters. Kept minimal on purpose.
#[component]
pub fn ClearanceBlock() -> impl IntoView {
    view! {
        <section class="clearance-block" aria-labelledby="clearance-block-heading">
            <h3 id="clearance-block-heading" class="clearance-block-title">
                "Background & eligibility"
            </h3>
            <dl class="clearance-block-grid">
                <div class="clearance-block-row">
                    <dt>"Citizenship"</dt>
                    <dd>
                        "United States (born Portland, Oregon). New Zealand (2025). Dual national."
                    </dd>
                </div>
                <div class="clearance-block-row">
                    <dt>"Clearance"</dt>
                    <dd>"None held."</dd>
                </div>
                <div class="clearance-block-row">
                    <dt>"Screening"</dt>
                    <dd>
                        "Willing to complete personnel security screening required by an employer, "
                        "including US or allied-nation processes where applicable."
                    </dd>
                </div>
                <div class="clearance-block-row">
                    <dt>"Location"</dt>
                    <dd>"Wellington, NZ. Open to relocation."</dd>
                </div>
            </dl>
            <p class="clearance-block-footnote">
                "Resume line: "
                <code class="clearance-block-resume-line">
                    "Dual US/NZ citizen · No clearance held · Open to relocation · Willing to complete required background screening"
                </code>
            </p>
        </section>
    }
}
