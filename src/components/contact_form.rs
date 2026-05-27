use leptos::*;
use serde::{Deserialize, Serialize};

const WEB3FORMS_ENDPOINT: &str = "https://api.web3forms.com/submit";

fn access_key() -> Option<&'static str> {
    option_env!("WEB3FORMS_ACCESS_KEY")
}

#[derive(Serialize)]
struct ContactPayload<'a> {
    access_key: &'a str,
    name: String,
    email: String,
    message: String,
    subject: &'a str,
    botcheck: String,
}

#[derive(Deserialize)]
struct ContactResponse {
    success: bool,
    message: String,
}

#[component]
pub fn ContactForm() -> impl IntoView {
    let configured = access_key().is_some();
    let name = create_rw_signal(String::new());
    let email = create_rw_signal(String::new());
    let message = create_rw_signal(String::new());
    let honeypot = create_rw_signal(String::new());
    let status = create_rw_signal(Option::<(bool, String)>::None);
    let submitting = create_rw_signal(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let Some(key) = access_key() else {
            status.set(Some((
                false,
                "Contact form is not configured for this build.".to_string(),
            )));
            return;
        };

        if !honeypot.get().is_empty() {
            status.set(Some((
                true,
                "Message sent. I'll get back to you soon.".to_string(),
            )));
            return;
        }

        submitting.set(true);
        status.set(None);

        let payload = ContactPayload {
            access_key: key,
            name: name.get(),
            email: email.get(),
            message: message.get(),
            subject: "Portfolio contact · Jack Weekly",
            botcheck: honeypot.get(),
        };

        spawn_local(async move {
            let result: Result<ContactResponse, String> = async {
                let request = gloo_net::http::Request::post(WEB3FORMS_ENDPOINT)
                    .json(&payload)
                    .map_err(|_| "Unable to prepare contact request.".to_string())?;

                let response = request
                    .send()
                    .await
                    .map_err(|_| "Unable to reach the contact service.".to_string())?;

                response
                    .json::<ContactResponse>()
                    .await
                    .map_err(|_| "Unexpected response from the contact service.".to_string())
            }
            .await;

            submitting.set(false);

            match result {
                Ok(body) if body.success => {
                    name.set(String::new());
                    email.set(String::new());
                    message.set(String::new());
                    status.set(Some((
                        true,
                        "Message sent. I'll get back to you soon.".to_string(),
                    )));
                }
                Ok(body) => status.set(Some((false, body.message))),
                Err(message) => status.set(Some((false, message))),
            }
        });
    };

    view! {
        <form class="contact-form" on:submit=on_submit>
            {(!configured).then(|| view! {
                <p class="contact-form-note">
                    "Local builds need WEB3FORMS_ACCESS_KEY set when running trunk."
                </p>
            })}

            <div class="contact-form-grid">
                <label class="contact-field">
                    <span class="contact-label">"Name"</span>
                    <input
                        type="text"
                        name="name"
                        class="contact-input"
                        required
                        autocomplete="name"
                        prop:value=move || name.get()
                        on:input=move |ev| name.set(event_target_value(&ev))
                    />
                </label>

                <label class="contact-field">
                    <span class="contact-label">"Email"</span>
                    <input
                        type="email"
                        name="email"
                        class="contact-input"
                        required
                        autocomplete="email"
                        prop:value=move || email.get()
                        on:input=move |ev| email.set(event_target_value(&ev))
                    />
                </label>
            </div>

            <label class="contact-field">
                <span class="contact-label">"Message"</span>
                <textarea
                    name="message"
                    class="contact-input contact-textarea"
                    required
                    rows="6"
                    prop:value=move || message.get()
                    on:input=move |ev| message.set(event_target_value(&ev))
                ></textarea>
            </label>

            <label class="contact-honeypot" aria-hidden="true">
                <span>"Leave blank"</span>
                <input
                    type="text"
                    name="botcheck"
                    tabindex="-1"
                    autocomplete="off"
                    prop:value=move || honeypot.get()
                    on:input=move |ev| honeypot.set(event_target_value(&ev))
                />
            </label>

            <div class="contact-form-actions">
                <button
                    type="submit"
                    class="home-cta contact-submit"
                    disabled=move || submitting.get() || !configured
                >
                    {move || if submitting.get() { "Sending..." } else { "Send message" }}
                </button>

                {move || status.get().map(|(success, message)| view! {
                    <p
                        class="contact-status"
                        class:contact-status-success=success
                        class:contact-status-error=!success
                        role="status"
                    >
                        {message}
                    </p>
                })}
            </div>
        </form>
    }
}
