use std::collections::HashSet;

use gloo_timers::callback::Interval;
use leptos::*;
use leptos_router::*;

use crate::situation_monitor::map::WorldMap;
use crate::situation_monitor::storage::{load_seen_ids, load_watch_terms, persist_seen_ids, save_watch_terms};
use crate::situation_monitor::{
    active_source_count, breaking_items, build_feed_index, category_class, category_label,
    cluster_stories, filter_items, format_checked_at, format_nzst_clock, format_utc_clock,
    group_by_time_bucket, item_age_label, matches_watch, priority_tier, region_label,
    social_item_count, top_items_for_category, CategoryMeta, FeedItem, FeedSnapshot, SourceMeta,
    StoryCluster, ALL_CATEGORY, ALL_REGION, DIGEST_INITIAL, FEED_LOCAL_URL, FEED_POLL_MS,
    FEED_RAW_URL,
};
use crate::utils::debounce::debounced_string;
use crate::utils::script_loader;
use crate::utils::{home_href, resolve_asset_url};

#[derive(Clone, PartialEq, Eq)]
enum PollStatus {
    Idle,
    Checking,
    Updated(String),
    Unchanged,
    Error(String),
}

async fn fetch_feed_snapshot(cache_bust: u32) -> Result<FeedSnapshot, String> {
    let raw_url = format!("{FEED_RAW_URL}?t={cache_bust}");
    if let Ok(response) = gloo_net::http::Request::get(&raw_url).send().await {
        if response.ok() {
            if let Ok(snapshot) = response.json::<FeedSnapshot>().await {
                return Ok(snapshot);
            }
        }
    }

    let resolved = resolve_asset_url(FEED_LOCAL_URL);
    let response = gloo_net::http::Request::get(&format!("{resolved}?t={cache_bust}"))
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
}

#[component]
pub fn SituationMonitorPage() -> impl IntoView {
    let snapshot = create_rw_signal(None::<FeedSnapshot>);
    let (error, set_error) = create_signal(None::<String>);
    let (refresh_tick, set_refresh_tick) = create_signal(0_u32);
    let (manual_refresh, set_manual_refresh) = create_signal(0_u32);
    let (manual_pending, set_manual_pending) = create_signal(false);
    let (last_check_ms, set_last_check_ms) = create_signal(js_sys::Date::now());
    let (poll_status, set_poll_status) = create_signal(PollStatus::Idle);
    let (seen_ids, set_seen_ids) = create_signal(load_seen_ids());
    let (new_ids, set_new_ids) = create_signal(HashSet::<String>::new());

    create_effect(move |_| {
        spawn_local(async move {
            script_loader::ensure_situation_map().await;
        });
    });

    create_effect(move |_| {
        let _ = refresh_tick.get();
        let _ = manual_refresh.get();
        let initial_load = snapshot.get_untracked().is_none();
        if manual_pending.get_untracked() || initial_load {
            set_poll_status.set(PollStatus::Checking);
        }

        spawn_local(async move {
            let tick = refresh_tick.get_untracked() + manual_refresh.get_untracked();
            match fetch_feed_snapshot(tick).await {
                Ok(data) => {
                    set_last_check_ms.set(js_sys::Date::now());
                    set_manual_pending.set(false);

                    let previous = snapshot.get_untracked();
                    let changed = !previous
                        .as_ref()
                        .is_some_and(|current| current.updated_at == data.updated_at);

                    if changed {
                        let seen = seen_ids.get_untracked();
                        let fresh: HashSet<String> = data
                            .items
                            .iter()
                            .filter(|item| !seen.contains(&item.id))
                            .map(|item| item.id.clone())
                            .collect();
                        set_new_ids.set(fresh);

                        let mut merged = seen;
                        for item in &data.items {
                            merged.insert(item.id.clone());
                        }
                        persist_seen_ids(merged.clone());
                        set_seen_ids.set(merged);

                        snapshot.set(Some(data.clone()));
                        set_poll_status.set(PollStatus::Updated(data.updated_label.clone()));
                    } else {
                        set_poll_status.set(PollStatus::Unchanged);
                    }

                    set_error.set(None);
                }
                Err(message) => {
                    set_last_check_ms.set(js_sys::Date::now());
                    set_manual_pending.set(false);
                    set_poll_status.set(PollStatus::Error(message.clone()));
                    if snapshot.get_untracked().is_none() {
                        set_error.set(Some(message));
                    }
                }
            }
        });
    });

    create_effect(move |_| {
        let interval = Interval::new(FEED_POLL_MS, move || {
            set_refresh_tick.update(|tick| *tick += 1);
        });
        on_cleanup(move || drop(interval));
    });

    view! {
        <section id="situation-monitor-nav">
            <A href=home_href() class="social-link cta-link">"← BACK TO HOME"</A>
        </section>

        <SituationMonitorLoaded
            snapshot=snapshot.read_only()
            error=error
            last_check_ms=last_check_ms
            poll_status=poll_status
            new_ids=new_ids
            on_refresh=move || {
                set_manual_pending.set(true);
                set_manual_refresh.update(|tick| *tick += 1);
            }
        />
    }
}

