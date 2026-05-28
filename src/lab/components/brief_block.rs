use leptos::*;
use crate::components::markdown_content::MarkdownContent;
use crate::utils::markdown::markdown_to_rendered_html;

#[component]
pub fn BriefBlock(
    id: String,
    title: Option<String>,
    body_md: String,
    cell_index: usize,
    selected: ReadSignal<Option<usize>>,
    on_select: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let html = store_value(markdown_to_rendered_html(&body_md));
    let is_selected = move || selected.get() == Some(cell_index);

    view! {
        <article
            class="lab-block"
            class:lab-block-selected=is_selected
            on:click=move |_| on_select.set(Some(cell_index))
        >
            <div class="lab-block-meta">
                <span class="row-tag">{format!("[TYPE: BRIEF]")}</span>
                <span class="row-date">{format!("[ID: {}]", id.to_uppercase())}</span>
            </div>
            {title.map(|t| view! { <h3 class="lab-block-title">{t}</h3> })}
            {html.with_value(|content| view! {
                <MarkdownContent html=content.clone() class="markdown-content lab-block-body" />
            })}
        </article>
    }
}
