use leptos::*;

/// Brief eligibility summary for recruiters. Kept minimal on purpose.
#[component]
pub fn ClearanceBlock() -> impl IntoView {
    view! {
        <section class="clearance-block" aria-labelledby="clearance-block-heading">
            <h3 id="clearance-block-heading" class="clearance-block-title">
                "Background & eligibility"
            </h3>
            <p class="clearance-block-intro">
                "Dual citizen based in Wellington. Open to national security and intelligence work "
                "in New Zealand and internationally."
            </p>
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
                    <dt>"Location"</dt>
                    <dd>
                        "Wellington, New Zealand. Open to roles across NZ and internationally, including relocation."
                    </dd>
                </div>
                <div class="clearance-block-row">
                    <dt>"Screening"</dt>
                    <dd>
                        "Willing to complete employer-required personnel security screening, including "
                        "NZSIS vetting for New Zealand government and national security roles, and equivalent "
                        "United States or allied-nation processes where applicable."
                    </dd>
                </div>
            </dl>
        </section>
    }
}