#[component]
fn SituationMonitorLoaded(
    snapshot: ReadSignal<Option<FeedSnapshot>>,
    error: ReadSignal<Option<String>>,
    last_check_ms: ReadSignal<f64>,
    poll_status: ReadSignal<PollStatus>,
    new_ids: ReadSignal<HashSet<String>>,
    on_refresh: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let (active_category, set_active_category) = create_signal(ALL_CATEGORY.to_string());
    let (active_region, set_active_region) = create_signal(None::<String>);
    let (query, set_query) = create_signal(String::new());
    let debounced_query = debounced_string(query.into(), 250);
    let (watch_input, set_watch_input) = create_signal(load_watch_terms().join(", "));
    let (watch_terms, set_watch_terms) = create_signal(load_watch_terms());
    let (watch_only, set_watch_only) = create_signal(false);
    let (show_digest, set_show_digest) = create_signal(false);
    let (expanded_buckets, set_expanded_buckets) = create_signal(HashSet::<u8>::new());
    let (clock_tick, set_clock_tick) = create_signal(0_u32);

    create_effect(move |_| {
        let interval = Interval::new(1_000, move || {
            set_clock_tick.update(|tick| *tick += 1);
        });
        on_cleanup(move || drop(interval));
    });

    let is_filtered = move || {
        active_category.get() != ALL_CATEGORY
            || active_region.get().is_some()
            || !debounced_query.get().trim().is_empty()
            || watch_only.get()
    };

    let clear_filters = move || {
        set_active_category.set(ALL_CATEGORY.to_string());
        set_active_region.set(None);
        set_query.set(String::new());
        set_watch_only.set(false);
    };

    let feed_index = create_memo(move |_| {
        snapshot
            .get()
            .map(|data| build_feed_index(&data.items))
            .unwrap_or_default()
    });

    let region_counts = create_memo(move |_| feed_index.get().region_counts.clone());
    let region_previews = create_memo(move |_| feed_index.get().region_previews.clone());

    let filtered_items = create_memo(move |_| {
        let index = feed_index.get();
        filter_items(
            &index.items,
            &active_category.get(),
            &debounced_query.get(),
            active_region.get().as_deref(),
            &watch_terms.get(),
            watch_only.get(),
        )
    });

    let breaking = create_memo(move |_| breaking_items(&feed_index.get().items));
    let digest_clusters = create_memo(move |_| cluster_stories(&filtered_items.get()));

    let countdown_label = move || {
        let _ = clock_tick.get();
        let elapsed = js_sys::Date::now() - last_check_ms.get();
        let remaining = ((FEED_POLL_MS as f64 - elapsed) / 1000.0).max(0.0) as u32;
        let minutes = remaining / 60;
        let seconds = remaining % 60;
        format!("Next poll {minutes:02}:{seconds:02}")
    };

    let poll_label = move || {
        let checked = format_checked_at(last_check_ms.get());
        match poll_status.get() {
            PollStatus::Idle => format!("Last checked {checked}"),
            PollStatus::Checking => {
                if snapshot.get().is_some() {
                    checked
                } else {
                    "Loading feed…".to_string()
                }
            }
            PollStatus::Updated(label) => format!("Updated {label} · checked {checked}"),
            PollStatus::Unchanged => format!("Checked {checked} · feed unchanged"),
            PollStatus::Error(message) => format!("Check failed {checked} · {message}"),
        }
    };

    let feed_loaded = move || snapshot.get().is_some();
    let digest_open = move || show_digest.get() || is_filtered();

    let on_region_select = move |region: String| {
        if region == ALL_REGION {
            set_active_region.set(None);
        } else {
            set_active_region.set(Some(region));
        }
    };

    let wall_categories = ["nz-pacific", "apac-security", "cyber", "global"];
    let refresh_handler = on_refresh.clone();

    view! {
        <section class="report-page situation-monitor-page sm-dashboard">
            <header class="report-header sm-header">
                <div class="report-header-meta">
                    <span class="home-tag">"Analyst workstation"</span>
                    <time class="home-date">
                        {move || snapshot.get().map(|data| data.updated_label).unwrap_or_else(|| "Loading feed…".to_string())}
                    </time>
                </div>
                <h2 class="report-title">"Situation Monitor"</h2>
                <p class="report-subtitle">
                    "Regional map, breaking headlines, and category digests. Polls GitHub every five minutes."
                </p>
            </header>

            {move || error.get().map(|message| view! {
                <p class="sm-feed-error" role="status">{message}</p>
            })}

            <div class="sm-ops-bar">
                <span class="sm-ops-pill sm-ops-live">"● LIVE"</span>
                <span class="sm-ops-pill">{countdown_label}</span>
                <span
                    class="sm-ops-pill sm-ops-poll"
                    class:sm-ops-poll-checking=move || poll_status.get() == PollStatus::Checking
                    class:sm-ops-poll-active=move || matches!(poll_status.get(), PollStatus::Updated(_))
                >
                    {poll_label}
                </span>
                <button type="button" class="sm-refresh-btn" on:click=move |_| refresh_handler()>
                    "Refresh now"
                </button>
                <span class="sm-ops-pill">{move || { let _ = clock_tick.get(); format_utc_clock() }}</span>
                <span class="sm-ops-pill">{move || { let _ = clock_tick.get(); format_nzst_clock() }}</span>
                <span class="sm-ops-pill sm-ops-new-slot" class:sm-ops-new-visible=move || !new_ids.get().is_empty()>
                    {move || {
                        let count = new_ids.get().len();
                        if count > 0 {
                            format!("{count} new since last visit")
                        } else {
                            String::new()
                        }
                    }}
                </span>
            </div>

            <section class="sm-map-section">
                <div class="sm-map-section-head">
                    <h3 class="sm-panel-title">"Regional activity"</h3>
                    <span class="sm-map-hint">"Click a region on the map or chip below to filter headlines"</span>
                </div>
                <WorldMap
                    region_counts=region_counts
                    region_previews=region_previews
                    active_region=active_region
                    on_select=on_region_select
                />
            </section>

            <section class="sm-breaking-lane">
                <div class="sm-breaking-head">
                    <h3 class="sm-panel-title">"Breaking & priority"</h3>
                    <span class="sm-breaking-meta">{move || format!("{} items", breaking.get().len())}</span>
                </div>
                <div class="sm-breaking-scroll">
                    {move || {
                        if !feed_loaded() {
                            view! {
                                <p class="sm-breaking-empty sm-skeleton-line">"Loading breaking headlines…"</p>
                            }.into_view()
                        } else {
                            let items = breaking.get();
                            if items.is_empty() {
                                view! { <p class="sm-breaking-empty">"No breaking items in the last hour."</p> }.into_view()
                            } else {
                                items.into_iter().map(|item| view! {
                                <BreakingCard item=item new_ids=new_ids />
                            }).collect_view()
                            }
                        }
                    }}
                </div>
            </section>

            <section class="sm-category-wall" class:sm-category-wall-collapsed=move || is_filtered()>
                {wall_categories.iter().map(|category_id| {
                    let id = *category_id;
                    view! {
                        <CategoryWallPanel
                            category_id=id
                            snapshot
                            new_ids
                            on_select=move || set_active_category.set(id.to_string())
                        />
                    }
                }).collect_view()}
            </section>

            <div class="sm-filter-bar" class:sm-filter-bar-visible=move || is_filtered() aria-live="polite">
                <span class="sm-filter-bar-label">
                    {move || format!("{} stories match filters", digest_clusters.get().len())}
                </span>
                <button type="button" class="sm-filter-bar-clear" on:click=move |_| clear_filters()>
                    "Clear all filters"
                </button>
            </div>

            <div class="sm-trends">
                <span class="sm-trends-label">"Trending"</span>
                {move || {
                    if !feed_loaded() {
                        view! { <span class="sm-trends-empty sm-skeleton-line">"Loading trends…"</span> }.into_view()
                    } else {
                        let trends = snapshot.get().unwrap().trends.clone();
                        if trends.is_empty() {
                            view! { <span class="sm-trends-empty">"Trends appear after the next feed aggregation run"</span> }.into_view()
                        } else {
                            trends.into_iter().take(8).map(|trend| {
                                let term = trend.term.clone();
                                let select_term = term.clone();
                                view! {
                                    <button
                                        type="button"
                                        class="sm-trend-chip"
                                        on:click=move |_| set_query.set(select_term.clone())
                                    >
                                        <span>{term}</span>
                                        <span class="sm-trend-count">{trend.count}</span>
                                    </button>
                                }
                            }).collect_view()
                        }
                    }
                }}
            </div>

            <div class="sm-dashboard-grid">
                <div class="sm-main">
                    <div class="sm-controls">
                        <label class="sm-search">
                            <span class="sm-search-label">"Search"</span>
                            <input
                                type="search"
                                class="sm-search-input"
                                placeholder="Filter headlines, summaries, handles, or sources..."
                                prop:value=move || query.get()
                                on:input=move |ev| set_query.set(event_target_value(&ev))
                            />
                        </label>

                        <label class="sm-search">
                            <span class="sm-search-label">"Watchlist (comma-separated)"</span>
                            <input
                                type="text"
                                class="sm-search-input"
                                placeholder="e.g. ukraine, ransomware, wellington"
                                prop:value=move || watch_input.get()
                                on:input=move |ev| set_watch_input.set(event_target_value(&ev))
                                on:change=move |_| {
                                    let terms: Vec<String> = watch_input
                                        .get()
                                        .split(',')
                                        .map(str::trim)
                                        .filter(|term| !term.is_empty())
                                        .map(str::to_lowercase)
                                        .collect();
                                    save_watch_terms(&terms);
                                    set_watch_terms.set(terms);
                                }
                            />
                        </label>

                        <div class="sm-control-row">
                            <label class="sm-toggle">
                                <input
                                    type="checkbox"
                                    prop:checked=move || watch_only.get()
                                    on:change=move |ev| set_watch_only.set(event_target_checked(&ev))
                                />
                                <span>"Watchlist only"</span>
                            </label>
                            <button
                                type="button"
                                class="sm-region-filter-clear"
                                class:sm-region-filter-clear-visible=move || active_region.get().is_some()
                                on:click=move |_| set_active_region.set(None)
                            >
                                {move || format!("Region: {} ×", region_label(active_region.get().as_deref().unwrap_or("global")))}
                            </button>
                        </div>

                        <div class="sm-tabs" role="tablist" aria-label="Category filters">
                            {move || {
                                if !feed_loaded() {
                                    view! { <span class="sm-skeleton-line">"Loading categories…"</span> }.into_view()
                                } else {
                                    let data = snapshot.get().unwrap();
                                    view! {
                                        <CategoryTab
                                            category=CategoryMeta {
                                                id: ALL_CATEGORY.to_string(),
                                                label: "All".to_string(),
                                                count: data.items.len(),
                                            }
                                            active=move || active_category.get() == ALL_CATEGORY
                                            on_select=move || set_active_category.set(ALL_CATEGORY.to_string())
                                        />
                                        {data.categories.clone().into_iter().filter(|category| category.count > 0).map(|category| {
                                            let id = category.id.clone();
                                            let active_id = id.clone();
                                            view! {
                                                <CategoryTab
                                                    category=category
                                                    active=move || active_category.get() == active_id
                                                    on_select=move || set_active_category.set(id.clone())
                                                />
                                            }
                                        }).collect_view()}
                                    }.into_view()
                                }
                            }}
                        </div>
                    </div>

                    <section class="sm-digest-area">
                        <div class="sm-digest-toggle-wrap" class:sm-digest-toggle-hidden=move || digest_open()>
                            <p class="sm-digest-intro">
                                "Overview mode — use the map, breaking lane, and category panels above. "
                                "Open the digest when you want a scannable list."
                            </p>
                            <button type="button" class="sm-digest-toggle" on:click=move |_| set_show_digest.set(true)>
                                {move || format!("Open headline digest ({} stories)", digest_clusters.get().len())}
                            </button>
                        </div>

                        <div class="sm-digest-panel" class:sm-digest-panel-open=move || digest_open()>
                            <DigestFeed
                                clusters=digest_clusters.get()
                                new_ids=new_ids
                                watch_terms=watch_terms
                                expanded_buckets=expanded_buckets
                                set_expanded_buckets=set_expanded_buckets
                                show_collapse=move || !is_filtered()
                                on_collapse=set_show_digest
                            />
                        </div>
                    </section>
                </div>

                <aside class="sm-sidebar">
                    <section class="sm-sidebar-panel sm-stats-panel">
                        <h3 class="sm-sidebar-title">"Snapshot"</h3>
                        <dl class="sm-stats-dl">
                            <div><dt>"Headlines"</dt><dd>{move || snapshot.get().map(|d| d.items.len()).unwrap_or(0)}</dd></div>
                            <div><dt>"Sources live"</dt><dd>{move || snapshot.get().map(|d| active_source_count(&d)).unwrap_or(0)}</dd></div>
                            <div><dt>"X posts"</dt><dd>{move || snapshot.get().map(|d| social_item_count(&d)).unwrap_or(0)}</dd></div>
                            <div><dt>"Showing"</dt><dd>{move || digest_clusters.get().len()}</dd></div>
                        </dl>
                    </section>

                    <section class="sm-sidebar-panel">
                        <h3 class="sm-sidebar-title">"Source health"</h3>
                        <ul class="sm-sources-list sm-sources-list-compact">
                            {move || {
                                if !feed_loaded() {
                                    view! { <li class="sm-skeleton-line">"Loading source health…"</li> }.into_view()
                                } else {
                                    snapshot.get().unwrap().sources.clone().into_iter().map(|source| view! {
                                        <SourceHealthRow source=source />
                                    }).collect_view()
                                }
                            }}
                        </ul>
                    </section>

                    <section class="sm-sidebar-panel sm-methodology-compact">
                        <h3 class="sm-sidebar-title">"Coverage"</h3>
                        <ul class="sm-coverage-list">
                            <li>"NZ & Pacific newswire and regional reporting"</li>
                            <li>"APAC security analysis, Nikkei Asia, and Lowy Interpreter"</li>
                            <li>"Cyber advisories, The Record, and Krebs on Security"</li>
                            <li>"Curated OSINT watchlist on X (106 accounts)"</li>
                        </ul>
                        <a
                            class="sm-list-link"
                            href="https://x.com/i/lists/1978231089639690329"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "View OSINT list on X →"
                        </a>
                    </section>
                </aside>
            </div>
        </section>
    }
}

