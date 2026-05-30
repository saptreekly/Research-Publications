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
                        "United States (born Portland, Oregon; citizen since birth). "
                        "New Zealand (naturalised 2025). Dual national."
                    </dd>
                </div>
                <div class="clearance-block-row">
                    <dt>"Residency"</dt>
                    <dd>
                        "New Zealand permanent resident since 2008. Resided only in the United States "
                        "and New Zealand; brief leisure visits to Australia."
                    </dd>
                </div>
                <div class="clearance-block-row">
                    <dt>"Driver's licence"</dt>
                    <dd>"Full New Zealand licence (since 2019)."</dd>
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
                        "Eligible for Top Secret Special (TSS) vetting under NZ residency since 2008 "
                        "with prior US citizenship. Willing to complete employer-required personnel security "
                        "screening, including NZSIS vetting and equivalent allied-nation processes where applicable."
                    </dd>
                </div>
            </dl>
        </section>
    }
}
