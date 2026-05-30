use leptos::*;
use leptos_router::*;
use crate::situation_monitor::{
    active_source_count, category_label, filter_items, CategoryMeta, FeedSnapshot, ALL_CATEGORY,
    FEED_URL,
};
use crate::utils::{home_href, resolve_asset_url};

#[component]
pub fn SituationMonitorPage() -> impl IntoView {
    let feed = create_resource(
        || FEED_URL,
        |url| async move {
            let resolved = resolve_asset_url(url);
            let response = gloo_net::http::Request::get(&resolved)
                .send()
                .await
                .map_err(|_| "Unable to load situation monitor feed.".to_string())?;

            if !response.ok() {
                return Err(format!("Feed not found ({})", response.status()));
            }

            response
                .json::<FeedSnapshot>()
                .await
                .map_err(|_| "Unable to parse situation monitor feed.".to_string())
        },
    );

    view! {
        <section id="situation-monitor-nav">
            <A href=home_href() class="social-link cta-link">"← BACK TO HOME"</A>
        </section>

        <Suspense fallback=move || view! {
            <section class="report-page situation-monitor-page">
                <p class="sm-loading">"Loading situation monitor..."</p>
            </section>
        }>
            {move || match feed.get() {
                Some(Ok(snapshot)) => view! { <SituationMonitorLoaded snapshot /> }.into_view(),
                Some(Err(message)) => view! {
                    <section class="report-page situation-monitor-page">
                        <p class="doc-error">{message.clone()}</p>
                    </section>
                }.into_view(),
                None => view! {
                    <section class="report-page situation-monitor-page">
                        <p class="sm-loading">"Loading situation monitor..."</p>
                    </section>
                }.into_view(),
            }}
        </Suspense>
    }
}

#[component]
fn SituationMonitorLoaded(snapshot: FeedSnapshot) -> impl IntoView {
    let snapshot = store_value(snapshot);
    let (active_category, set_active_category) = create_signal(ALL_CATEGORY.to_string());
    let (query, set_query) = create_signal(String::new());

    let filtered_items = move || {
        snapshot.with_value(|data| {
            filter_items(&data.items, &active_category.get(), &query.get())
        })
    };

    let filtered_count = move || filtered_items().len();

    view! {
        <section class="report-page situation-monitor-page">
            <header class="report-header">
                <div class="report-header-meta">
                    <span class="home-tag">"Open source"</span>
                    <time class="home-date">{move || snapshot.with_value(|data| data.updated_label.clone())}</time>
                </div>
                <h2 class="report-title">"Situation Monitor"</h2>
                <p class="report-subtitle">
                    "Public-source aggregation for regional awareness. Updated on a schedule from open RSS feeds."
                </p>
            </header>

            <div class="sm-stats">
                <div class="sm-stat">
                    <span class="sm-stat-label">"Items"</span>
                    <span class="sm-stat-value">{move || snapshot.with_value(|data| data.items.len())}</span>
                </div>
                <div class="sm-stat">
                    <span class="sm-stat-label">"Sources live"</span>
                    <span class="sm-stat-value">{move || snapshot.with_value(active_source_count)}</span>
                </div>
                <div class="sm-stat">
                    <span class="sm-stat-label">"Showing"</span>
                    <span class="sm-stat-value">{filtered_count}</span>
                </div>
            </div>

            <div class="sm-controls">
                <label class="sm-search">
                    <span class="sm-search-label">"Search"</span>
                    <input
                        type="search"
                        class="sm-search-input"
                        placeholder="Filter headlines, summaries, or sources..."
                        prop:value=move || query.get()
                        on:input=move |ev| set_query.set(event_target_value(&ev))
                    />
                </label>

                <div class="sm-tabs" role="tablist" aria-label="Category filters">
                    <CategoryTab
                        category=CategoryMeta {
                            id: ALL_CATEGORY.to_string(),
                            label: "All".to_string(),
                            count: snapshot.with_value(|data| data.items.len()),
                        }
                        active=move || active_category.get() == ALL_CATEGORY
                        on_select=move || set_active_category.set(ALL_CATEGORY.to_string())
                    />
                    {move || snapshot.with_value(|data| data.categories.clone()).into_iter().map(|category| {
                        let id = category.id.clone();
                        view! {
                            <CategoryTab
                                category=category
                                active=move || active_category.get() == id
                                on_select={
                                    let id = id.clone();
                                    move || set_active_category.set(id.clone())
                                }
                            />
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="sm-feed">
                {move || {
                    let items = filtered_items();
                    if items.is_empty() {
                        view! {
                            <p class="sm-empty">"No items match the current filter."</p>
                        }.into_view()
                    } else {
                        items.into_iter().map(|item| view! {
                            <article class="sm-item">
                                <div class="sm-item-meta">
                                    <span class="home-tag">{category_label(&item.category)}</span>
                                    <span class="home-date">{item.published_label.clone()}</span>
                                </div>
                                <h3 class="sm-item-title">
                                    <a
                                        href=item.url.clone()
                                        target="_blank"
                                        rel="noopener noreferrer"
                                    >
                                        {item.title.clone()}
                                    </a>
                                </h3>
                                <p class="sm-item-source">{item.source_name.clone()}</p>
                                {(!item.summary.is_empty()).then(|| view! {
                                    <p class="sm-item-summary">{item.summary.clone()}</p>
                                })}
                            </article>
                        }).collect_view()
                    }
                }}
            </div>

            <aside class="sm-methodology">
                <h3 class="sm-methodology-title">"Methodology"</h3>
                <p>
                    "This dashboard aggregates publicly available RSS feeds only. It is a portfolio demonstration "
                    "of open-source monitoring discipline, not an operational intelligence product. "
                    "Headlines link to original publishers."
                </p>
                <details class="sm-sources-panel">
                    <summary class="sm-sources-toggle">"Source list"</summary>
                    <ul class="sm-sources-list">
                        {move || snapshot.with_value(|data| data.sources.clone()).into_iter().map(|source| view! {
                            <li class="sm-sources-item">
                                <a
                                    href=source.url.clone()
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    {source.name.clone()}
                                </a>
                                <span class="sm-sources-meta">
                                    {format!(
                                        "{} · {} items",
                                        category_label(&source.category),
                                        source.item_count
                                    )}
                                </span>
                                {(source.status != "ok").then(|| view! {
                                    <span class="sm-sources-error">"Feed unavailable on last refresh"</span>
                                })}
                            </li>
                        }).collect_view()}
                    </ul>
                </details>
            </aside>
        </section>
    }
}

#[component]
fn CategoryTab<F>(
    category: CategoryMeta,
    active: F,
    on_select: impl Fn() + 'static + Clone,
) -> impl IntoView
where
    F: Fn() -> bool + 'static,
{
    view! {
        <button
            type="button"
            class="sm-tab"
            class:sm-tab-active=move || active()
            role="tab"
            aria-selected=move || active()
            on:click=move |_| on_select.clone()()
        >
            <span>{category.label.clone()}</span>
            <span class="sm-tab-count">{category.count}</span>
        </button>
    }
}
