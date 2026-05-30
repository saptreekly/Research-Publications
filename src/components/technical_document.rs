use leptos::*;
use crate::components::markdown_content::MarkdownContent;
use crate::utils::{is_html_content, markdown::rendered_html_path, resolve_asset_url};

#[component]
pub fn TechnicalDocument(src: &'static str) -> impl IntoView {
    let content = create_resource(move || src, |src| async move {
        let url = resolve_asset_url(&rendered_html_path(src));
        let response = match gloo_net::http::Request::get(&url).send().await {
            Ok(response) => response,
            Err(_) => return Err("Unable to reach document source.".to_string()),
        };

        if !response.ok() {
            return Err(format!("Document not found ({})", response.status()));
        }

        let text = match response.text().await {
            Ok(text) => text,
            Err(_) => return Err("Unable to read document contents.".to_string()),
        };

        if is_html_content(&text) {
            return Ok(text);
        }

        Err("Document was not pre-rendered to HTML. Run cargo build to regenerate static/rendered assets.".to_string())
    });

    view! {
        <Suspense fallback=move || view! { <p>"Loading module..."</p> }>
            {move || content.get().map(|result| match result {
                Ok(html) => view! {
                    <MarkdownContent html=html />
                }.into_view(),
                Err(message) => view! {
                    <p class="doc-error">{message.clone()}</p>
                }.into_view(),
            })}
        </Suspense>
    }
}
