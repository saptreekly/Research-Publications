use leptos::*;
use crate::lab::types::LabBlock;

#[component]
pub fn CommandBar(
    blocks: ReadSignal<Vec<LabBlock>>,
    selected: ReadSignal<Option<usize>>,
    on_run: Callback<()>,
    on_verify: Callback<()>,
    on_clear: Callback<()>,
) -> impl IntoView {
    let can_run = move || {
        selected
            .get()
            .and_then(|index| {
                blocks.with(|all| all.get(index).map(|block| block.is_runnable()))
            })
            .unwrap_or(false)
    };

    let can_verify = move || {
        selected
            .get()
            .and_then(|index| {
                blocks.with(|all| all.get(index).map(|block| block.is_verifiable()))
            })
            .unwrap_or(false)
    };

    view! {
        <div class="lab-command-bar">
            <button
                class="lab-command-btn"
                disabled=move || !can_run()
                on:click=move |_| on_run.call(())
            >
                "RUN"
            </button>
            <button
                class="lab-command-btn"
                disabled=move || !can_verify()
                on:click=move |_| on_verify.call(())
            >
                "VERIFY"
            </button>
            <button
                class="lab-command-btn lab-command-btn-secondary"
                disabled=move || selected.get().is_none()
                on:click=move |_| on_clear.call(())
            >
                "CLEAR OUTPUT"
            </button>
            <div class="lab-command-hint">
                {move || if can_run() {
                    "SHIFT+ENTER TO RUN SELECTED BLUEPRINT"
                } else if can_verify() {
                    "SHIFT+ENTER TO VERIFY SELECTED TESTS"
                } else {
                    "SELECT A BLUEPRINT OR VERIFY BLOCK"
                }}
            </div>
        </div>
    }
}