#[component]
fn DigestFeed(
    clusters: Vec<StoryCluster>,
    new_ids: ReadSignal<HashSet<String>>,
    watch_terms: ReadSignal<Vec<String>>,
    expanded_buckets: ReadSignal<HashSet<u8>>,
    set_expanded_buckets: WriteSignal<HashSet<u8>>,
    show_collapse: impl Fn() -> bool + 'static,
    on_collapse: WriteSignal<bool>,
) -> impl IntoView {
    let cluster_map: std::collections::HashMap<String, StoryCluster> = clusters
        .into_iter()
        .map(|cluster| (cluster.lead.id.clone(), cluster))
        .collect();

    let grouped = group_by_time_bucket(
        &cluster_map
            .values()
            .map(|cluster| cluster.lead.clone())
            .collect::<Vec<_>>(),
    );

    view! {
        <section class="sm-digest">
            <div class="sm-digest-head">
                <h3 class="sm-panel-title">"Headline digest"</h3>
                {move || show_collapse().then(|| view! {
                    <button
                        type="button"
                        class="sm-digest-collapse"
                        on:click=move |_| on_collapse.set(false)
                    >
                        "Collapse digest"
                    </button>
                })}
            </div>

            {if grouped.is_empty() {
                view! { <p class="sm-empty">"No items match the current filter."</p> }.into_view()
            } else {
                grouped.into_iter().map(|(bucket, items)| {
                    let bucket_order = bucket.order();
                    let hidden_count = items.len().saturating_sub(DIGEST_INITIAL);
                    let expanded = move || expanded_buckets.get().contains(&bucket_order);
                    view! {
                        <section class="sm-digest-group">
                            <h4 class="sm-digest-group-title">{bucket.label()}</h4>
                            <ul class="sm-digest-list">
                                {items.into_iter().enumerate().map(|(index, item)| {
                                    let item_id = item.id.clone();
                                    let cluster = cluster_map.get(&item_id).cloned();
                                    view! {
                                        <DigestRow
                                            item=item
                                            cluster=cluster
                                            new_ids=new_ids
                                            watch_terms=watch_terms
                                            index=index
                                            bucket_order=bucket_order
                                            expanded_buckets=expanded_buckets
                                        />
                                    }
                                }).collect_view()}
                            </ul>
                            {(hidden_count > 0).then(|| view! {
                                <button
                                    type="button"
                                    class="sm-digest-more"
                                    on:click=move |_| {
                                        set_expanded_buckets.update(|set| {
                                            if expanded() {
                                                set.remove(&bucket_order);
                                            } else {
                                                set.insert(bucket_order);
                                            }
                                        });
                                    }
                                >
                                    {move || if expanded() {
                                        "Show fewer".to_string()
                                    } else {
                                        format!("Show {hidden_count} more in {}", bucket.label())
                                    }}
                                </button>
                            })}
                        </section>
                    }
                }).collect_view()
            }}
        </section>
    }
}

