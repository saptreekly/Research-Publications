use leptos::*;
use comrak::{markdown_to_html, Options};

#[component]
pub fn TechnicalDocument(content: &'static str) -> impl IntoView {
    let options = Options::default();
    let html = markdown_to_html(content, &options);

    view! {
        <div class="markdown-content" inner_html=html />
    }
}
