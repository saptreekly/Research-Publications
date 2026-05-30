use std::collections::HashMap;
use gloo_net::http::Request;
use leptos::*;
use web_sys::KeyboardEvent;
use crate::lab::components::block_stream::BlockStream;
use crate::lab::components::command_bar::CommandBar;
use crate::lab::components::inspector::Inspector;
use crate::lab::kernel::{run_blueprint, run_verify};
use crate::lab::types::{
    collect_probe_values, BlockKind, ExecutionStatus, KernelSession, LabModule,
};
use crate::utils::{markdown::rendered_lab_cache_path, resolve_asset_url};

#[component]
pub fn LabWorkspace(
    module_src: &'static str,
    module_id: &'static str,
    module_title: &'static str,
    #[prop(default = false)] embedded: bool,
) -> impl IntoView {
    let _ = module_src;
    let module = create_resource(
        move || (module_id, module_title),
        move |(module_id, module_title)| async move {
            let url = resolve_asset_url(&rendered_lab_cache_path(module_id));
            let response = Request::get(&url).send().await.ok()?;
            if !response.ok() {
                return None;
            }
            let mut module: LabModule = response.json().await.ok()?;
            if module.title.is_empty() {
                module.title = module_title.to_string();
            }
            Some(module)
        },
    );

    view! {
        <Suspense fallback=move || view! {
            <div class="lab-loading">
                <span class="row-tag">"[LAB]"</span>
                <span class="row-date">"Loading module stream..."</span>
            </div>
        }>
            {move || match module.get() {
                None => view! { <div></div> }.into_view(),
                Some(None) => view! {
                    <p class="doc-error">"Unable to load lab module."</p>
                }.into_view(),
                Some(Some(loaded)) => view! {
                    <LabWorkspaceLoaded module=loaded embedded=embedded />
                }.into_view(),
            }}
        </Suspense>
    }
}

#[component]
fn LabWorkspaceLoaded(module: LabModule, #[prop(default = false)] embedded: bool) -> impl IntoView {
    let blocks_signal = create_rw_signal(module.blocks);
    let selected = create_rw_signal(None::<usize>);
    let session = create_rw_signal(KernelSession::local_stub());

    let run_selected = {
        let blocks_signal = blocks_signal;
        let selected = selected;
        let session = session;
        Callback::new(move |_| {
            let Some(index) = selected.get() else { return };
            let code = blocks_signal.with(|all| {
                all.get(index).and_then(|block| match &block.kind {
                    BlockKind::Blueprint { code, .. } => Some(code.clone()),
                    _ => None,
                })
            });
            let Some(code) = code else { return };

            blocks_signal.update(|all| {
                if let Some(block) = all.get_mut(index) {
                    block.execution.status = ExecutionStatus::Running;
                }
            });

            let outcome = run_blueprint(&code, &session.get());

            session.set(outcome.session);
            blocks_signal.update(|all| {
                if let Some(block) = all.get_mut(index) {
                    block.execution = outcome.execution;
                }
            });
        })
    };

    let verify_selected = {
        let blocks_signal = blocks_signal;
        let selected = selected;
        let session = session;
        Callback::new(move |_| {
            let Some(index) = selected.get() else { return };
            let cases_and_probes = blocks_signal.with(|all| {
                let cases = all.get(index).and_then(|block| match &block.kind {
                    BlockKind::Verify { cases, .. } => Some(cases.clone()),
                    _ => None,
                });
                let probe_map: HashMap<String, i64> =
                    collect_probe_values(all).into_iter().collect();
                cases.map(|cases| (cases, probe_map))
            });
            let Some((cases, probes)) = cases_and_probes else { return };

            blocks_signal.update(|all| {
                if let Some(block) = all.get_mut(index) {
                    block.execution.status = ExecutionStatus::Running;
                }
            });

            let outcome = run_verify(&cases, &probes, &session.get());

            session.set(outcome.session);
            blocks_signal.update(|all| {
                if let Some(block) = all.get_mut(index) {
                    block.execution = outcome.execution;
                }
            });
        })
    };

    let clear_selected = {
        let blocks_signal = blocks_signal;
        let selected = selected;
        Callback::new(move |_| {
            let Some(index) = selected.get() else { return };
            blocks_signal.update(|all| {
                if let Some(block) = all.get_mut(index) {
                    block.execution = Default::default();
                }
            });
        })
    };

    let handle_keydown = {
        let blocks_signal = blocks_signal;
        let selected = selected;
        let run_selected = run_selected.clone();
        let verify_selected = verify_selected.clone();
        move |event: KeyboardEvent| {
            if event.shift_key() && event.key() == "Enter" {
                event.prevent_default();
                let Some(index) = selected.get() else { return };
                let is_run = blocks_signal.with(|all| {
                    all.get(index)
                        .map(|block| block.is_runnable())
                        .unwrap_or(false)
                });
                let is_verify = blocks_signal.with(|all| {
                    all.get(index)
                        .map(|block| block.is_verifiable())
                        .unwrap_or(false)
                });
                if is_run {
                    run_selected.call(());
                } else if is_verify {
                    verify_selected.call(());
                }
            }
        }
    };

    view! {
        <section
            class="lab-workspace"
            class:lab-workspace-embedded=embedded
            tabindex="-1"
            on:keydown=handle_keydown
        >
            {(!embedded).then(|| view! {
                <div class="lab-workspace-header">
                    <h2>{format!("LAB // {}", module.title.to_uppercase())}</h2>
                    <p class="section-intro lab-workspace-intro">
                        "Adjust probe parameters, run blueprint code, and verify against reference test cases."
                    </p>
                </div>
            })}

            <div class="lab-workspace-body">
                <div class="lab-workspace-main">
                    <BlockStream
                        blocks=blocks_signal.read_only()
                        set_blocks=blocks_signal.write_only()
                        selected=selected.read_only()
                        set_selected=selected.write_only()
                    />
                    <CommandBar
                        blocks=blocks_signal.read_only()
                        selected=selected.read_only()
                        on_run=run_selected
                        on_verify=verify_selected
                        on_clear=clear_selected
                    />
                </div>
                <Inspector
                    blocks=blocks_signal.read_only()
                    selected=selected.read_only()
                    session=session.read_only()
                />
            </div>
        </section>
    }
}
