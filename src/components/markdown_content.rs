use leptos::*;
use leptos::html::Div;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = renderMarkdownMath)]
    fn render_markdown_math(element: &web_sys::Element);
}

#[component]
pub fn MarkdownContent(
    #[prop(into)] html: String,
    #[prop(default = "markdown-content")] class: &'static str,
) -> impl IntoView {
    let node_ref = create_node_ref::<Div>();
    let html = store_value(html);

    create_effect(move |_| {
        let current = html.get_value();
        if current.is_empty() {
            return;
        }
        if let Some(element) = node_ref.get() {
            render_markdown_math(&element);
        }
    });

    view! {
        <div class=class node_ref=node_ref inner_html=move || html.get_value() />
    }
}
