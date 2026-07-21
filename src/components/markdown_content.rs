use leptos::*;
use leptos::html::Div;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlAnchorElement, MouseEvent};

use crate::utils::script_loader;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = renderMarkdownMath)]
    fn render_markdown_math(element: &web_sys::Element);
}

/// `<base href="/Research-Publications/">` makes bare `#fn-…` links resolve to the site
/// root. Rewrite them against the current pathname and handle clicks locally so the
/// router does not navigate away from the document.
fn rewrite_fragment_hrefs(root: &Element) {
    let Ok(links) = root.query_selector_all("a[href^='#']") else {
        return;
    };
    let pathname = web_sys::window()
        .and_then(|window| window.location().pathname().ok())
        .unwrap_or_default();
    if pathname.is_empty() {
        return;
    }

    for index in 0..links.length() {
        let Some(node) = links.item(index) else {
            continue;
        };
        let Ok(anchor) = node.dyn_into::<HtmlAnchorElement>() else {
            continue;
        };
        let Some(href) = anchor.get_attribute("href") else {
            continue;
        };
        if !href.starts_with('#') || href.len() < 2 {
            continue;
        }
        let _ = anchor.set_attribute("href", &format!("{pathname}{href}"));
    }
}

fn attach_fragment_click_handler(root: &Element) {
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let Some(target) = event.target() else {
            return;
        };
        let Ok(element) = target.dyn_into::<Element>() else {
            return;
        };
        let Ok(Some(anchor_el)) = element.closest("a[href*='#']") else {
            return;
        };
        let Ok(anchor) = anchor_el.dyn_into::<HtmlAnchorElement>() else {
            return;
        };
        let href = anchor.href();
        let Some((_, fragment)) = href.split_once('#') else {
            return;
        };
        if fragment.is_empty() {
            return;
        }

        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let Some(destination) = document.get_element_by_id(fragment) else {
            return;
        };

        event.prevent_default();
        event.stop_propagation();
        destination.scroll_into_view();
    }) as Box<dyn FnMut(_)>);

    let _ = root.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
    closure.forget();
}

#[component]
pub fn MarkdownContent(
    #[prop(into)] html: String,
    #[prop(default = "markdown-content")] class: &'static str,
) -> impl IntoView {
    let node_ref = create_node_ref::<Div>();
    let html = store_value(html);
    let fragment_handler_attached = store_value(false);

    create_effect(move |_| {
        let current = html.get_value();
        if current.is_empty() {
            return;
        }

        let Some(element) = node_ref.get() else {
            return;
        };

        rewrite_fragment_hrefs(&element);
        if !fragment_handler_attached.get_value() {
            attach_fragment_click_handler(&element);
            fragment_handler_attached.set_value(true);
        }

        spawn_local(async move {
            script_loader::ensure_katex().await;
            render_markdown_math(&element);
            // Re-apply after KaTeX may have touched the DOM; also covers first-paint races
            // where inner_html lands just after the synchronous rewrite above.
            rewrite_fragment_hrefs(&element);
        });
    });

    view! {
        <div class=class node_ref=node_ref inner_html=move || html.get_value() />
    }
}
