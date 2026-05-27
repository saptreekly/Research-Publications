use leptos::*;
use crate::lab::types::{BlockKind, ExecutionStatus, LabBlock, VerifyResult};

#[component]
pub fn VerifyBlock(
    index: usize,
    blocks: ReadSignal<Vec<LabBlock>>,
    selected: ReadSignal<Option<usize>>,
    on_select: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let meta = Memo::new(move |_| {
        blocks
            .get()
            .get(index)
            .map(|block| match &block.kind {
                BlockKind::Verify { id, cases, .. } => (id.clone(), cases.len()),
                _ => (String::new(), 0),
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
        ExecutionStatus::Success => "[STATE: PASS]",
        ExecutionStatus::Error => "[STATE: FAIL]",
    };

    view! {
        <article
            class="lab-block lab-block-verify"
            class:lab-block-selected=is_selected
            on:click=move |_| on_select.set(Some(index))
        >
            <div class="lab-block-meta">
                <span class="row-tag">"[TYPE: VERIFY]"</span>
                <span class="row-date">{move || format!("[{}]", status_label())}</span>
            </div>
            <h3 class="lab-block-title">
                {move || {
                    let (id, count) = meta.get();
                    format!("{} // {} TESTS", id.to_uppercase(), count)
                }}
            </h3>

            {move || {
                let exec = execution.get();
                if exec.verify_results.is_empty() {
                    view! {
                        <div class="row-date lab-verify-hint">
                            "Select this block and press VERIFY to run test cases."
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="lab-verify-results">
                            {exec.verify_results.into_iter().map(|result| view! {
                                <VerifyResultRow result=result />
                            }).collect_view()}
                        </div>
                    }.into_view()
                }
            }}

            {move || {
                let exec = execution.get();
                (!exec.stdout.is_empty()).then(|| view! {
                    <div class="lab-output">
                        <pre class="lab-output-stdout">{exec.stdout.join("\n")}</pre>
                    </div>
                })
            }}
        </article>
    }
}

#[component]
fn VerifyResultRow(result: VerifyResult) -> impl IntoView {
    let status_class = if result.passed {
        "lab-verify-pass"
    } else {
        "lab-verify-fail"
    };

    view! {
        <div class=format!("lab-verify-row {}", status_class)>
            <div class="lab-verify-row-header">
                <span class="row-tag">
                    {if result.passed { "[PASS]" } else { "[FAIL]" }}
                </span>
                <span class="row-date lab-verify-expression">{result.expression.clone()}</span>
            </div>
            <div class="lab-verify-row-detail">
                <span class="row-date">{format!("EXPECTED: {}", result.expected)}</span>
                <span class="row-date">{format!("GOT: {}", result.got)}</span>
            </div>
        </div>
    }
}