#[component]
fn DigestRow(
    item: FeedItem,
    cluster: Option<StoryCluster>,
    new_ids: ReadSignal<HashSet<String>>,
    watch_terms: ReadSignal<Vec<String>>,
    index: usize,
    bucket_order: u8,
    expanded_buckets: ReadSignal<HashSet<u8>>,
) -> impl IntoView {
    let item_id = item.id.clone();
    let is_new = move || new_ids.get().contains(&item_id);
    let watch_target = item.clone();
    let watched = move || matches_watch(&watch_target, &watch_terms.get());
    let age = item_age_label(&item);
    let regions = item.regions.clone();
    let region_hint = regions.first().map(|r| region_label(r)).unwrap_or("Global");
    let size = cluster.map(|c| c.size).unwrap_or(1);
    let title = item.title.clone();
    let url = item.url.clone();
    let source_name = item.source_name.clone();

    view! {
        <li
            class="sm-digest-row"
            class:sm-digest-row-hidden=move || {
                index >= DIGEST_INITIAL && !expanded_buckets.get().contains(&bucket_order)
            }
            class:sm-digest-row-watched=move || watched()
        >
            {move || is_new().then(|| view! { <span class="sm-new-dot" title="New since last visit"></span> })}
            <span class="sm-digest-age">{age}</span>
            <span class="sm-digest-source">{source_name}</span>
            <a class="sm-digest-title" href=url target="_blank" rel="noopener noreferrer">
                {title}
            </a>
            <span class="sm-digest-region">{region_hint}</span>
            {(size > 1).then(|| view! {
                <span class="sm-digest-related">{format!("+{size}")}</span>
            })}
        </li>
    }
}

