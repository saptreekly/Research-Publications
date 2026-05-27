use leptos::*;
use crate::lab::types::{BlockKind, ExecutionStatus, KernelSession, LabBlock};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

#[component]
pub fn Inspector(
    blocks: ReadSignal<Vec<LabBlock>>,
    selected: ReadSignal<Option<usize>>,
    session: ReadSignal<KernelSession>,
) -> impl IntoView {
    view! {
        <aside class="lab-inspector">
            <h2 class="lab-inspector-heading">"INSPECTOR"</h2>

            <div class="lab-inspector-section">
                <div class="cert-label">"KERNEL"</div>
                <div class="cert-title">{move || session.get().kernel_label.clone()}</div>
                {move || session.get().last_run_ms.map(|ms| view! {
                    <div class="row-date lab-inspector-meta">{format!("LAST RUN: {}ms", ms)}</div>
                })}
            </div>

            <div class="lab-inspector-section">
                <div class="cert-label">"PROBE SNAPSHOT"</div>
                {move || {
                    let probes: Vec<_> = blocks
                        .get()
                        .iter()
                        .filter_map(|block| match &block.kind {
                            BlockKind::Probe { params, .. } => Some(params.clone()),
                            _ => None,
                        })
                        .flatten()
                        .collect();

                    if probes.is_empty() {
                        view! {
                            <div class="row-date lab-inspector-meta">"No probe parameters in this module."</div>
                        }.into_view()
                    } else {
                        let a = probes.iter().find(|p| p.name == "a").map(|p| p.value);
                        let n = probes.iter().find(|p| p.name == "n").map(|p| p.value);

                        view! {
                            {probes.into_iter().map(|param| view! {
                                <div class="lab-inspector-var">
                                    <span class="row-tag">{param.name.clone()}</span>
                                    <span class="row-date">{param.value.to_string()}</span>
                                </div>
                            }).collect_view()}
                            {match (a, n) {
                                (Some(a_val), Some(n_val)) => view! {
                                    <div class="row-date lab-inspector-meta">
                                        {format!("GCD({}, {}) = {}", a_val, n_val, gcd(a_val, n_val))}
                                    </div>
                                }.into_view(),
                                _ => view! { <div></div> }.into_view(),
                            }}
                        }.into_view()
                    }
                }}
            </div>

            <div class="lab-inspector-section">
                <div class="cert-label">"ACTIVE BLOCK"</div>
                {move || {
                    if let Some(index) = selected.get() {
                        if let Some(block) = blocks.get().get(index) {
                            return view! {
                                <div class="cert-title">{format!("{} // {}", block.type_label(), block.id().to_uppercase())}</div>
                                <div class="row-date lab-inspector-meta">
                                    {match block.execution.status {
                                        ExecutionStatus::Idle => "EXECUTION: IDLE",
                                        ExecutionStatus::Running => "EXECUTION: RUNNING",
                                        ExecutionStatus::Success => "EXECUTION: SUCCESS",
                                        ExecutionStatus::Error => "EXECUTION: ERROR",
                                    }}
                                </div>
                                {match &block.kind {
                                    BlockKind::Blueprint { language, code, .. } => view! {
                                        <div class="row-date lab-inspector-meta">
                                            {format!("LANG: {} // {} LINES", language.to_uppercase(), code.lines().count())}
                                        </div>
                                    }.into_view(),
                                    BlockKind::Probe { params, .. } => view! {
                                        <div class="row-date lab-inspector-meta">
                                            {format!("PARAMS: {}", params.len())}
                                        </div>
                                    }.into_view(),
                                    BlockKind::Verify { cases, .. } => view! {
                                        <div class="row-date lab-inspector-meta">
                                            {format!("TESTS: {} // PASSED: {}", cases.len(), block.execution.verify_results.iter().filter(|r| r.passed).count())}
                                        </div>
                                    }.into_view(),
                                    _ => view! { <div></div> }.into_view(),
                                }}
                            }.into_view();
                        }
                    }

                    view! {
                        <div class="cert-title">"NONE SELECTED"</div>
                        <div class="row-date lab-inspector-meta">"Select a block in the module stream."</div>
                    }.into_view()
                }}
            </div>

            <div class="lab-inspector-section">
                <div class="cert-label">"SESSION VARIABLES"</div>
                {move || {
                    let vars = session.get().variables;
                    if vars.is_empty() {
                        view! {
                            <div class="row-date lab-inspector-meta">"No variables yet. Run or verify a block."</div>
                        }.into_view()
                    } else {
                        vars.into_iter().map(|(name, value)| view! {
                            <div class="lab-inspector-var">
                                <span class="row-tag">{name}</span>
                                <span class="row-date">{value}</span>
                            </div>
                        }).collect_view()
                    }
                }}
            </div>
        </aside>
    }
}
