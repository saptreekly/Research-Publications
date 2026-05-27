use leptos::*;
use comrak::{markdown_to_html, Options};

#[component]
pub fn TechnicalDocument(src: &'static str) -> impl IntoView {
    let content = create_resource(move || src, |src| async move {
        gloo_net::http::Request::get(src)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap_or_else(|_| "Error loading document".to_string())
    });

    view! {
        <Suspense fallback=move || view! { <p>"Loading module..."</p> }>
            {move || content.get().map(|md| {
                let options = Options::default();
                let html = markdown_to_html(&md, &options);
                view! { <div class="markdown-content" inner_html=html /> }
            })}
        </Suspense>
    }
}