#[component]
fn BreakingCard(item: FeedItem, new_ids: ReadSignal<HashSet<String>>) -> impl IntoView {
    let id = item.id.clone();
    let is_new = move || new_ids.get().contains(&id);
    let priority_class = priority_tier(item.priority);

    view! {
        <article class=format!("sm-breaking-card {priority_class}")>
            {move || is_new().then(|| view! { <span class="sm-new-badge">"NEW"</span> })}
            <a href=item.url.clone() target="_blank" rel="noopener noreferrer">
                {item.title.clone()}
            </a>
            <span class="sm-breaking-source">{item.source_name.clone()}</span>
        </article>
    }
}

#[component]
fn CategoryWallPanel<F>(
    category_id: &'static str,
    snapshot: ReadSignal<Option<FeedSnapshot>>,
    new_ids: ReadSignal<HashSet<String>>,
    on_select: F,
) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    let cat_class = category_class(category_id);
    let select_handler = on_select.clone();

    view! {
        <article class=format!("sm-wall-panel {cat_class}")>
            <button type="button" class="sm-wall-head" on:click=move |_| select_handler()>
                <span>{category_label(category_id)}</span>
                <span class="sm-wall-open">"View all →"</span>
            </button>
            <ul class="sm-wall-list">
                {move || {
                    let items = snapshot
                        .get()
                        .map(|data| top_items_for_category(&data.items, category_id, 3))
                        .unwrap_or_default();
                    items.into_iter().map(|item| {
                        let item_id = item.id.clone();
                        view! {
                            <li class="sm-wall-item">
                                {move || new_ids.get().contains(&item_id).then(|| view! {
                                    <span class="sm-new-dot" title="New since last visit"></span>
                                })}
                                <a href=item.url.clone() target="_blank" rel="noopener noreferrer">
                                    {item.title.clone()}
                                </a>
                            </li>
                        }
                    }).collect_view()
                }}
            </ul>
        </article>
    }
}

