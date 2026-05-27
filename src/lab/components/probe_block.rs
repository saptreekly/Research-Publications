use leptos::*;
use crate::lab::types::{BlockKind, LabBlock};

#[component]
pub fn ProbeBlock(
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
            .and_then(|block| match &block.kind {
                BlockKind::Probe { id, .. } => Some(id.clone()),
                _ => None,
            })
            .unwrap_or_default()
    });

    let is_selected = move || selected.get() == Some(index);

    view! {
        <article
            class="lab-block lab-block-probe"
            class:lab-block-selected=is_selected
            on:click=move |_| on_select.set(Some(index))
        >
            <div class="lab-block-meta">
                <span class="row-tag">"[TYPE: PROBE]"</span>
                <span class="row-date">{move || format!("[ID: {}]", meta.get().to_uppercase())}</span>
            </div>
            <h3 class="lab-block-title">"PARAMETER CONTROLS"</h3>
            <div class="lab-probe-grid">
                {move || {
                    blocks
                        .get()
                        .get(index)
                        .and_then(|block| match &block.kind {
                            BlockKind::Probe { params, .. } => Some(params.clone()),
                            _ => None,
                        })
                        .map(|params| {
                            params
                                .into_iter()
                                .enumerate()
                                .map(|(param_index, param)| {
                                    view! {
                                        <div class="lab-probe-control" on:click=|ev| ev.stop_propagation()>
                                            <div class="lab-probe-control-header">
                                                <span class="row-tag">{param.name.clone()}</span>
                                                <span class="row-date">{param.value.to_string()}</span>
                                            </div>
                                            <input
                                                class="lab-probe-slider"
                                                type="range"
                                                min=param.min.to_string()
                                                max=param.max.to_string()
                                                prop:value=param.value.to_string()
                                                on:input=move |ev| {
                                                    let value = event_target_value(&ev)
                                                        .parse::<i64>()
                                                        .unwrap_or(param.value);
                                                    set_blocks.update(|all| {
                                                        if let Some(block) = all.get_mut(index) {
                                                            if let BlockKind::Probe { params, .. } = &mut block.kind {
                                                                if let Some(slot) = params.get_mut(param_index) {
                                                                    slot.value = value.clamp(slot.min, slot.max);
                                                                }
                                                            }
                                                            block.execution = Default::default();
                                                        }
                                                        for block in all.iter_mut() {
                                                            if matches!(block.kind, BlockKind::Verify { .. }) {
                                                                block.execution = Default::default();
                                                            }
                                                        }
                                                    });
                                                }
                                            />
                                            <div class="lab-probe-range">
                                                <span class="row-date">{param.min.to_string()}</span>
                                                <span class="row-date">{param.max.to_string()}</span>
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()
                        })
                        .unwrap_or_default()
                }}
            </div>
        </article>
    }
}
