use leptos::*;
use crate::lab::types::{BlockExecution, BlockKind, ExecutionStatus, LabBlock};

#[component]
pub fn BlueprintBlock(
    index: usize,
    blocks: ReadSignal<Vec<LabBlock>>,
    set_blocks: WriteSignal<Vec<LabBlock>>,
    selected: ReadSignal<Option<usize>>,
    on_select: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let meta = Memo::new(move |_| {
        blocks
            .get()
            .get(index)
            .map(|block| match &block.kind {
                BlockKind::Blueprint { id, language, .. } => (id.clone(), language.clone()),
                _ => (String::new(), String::new()),
            })
            .unwrap_or_default()
    });

    let code = Memo::new(move |_| {
        blocks
            .get()
            .get(index)
            .and_then(|block| match &block.kind {
                BlockKind::Blueprint { code, .. } => Some(code.clone()),
                _ => None,
            })
            .unwrap_or_default()
    });

    let execution = Memo::new(move |_| {
        blocks
            .get()
            .get(index)
            .map(|block| block.execution.clone())
            .unwrap_or_default()
    });

    let is_selected = move || selected.get() == Some(index);

    let status_label = move || match execution.get().status {
        ExecutionStatus::Idle => "[STATE: IDLE]",
        ExecutionStatus::Running => "[STATE: RUNNING]",
        ExecutionStatus::Success => "[STATE: SUCCESS]",
        ExecutionStatus::Error => "[STATE: ERROR]",
    };

    view! {
        <article
            class="lab-block lab-block-blueprint"
            class:lab-block-selected=is_selected
            on:click=move |_| on_select.set(Some(index))
        >
            <div class="lab-block-meta">
                <span class="row-tag">"[TYPE: BLUEPRINT]"</span>
                <span class="row-date">{move || format!("[{}]", status_label())}</span>
            </div>
            <h3 class="lab-block-title">
                {move || {
                    let (id, language) = meta.get();
                    format!("{} // {}", id.to_uppercase(), language.to_uppercase())
                }}
            </h3>
            <textarea
                class="lab-code-editor"
                prop:value=move || code.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_blocks.update(|all| {
                        if let Some(block) = all.get_mut(index) {
                            if let BlockKind::Blueprint { code, .. } = &mut block.kind {
                                *code = value;
                            }
                        }
                    });
                }
                on:click=|ev| ev.stop_propagation()
                spellcheck="false"
                rows=12
            />
            <BlueprintOutput execution=execution />
        </article>
    }
}

#[component]
fn BlueprintOutput(execution: Memo<BlockExecution>) -> impl IntoView {
    view! {
        {move || {
            let exec = execution.get();
            if exec.status == ExecutionStatus::Idle
                && exec.stdout.is_empty()
                && exec.error.is_none()
                && exec.result.is_none()
            {
                return view! { <div></div> }.into_view();
            }

            view! {
                <div class="lab-output">
                    <div class="lab-output-meta">
                        <span class="row-tag">"[OUTPUT]"</span>
                        {exec.result.as_ref().map(|value| view! {
                            <span class="row-date">{format!("[RESULT: {}]", value)}</span>
                        })}
                    </div>
                    {exec.error.as_ref().map(|err| view! {
                        <pre class="lab-output-error">{err.clone()}</pre>
                    })}
                    {(!exec.stdout.is_empty()).then(|| view! {
                        <pre class="lab-output-stdout">{exec.stdout.join("\n")}</pre>
                    })}
                </div>
            }.into_view()
        }}
    }
}
