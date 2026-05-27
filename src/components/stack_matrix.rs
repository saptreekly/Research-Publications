use leptos::*;
use crate::utils::resolve_asset_url;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
struct StackItem {
    language: String,
    bytes: u64,
}

#[derive(Clone, Deserialize, Serialize)]
struct StackData {
    updated_at: String,
    languages: Vec<StackItem>,
}

#[component]
pub fn StackMatrix() -> impl IntoView {
    let stack = create_resource(
        || (),
        |_| async move {
            let url = resolve_asset_url("static/stack.json");
            if let Ok(response) = gloo_net::http::Request::get(&url).send().await {
                if let Ok(data) = response.json::<StackData>().await {
                    return data;
                }
            }
            StackData {
                updated_at: "N/A".to_string(),
                languages: vec![],
            }
        },
    );

    view! {
        <div class="stack-matrix">
            <Suspense fallback=move || view! { <div class="stack-label">"Loading language distribution..."</div> }>
                {move || stack.get().map(|data| view! {
                    {data.languages.into_iter().map(|item| {
                        let max_bytes = 100000.0;
                        let percentage = ((item.bytes as f64 / max_bytes) * 100.0).min(100.0);
                        view! {
                            <div class="stack-row">
                                <div class="stack-label">{item.language}</div>
                                <div class="bar-container">
                                    <div class="bar" style=format!("width: {}%;", percentage)></div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                    <div class="row-date stack-audit">
                        "LAST AUDIT: " {data.updated_at}
                    </div>
                })}
            </Suspense>
        </div>
    }
}
