use std::collections::HashMap;

use gloo_net::http::Request;
use leptos::*;
use web_sys::KeyboardEvent;

use crate::components::markdown_content::MarkdownContent;
use crate::components::technical_document::TechnicalDocument;
use crate::lab::editor;
use crate::lab::kernel::{run_blueprint, run_verify_user_code};
use crate::lab::layout::parse_lab_blocks;
use crate::lab::types::{
    BlockExecution, ExecutionStatus, KernelSession, LabModule, ProbeParam, VerifyResult,
};
use crate::utils::{markdown::rendered_lab_cache_path, resolve_asset_url};

#[component]
pub fn ModuleExercisePanels(
    module_id: &'static str,
    _module_title: &'static str,
    theory_src: &'static str,
) -> impl IntoView {
    let module = create_resource(
        move || module_id,
        |module_id| async move {
            let url = resolve_asset_url(&rendered_lab_cache_path(module_id));
            let response = Request::get(&url).send().await.ok()?;
            if !response.ok() {
                return None;
            }
            response.json::<LabModule>().await.ok()
        },
    );

    view! {
        <Suspense fallback=move || view! {
            <div class="lab-loading">
                <span class="row-tag">"[LAB]"</span>
                <span class="row-date">"Loading exercise..."</span>
            </div>
        }>
            {move || match module.get() {
                None => view! { <div></div> }.into_view(),
                Some(None) => view! {
                    <p class="doc-error">"Unable to load lab module."</p>
                }.into_view(),
                Some(Some(loaded)) => {
                    let parsed = parse_lab_blocks(&loaded.blocks);
                    let probe_values = create_rw_signal(parsed.probe.clone().unwrap_or_default());
                    view! {
                        <div class="curriculum-split">
                            <div class="curriculum-panel curriculum-panel-description">
                                <div class="curriculum-panel-label">"Description"</div>
                                <div class="curriculum-panel-scroll">
                                    <TechnicalDocument src=theory_src />
                                    <LabDescriptionPanel
                                        briefs=parsed.briefs.clone()
                                        verify_cases=parsed.verify_cases.clone()
                                        probe_values=probe_values.read_only()
                                        set_probe_values=probe_values.write_only()
                                    />
                                </div>
                            </div>
                            <div class="curriculum-panel curriculum-panel-code">
                                <div class="curriculum-panel-label">"Code"</div>
                                <LeetCodeCodePanel
                                    starter_code=parsed.starter_code.clone()
                                    verify_cases=parsed.verify_cases.clone()
                                    probe_values=probe_values.read_only()
                                />
                            </div>
                        </div>
                    }.into_view()
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn LabDescriptionPanel(
    briefs: Vec<(Option<String>, String)>,
    verify_cases: Vec<crate::lab::types::VerifyCase>,
    probe_values: ReadSignal<Vec<ProbeParam>>,
    set_probe_values: WriteSignal<Vec<ProbeParam>>,
) -> impl IntoView {
    view! {
        <div class="leetcode-description">
            {briefs.into_iter().map(|(title, html)| view! {
                <section class="leetcode-description-section">
                    {title.map(|t| view! { <h3 class="leetcode-description-heading">{t}</h3> })}
                    <MarkdownContent html=html class="markdown-content" />
                </section>
            }).collect_view()}

            {move || {
                let params = probe_values.get();
                (!params.is_empty()).then(|| view! {
                    <section class="leetcode-examples">
                        <h3 class="leetcode-description-heading">"Try it with"</h3>
                        <div class="leetcode-probe-grid">
                            {params.into_iter().enumerate().map(|(index, param)| {
                                let name = param.name.clone();
                                view! {
                                    <div class="lab-probe-control">
                                        <div class="lab-probe-control-header">
                                            <span class="row-tag">{name.clone()}</span>
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
                                                set_probe_values.update(|all| {
                                                    if let Some(slot) = all.get_mut(index) {
                                                        slot.value = value.clamp(slot.min, slot.max);
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
                            }).collect_view()}
                        </div>
                    </section>
                })
            }}

            {(!verify_cases.is_empty()).then(|| {
                let cases = verify_cases.clone();
                view! {
                    <section class="leetcode-testcases">
                        <h3 class="leetcode-description-heading">"Test cases"</h3>
                        <ul class="leetcode-testcase-list">
                            {cases.into_iter().map(|case| view! {
                                <li><code>{case.expression.clone()}</code></li>
                            }).collect_view()}
                        </ul>
                    </section>
                }
            })}
        </div>
    }
}

#[component]
fn LeetCodeCodePanel(
    starter_code: String,
    verify_cases: Vec<crate::lab::types::VerifyCase>,
    probe_values: ReadSignal<Vec<ProbeParam>>,
) -> impl IntoView {
    let code = create_rw_signal(starter_code);
    let run_output = create_rw_signal(BlockExecution::default());
    let submit_output = create_rw_signal(BlockExecution::default());
    let session = create_rw_signal(KernelSession::local_stub());

    let probes_map = move || {
        probe_values
            .get()
            .into_iter()
            .map(|param| (param.name.clone(), param.value))
            .collect::<HashMap<String, i64>>()
    };

    let on_run = {
        let run_output = run_output;
        let session = session;
        Callback::new(move |_| {
            let outcome = run_blueprint(&code.get(), &session.get());
            session.set(outcome.session);
            run_output.set(outcome.execution);
        })
    };

    let on_submit = {
        let submit_output = submit_output;
        let session = session;
        let verify_cases = verify_cases.clone();
        Callback::new(move |_| {
            let outcome =
                run_verify_user_code(&code.get(), &verify_cases, &probes_map(), &session.get());
            session.set(outcome.session);
            submit_output.set(outcome.execution);
        })
    };

    let textarea_ref = create_node_ref::<html::Textarea>();

    create_effect(move |_| {
        let _ = code.get();
        if let Some(textarea) = textarea_ref.get() {
            editor::init(&textarea);
            editor::refresh(&textarea);
        }
    });

    let handle_keydown = {
        let on_run = on_run.clone();
        let code = code;
        move |event: KeyboardEvent| {
            if event.meta_key() && event.key() == "Enter" {
                event.prevent_default();
                on_run.call(());
                return;
            }

            if let Some(textarea) = textarea_ref.get() {
                if crate::lab::editor_keys::handle_keydown(&event, &textarea) {
                    event.prevent_default();
                    event.stop_propagation();
                    code.set(textarea.value());
                    crate::lab::editor::refresh(&textarea);
                }
            }
        }
    };

    let focus_editor = move |_| {
        if let Some(textarea) = textarea_ref.get() {
            crate::lab::editor_keys::focus_textarea(&textarea);
        }
    };

    view! {
        <div class="leetcode-lab">
            <div class="leetcode-lab-toolbar">
                <button type="button" class="social-link cta-link" on:click=move |_| on_run.call(())>
                    "RUN"
                </button>
                <button type="button" class="social-link cta-link leetcode-submit-btn" on:click=move |_| on_submit.call(())>
                    "SUBMIT"
                </button>
                <span class="leetcode-toolbar-hint">"⌘ + Enter to run · Tab to indent"</span>
            </div>

            <div class="lab-code-editor-shell leetcode-editor-shell" on:click=focus_editor>
                <pre class="lab-code-highlight language-julia" aria-hidden="true">
                    <code class="language-julia"></code>
                </pre>
                <textarea
                    node_ref=textarea_ref
                    class="lab-code-editor language-julia"
                    prop:value=move || code.get()
                    tabindex="0"
                    on:keydown=handle_keydown
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        code.set(value);
                        if let Some(textarea) = textarea_ref.get() {
                            editor::refresh(&textarea);
                        }
                    }
                    spellcheck="false"
                    rows=22
                    autocapitalize="off"
                    autocomplete="off"
                    autocorrect="off"
                />
            </div>

            <RunConsole execution=run_output.read_only() />
            <SubmitResults execution=submit_output.read_only() />
        </div>
    }
}

#[component]
pub fn LeetCodeLabWorkspace(module_id: &'static str, _module_title: &'static str) -> impl IntoView {
    view! {
        <ModuleExercisePanels
            module_id=module_id
            _module_title=""
            theory_src=""
        />
    }
}

#[component]
fn RunConsole(execution: ReadSignal<BlockExecution>) -> impl IntoView {
    view! {
        {move || {
            let exec = execution.get();
            if exec.status == ExecutionStatus::Idle
                && exec.stdout.is_empty()
                && exec.error.is_none()
            {
                return view! { <div></div> }.into_view();
            }

            view! {
                <div class="leetcode-console">
                    <div class="leetcode-console-label">"Console"</div>
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

#[component]
fn SubmitResults(execution: ReadSignal<BlockExecution>) -> impl IntoView {
    view! {
        {move || {
            let exec = execution.get();
            if exec.verify_results.is_empty() && exec.error.is_none() {
                return view! { <div></div> }.into_view();
            }

            view! {
                <div class="leetcode-submit-results">
                    <div class="leetcode-console-label">"Results"</div>
                    {exec.error.as_ref().map(|err| view! {
                        <pre class="lab-output-error">{err.clone()}</pre>
                    })}
                    {(!exec.verify_results.is_empty()).then(|| view! {
                        <div class="lab-verify-results">
                            {exec.verify_results.into_iter().map(|result| view! {
                                <VerifyResultRow result=result />
                            }).collect_view()}
                        </div>
                    })}
                </div>
            }.into_view()
        }}
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
                    {if result.passed { "PASS" } else { "FAIL" }}
                </span>
                <span class="row-date lab-verify-expression">{result.expression.clone()}</span>
            </div>
            {(!result.passed).then(|| view! {
                <div class="lab-verify-row-detail">
                    <span class="row-date">{format!("Expected: {}", result.expected)}</span>
                    <span class="row-date">{format!("Got: {}", result.got)}</span>
                </div>
            })}
        </div>
    }
}
