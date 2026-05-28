use leptos::*;
use crate::components::clearance_block::ClearanceBlock;
use crate::components::contact_form::ContactForm;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <section class="contact-page">
            <header class="contact-page-header">
                <p class="home-section-kicker">"Get in touch"</p>
                <h2 class="contact-page-title">"Contact"</h2>
                <p class="contact-page-desc">
                    "Professional inquiries welcome, including national security and intelligence roles "
                    "in New Zealand and internationally."
                </p>
            </header>
            <ClearanceBlock />
            <ContactForm />
        </section>
    }
}
