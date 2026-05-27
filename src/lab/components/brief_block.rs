use comrak::{markdown_to_html, Options};
use leptos::*;

#[component]
pub fn BriefBlock(
    id: String,
    title: Option<String>,
    body_md: String,
    cell_index: usize,
    selected: ReadSignal<Option<usize>>,
    on_select: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let options = Options::default();
    let html = markdown_to_html(&body_md, &options);

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
            <div class="markdown-content lab-block-body" inner_html=html />
        </article>
    }
}
