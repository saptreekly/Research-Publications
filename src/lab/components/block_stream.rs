use leptos::*;
use crate::lab::components::blueprint_block::BlueprintBlock;
use crate::lab::components::brief_block::BriefBlock;
use crate::lab::components::probe_block::ProbeBlock;
use crate::lab::components::verify_block::VerifyBlock;
use crate::lab::types::{BlockKind, LabBlock};

#[component]
pub fn BlockStream(
    blocks: ReadSignal<Vec<LabBlock>>,
    set_blocks: WriteSignal<Vec<LabBlock>>,
    selected: ReadSignal<Option<usize>>,
    set_selected: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let block_count = Memo::new(move |_| blocks.get().len());

    view! {
        <div class="lab-stream">
            {move || {
                let count = block_count.get();
                (0..count).map(|index| {
                    let block = blocks.get().get(index).cloned();
                    match block {
                        Some(LabBlock { kind: BlockKind::Brief { id, title, brief_html, .. }, .. }) => {
                            view! {
                                <BriefBlock
                                    id=id
                                    title=title
                                    brief_html=brief_html
                                    cell_index=index
                                    selected=selected
                                    on_select=set_selected
                                />
                            }.into_view()
                        }
                        Some(LabBlock { kind: BlockKind::Probe { .. }, .. }) => {
                            view! {
                                <ProbeBlock
                                    index=index
                                    blocks=blocks
                                    set_blocks=set_blocks
                                    selected=selected
                                    on_select=set_selected
                                />
                            }.into_view()
                        }
                        Some(LabBlock { kind: BlockKind::Blueprint { .. }, .. }) => {
                            view! {
                                <BlueprintBlock
                                    index=index
                                    blocks=blocks
                                    set_blocks=set_blocks
                                    selected=selected
                                    on_select=set_selected
                                />
                            }.into_view()
                        }
                        Some(LabBlock { kind: BlockKind::Starter { .. }, .. }) => {
                            view! { <div></div> }.into_view()
                        }
                        Some(LabBlock { kind: BlockKind::Verify { .. }, .. }) => {
                            view! {
                                <VerifyBlock
                                    index=index
                                    blocks=blocks
                                    selected=selected
                                    on_select=set_selected
                                />
                            }.into_view()
                        }
                        None => view! { <div></div> }.into_view(),
                    }
                }).collect_view()
            }}
        </div>
    }
}