#[component]
fn SourceHealthRow(source: SourceMeta) -> impl IntoView {
    let healthy = source.status == "ok";
    view! {
        <li class="sm-sources-item">
            <div class="sm-source-row">
                <span class=format!("sm-source-dot {}", if healthy { "sm-source-dot-ok" } else { "sm-source-dot-error" })></span>
                <a href=source.url.clone() target="_blank" rel="noopener noreferrer">
                    {source.name.clone()}
                </a>
                <span class="sm-sources-meta">{source.item_count}</span>
            </div>
            <span class="sm-sources-meta">{category_label(&source.category)}</span>
            {(source.status != "ok").then(|| view! {
                <span class="sm-sources-error">"Unavailable on last refresh"</span>
            })}
        </li>
    }
}

#[component]
fn CategoryTab<F>(
    category: CategoryMeta,
    active: F,
    on_select: impl Fn() + 'static + Clone,
) -> impl IntoView
where
    F: Fn() -> bool + 'static + Clone,
{
    let active_class = active.clone();
    let active_selected = active;

    view! {
        <button
            type="button"
            class="sm-tab"
            class:sm-tab-active=move || active_class()
            role="tab"
            aria-selected=move || active_selected()
            on:click=move |_| on_select.clone()()
        >
            <span>{category.label.clone()}</span>
            <span class="sm-tab-count">{category.count}</span>
        </button>
    }
}
