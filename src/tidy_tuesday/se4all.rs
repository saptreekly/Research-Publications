use leptos::*;
use crate::components::lazy_section::LazySection;
use crate::tidy_tuesday::analysis::{
    access_renewables_scatter, adoption_wave_crossover,
    adoption_wave_data, adoption_wave_insights, adoption_wave_stat_cards, compute_era_insights,
    compute_stat_cards, country_names, heatmap_values, histogram_bins, median_trajectory_data,
    median_trajectory_insights, renewable_growth_data, renewable_growth_insights,
    renewable_growth_leaders, renewable_growth_stat_cards, renewable_mix_series,
    solar_rank_race_series, top_countries_by_metric, top_country_deltas, YearDistribution,
    RENEWABLE_MIX,
};
use crate::tidy_tuesday::charts::{
    AdoptionWaveChart, AdoptionWavePoint, BumpChart, BumpPointData, BumpSeriesData, DumbbellChart,
    DumbbellRow, GrowthBarRow, GrowthLeaderGroup, HeatmapChart, HistogramChart, HorizontalBarChart,
    InsightCardData, InsightCards, LineChart, LineSeries, MedianBandPoint, MedianTrendsChart,
    RenewableGrowthChart, ScatterChart, ScatterPointData, StackLayerSeries,     StackedAreaChart, StatCardData, StatCards, value_format_for_metric,
};
use crate::tidy_tuesday::controls::{year_range_presets, YearRangePicker};
use crate::tidy_tuesday::{country_series, solar_ranking, year_bounds, ExploreData};
use crate::utils::debounce::{debounced_i32, debounced_usize};
use crate::utils::resolve_asset_url;

const LINE_COLORS: [&str; 6] = [
    "#009e73", "#56b4e9", "#d55e00", "#a855f7", "#0072b2", "#cc79a7",
];

#[component]
pub fn Se4AllExplorer(data_url: &'static str) -> impl IntoView {
    let data = create_resource(
        move || data_url,
        |url| async move {
            let resolved = resolve_asset_url(url);
            let response = gloo_net::http::Request::get(&resolved)
                .send()
                .await
                .map_err(|_| "Unable to load chart data.".to_string())?;

            if !response.ok() {
                return Err(format!("Chart data not found ({})", response.status()));
            }

            response
                .json::<ExploreData>()
                .await
                .map_err(|_| "Unable to parse chart data.".to_string())
        },
    );

    view! {
        <Suspense fallback=move || view! { <p class="tt-explorer-loading">"Loading interactive charts..."</p> }>
            {move || match data.get() {
                Some(Ok(dataset)) => view! { <Se4AllExplorerLoaded dataset /> }.into_view(),
                Some(Err(message)) => view! {
                    <p class="doc-error">{message.clone()}</p>
                }.into_view(),
                None => view! { <p class="tt-explorer-loading">"Loading interactive charts..."</p> }.into_view(),
            }}
        </Suspense>
    }
}

#[component]
fn Se4AllExplorerLoaded(dataset: ExploreData) -> impl IntoView {
    let dataset = store_value(dataset);
    let country_options = store_value(dataset.with_value(|data| country_names(&data.records)));
    let (min_year, max_year) = dataset.with_value(|data| year_bounds(&data.records));
    let year_presets = year_range_presets(min_year, max_year);
    let stat_cards = dataset.with_value(|data| {
        compute_stat_cards(&data.records, min_year, max_year)
            .into_iter()
            .map(|card| StatCardData {
                label: card.label,
                value: card.value,
                detail: card.detail,
            })
            .collect::<Vec<_>>()
    });
    let era_insights = dataset.with_value(|data| {
        compute_era_insights(&data.records, min_year, max_year)
            .into_iter()
            .map(|insight| InsightCardData {
                title: insight.title,
                highlight: insight.highlight,
                body: insight.body,
                accent: insight.accent,
            })
            .collect::<Vec<_>>()
    });

    let growth_start = create_rw_signal(2000_i32.max(min_year));
    let growth_end = create_rw_signal(max_year);
    let growth_start_chart = debounced_i32(growth_start, 120);
    let growth_end_chart = debounced_i32(growth_end, 120);
    let mix_country = create_rw_signal("Germany".to_string());
    let scatter_year = create_rw_signal(max_year);
    let scatter_year_chart = debounced_i32(scatter_year, 120);
    let trend_metric = create_rw_signal("wind_tfec".to_string());
    let trend_start = create_rw_signal(min_year);
    let trend_end = create_rw_signal(max_year);
    let trend_start_chart = debounced_i32(trend_start, 120);
    let trend_end_chart = debounced_i32(trend_end, 120);
    let selected_countries = create_rw_signal(vec![
        "Denmark".to_string(),
        "Ireland".to_string(),
        "Norway".to_string(),
    ]);
    let heatmap_metric = create_rw_signal("solar_tfec".to_string());
    let hist_metric = create_rw_signal("wind_tfec".to_string());
    let hist_year = create_rw_signal(max_year);
    let hist_year_chart = debounced_i32(hist_year, 120);
    let rank_year = create_rw_signal(max_year);
    let rank_year_chart = debounced_i32(rank_year, 120);
    let rank_count = create_rw_signal(15_usize);
    let rank_count_chart = debounced_usize(rank_count, 120);
    let rank_ascending = create_rw_signal(true);
    let delta_metric = create_rw_signal("elec_pct".to_string());

    let growth_presets = year_presets.clone();
    let trend_presets = year_presets.clone();

    view! {
        <div class="tt-explorer">
            <p class="tt-explorer-lead">
                {format!(
                    "SE4ALL country-level energy data ({min_year}–{max_year}). The stats and charts below are computed live from {count} country-year records. Drag sliders, switch metrics, and hover for detail.",
                    count = dataset.with_value(|data| data.records.len())
                )}
            </p>

            <section class="tt-panel tt-panel-stats">
                <div class="tt-panel-header">
                    <h3>"At a glance"</h3>
                    <p>"Headline statistics across countries: medians, adoption counts, and outliers from the full dataset."</p>
                </div>
                <StatCards cards=stat_cards />
            </section>

            <section class="tt-panel tt-panel-insights">
                <div class="tt-panel-header">
                    <h3>"What this era reveals"</h3>
                    <p>"Data-driven takeaways from 1990–2010, the baseline decade before SE4ALL's 2030 targets."</p>
                </div>
                <InsightCards insights=era_insights />
            </section>

            <section class="tt-panel">
                <div class="tt-panel-header">
                    <h3>"Global median trajectories"</h3>
                    <p>"How the typical country changed on access, renewables, and emerging technologies."</p>
                </div>
                <LazySection min_height=360>
                    {move || dataset.with_value(|data| median_trends_chart(data, min_year, max_year))}
                </LazySection>
            </section>

            <section class="tt-panel">
                <div class="tt-panel-header">
                    <h3>"Technology adoption wave"</h3>
                    <p>"How solar and wind TFEC reporting spread across countries, and where the technologies overlapped."</p>
                </div>
                <LazySection min_height=360>
                    {move || dataset.with_value(|data| {
                        adoption_wave_chart(data, min_year, max_year)
                    })}
                </LazySection>
            </section>

            <section class="tt-panel">
                <div class="tt-panel-header">
                    <h3>"Biggest gainers"</h3>
                    <p>{format!("Dumbbell chart: start value (blue) → end value (purple) for {min_year}–{max_year}. Longer lines = more change.")}</p>
                </div>
                <LazySection min_height=320>
                    <label class="tt-control tt-control-metric">
                        <span class="tt-control-label">"Metric"</span>
                        <select
                            prop:value=move || delta_metric.get()
                            on:change=move |ev| delta_metric.set(event_target_value(&ev))
                        >
                            <option value="elec_pct">"Electricity access (%)"</option>
                            <option value="renew_tfec">"Renewable TFEC share (%)"</option>
                            <option value="solar_tfec">"Solar TFEC share (%)"</option>
                            <option value="wind_tfec">"Wind TFEC share (%)"</option>
                            <option value="solar_tj">"Solar consumption (TJ)"</option>
                        </select>
                    </label>
                    {move || dataset.with_value(|data| delta_chart(
                        data,
                        &delta_metric.get(),
                        min_year,
                        max_year,
                    ))}
                </LazySection>
            </section>

            <section class="tt-panel">
                            <div class="tt-panel-header">
                                <h3>"Solar rank race"</h3>
                                <p>"Tracks the 2010 top 5 plus former #1 Japan. Lower on the chart is better. Shaded band = top 5. Hover points for terajoule values."</p>
                            </div>
                <LazySection min_height=320>
                    {move || dataset.with_value(|data| bump_chart(data))}
                </LazySection>
            </section>

            <section class="tt-panel tt-panel-growth">
                <div class="tt-panel-header">
                    <h3>"Which renewables grew fastest?"</h3>
                    <p>"Compare technologies by mean TFEC share growth and see which countries drove the steepest trajectories."</p>
                </div>
                <LazySection min_height=420>
                    <YearRangePicker
                        min_year=min_year
                        max_year=max_year
                        start=growth_start
                        end=growth_end
                        presets=growth_presets.clone()
                    />
                    {move || dataset.with_value(|data| {
                        growth_chart(data, growth_start_chart.get(), growth_end_chart.get())
                    })}
                </LazySection>
            </section>

            <section class="tt-panel">
                <div class="tt-panel-header">
                    <h3>"Renewable mix over time"</h3>
                    <p>"Stacked area chart of how each technology contributes to a country's renewable TFEC share. Hydro often dominates; watch solar and wind layers grow."</p>
                </div>
                <LazySection min_height=320>
                    <label class="tt-control tt-control-add-country">
                        <span class="tt-control-label">"Country"</span>
                        <select
                            prop:value=move || mix_country.get()
                            on:change=move |ev| mix_country.set(event_target_value(&ev))
                        >
                            {country_options.with_value(|options| options.clone()).into_iter().map(|country| view! {
                                <option value=country.clone()>{country}</option>
                            }).collect_view()}
                        </select>
                    </label>
                    {move || dataset.with_value(|data| mix_chart(
                        data,
                        &mix_country.get(),
                        min_year,
                        max_year,
                    ))}
                </LazySection>
            </section>

            <section class="tt-panel">
                <div class="tt-panel-header">
                    <h3>"Access vs. renewable share"</h3>
                    <p>"Each dot is a country. Dashed lines mark the median on each axis. Upper-right countries combine high grid access with high renewable TFEC share."</p>
                </div>
                <LazySection min_height=320>
                    <label class="tt-control">
                        <span class="tt-control-label">"Year"</span>
                        <input
                            type="range"
                            min=min_year.to_string()
                            max=max_year.to_string()
                            prop:value=move || scatter_year.get().to_string()
                            on:input=move |ev| {
                                scatter_year.set(
                                    event_target_value(&ev)
                                        .parse()
                                        .unwrap_or(max_year)
                                        .clamp(min_year, max_year),
                                );
                            }
                        />
                        <span class="tt-control-value">{move || scatter_year.get()}</span>
                    </label>
                    {move || dataset.with_value(|data| scatter_chart(data, scatter_year_chart.get()))}
                </LazySection>
            </section>

            <section class="tt-panel">
                <div class="tt-panel-header">
                    <h3>"Country energy trends"</h3>
                    <p>"Track how each country's share of total final energy consumption changed over time. Hover points for exact values."</p>
                </div>
                <LazySection min_height=420>
                    <div class="tt-controls tt-controls-wrap">
                        <label class="tt-control tt-control-metric">
                            <span class="tt-control-label">"Metric"</span>
                            <select
                                prop:value=move || trend_metric.get()
                                on:change=move |ev| trend_metric.set(event_target_value(&ev))
                            >
                                {dataset.with_value(|data| data.metrics.iter().filter(|metric| metric.key.ends_with("_tfec")).map(|metric| view! {
                                    <option value=metric.key.clone()>{metric.label.clone()}</option>
                                }).collect_view())}
                            </select>
                        </label>
                        <YearRangePicker
                            min_year=min_year
                            max_year=max_year
                            start=trend_start
                            end=trend_end
                            presets=trend_presets.clone()
                        />
                    </div>
                    <div class="tt-chip-row">
                        <button
                            type="button"
                            class="tt-chip tt-chip-action"
                            on:click=move |_| selected_countries.set(vec![
                                "Denmark".into(), "Ireland".into(), "Norway".into(),
                            ])
                        >
                            "Nordic wind leaders"
                        </button>
                        <button
                            type="button"
                            class="tt-chip tt-chip-action"
                            on:click=move |_| selected_countries.set(vec![
                                "China".into(), "United States of America".into(), "Germany".into(),
                            ])
                        >
                            "Major economies"
                        </button>
                        {move || selected_countries.get().into_iter().map(|country| {
                            let country_for_click = country.clone();
                            view! {
                                <button
                                    type="button"
                                    class="tt-chip tt-chip-active"
                                    on:click=move |_| {
                                        selected_countries.update(|list| {
                                            list.retain(|entry| entry != &country_for_click);
                                        });
                                    }
                                >
                                    {country.clone()}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                    <label class="tt-control tt-control-add-country">
                        <span class="tt-control-label">"Add country"</span>
                        <select
                            on:change=move |ev| {
                                let country = event_target_value(&ev);
                                if country.is_empty() {
                                    return;
                                }
                                selected_countries.update(|list| {
                                    if !list.contains(&country) && list.len() < 6 {
                                        list.push(country);
                                    }
                                });
                            }
                        >
                            <option value="">"Choose a country…"</option>
                            {country_options.with_value(|options| options.clone()).into_iter().map(|country| view! {
                                <option value=country.clone()>{country}</option>
                            }).collect_view()}
                        </select>
                    </label>
                    {move || dataset.with_value(|data| trend_chart(
                        data,
                        &trend_metric.get(),
                        trend_start_chart.get(),
                        trend_end_chart.get(),
                        &selected_countries.get(),
                    ))}
                </LazySection>
            </section>

            <section class="tt-panel">
                <div class="tt-panel-header">
                    <h3>"Adoption heatmap"</h3>
                    <p>{format!("Top 16 countries by the selected metric in {max_year}. Brighter cells = higher TFEC share that year.")}</p>
                </div>
                <LazySection min_height=320>
                    <label class="tt-control tt-control-metric">
                        <span class="tt-control-label">"Metric"</span>
                        <select
                            prop:value=move || heatmap_metric.get()
                            on:change=move |ev| heatmap_metric.set(event_target_value(&ev))
                        >
                            {dataset.with_value(|data| data.metrics.iter().filter(|metric| metric.key.ends_with("_tfec")).map(|metric| view! {
                                <option value=metric.key.clone()>{metric.label.clone()}</option>
                            }).collect_view())}
                        </select>
                    </label>
                    {move || dataset.with_value(|data| heatmap_chart(
                        data,
                        &heatmap_metric.get(),
                        min_year,
                        max_year,
                    ))}
                </LazySection>
            </section>

            <section class="tt-panel">
                <div class="tt-panel-header">
                    <h3>"How widely adopted is it?"</h3>
                    <p>"Distribution of non-zero TFEC shares across all countries in a given year. Most countries cluster near zero for newer technologies."</p>
                </div>
                <LazySection min_height=320>
                    <div class="tt-controls tt-controls-wrap">
                        <label class="tt-control tt-control-metric">
                            <span class="tt-control-label">"Metric"</span>
                            <select
                                prop:value=move || hist_metric.get()
                                on:change=move |ev| hist_metric.set(event_target_value(&ev))
                            >
                                {dataset.with_value(|data| data.metrics.iter().filter(|metric| metric.key.ends_with("_tfec")).map(|metric| view! {
                                    <option value=metric.key.clone()>{metric.label.clone()}</option>
                                }).collect_view())}
                            </select>
                        </label>
                        <label class="tt-control">
                            <span class="tt-control-label">"Year"</span>
                            <input
                                type="range"
                                min=min_year.to_string()
                                max=max_year.to_string()
                                prop:value=move || hist_year.get().to_string()
                                on:input=move |ev| {
                                    hist_year.set(
                                        event_target_value(&ev)
                                            .parse()
                                            .unwrap_or(max_year)
                                            .clamp(min_year, max_year),
                                    );
                                }
                            />
                            <span class="tt-control-value">{move || hist_year.get()}</span>
                        </label>
                    </div>
                    {move || dataset.with_value(|data| histogram_chart(
                        data,
                        &hist_metric.get(),
                        hist_year_chart.get(),
                    ))}
                </LazySection>
            </section>

            <section class="tt-panel">
                <div class="tt-panel-header">
                    <h3>"Solar consumption ranking"</h3>
                    <p>"Rank countries by absolute solar terajoules, a different lens from TFEC share that captures total scale."</p>
                </div>
                <LazySection min_height=320>
                    <div class="tt-controls">
                        <label class="tt-control">
                            <span class="tt-control-label">"Year"</span>
                            <input
                                type="range"
                                min=min_year.to_string()
                                max=max_year.to_string()
                                prop:value=move || rank_year.get().to_string()
                                on:input=move |ev| {
                                    rank_year.set(
                                        event_target_value(&ev)
                                            .parse()
                                            .unwrap_or(max_year)
                                            .clamp(min_year, max_year),
                                    );
                                }
                            />
                            <span class="tt-control-value">{move || rank_year.get()}</span>
                        </label>
                        <label class="tt-control">
                            <span class="tt-control-label">"Countries shown"</span>
                            <input
                                type="range"
                                min="5"
                                max="25"
                                prop:value=move || rank_count.get().to_string()
                                on:input=move |ev| {
                                    rank_count.set(event_target_value(&ev).parse().unwrap_or(15));
                                }
                            />
                            <span class="tt-control-value">{move || rank_count.get()}</span>
                        </label>
                        <label class="tt-control tt-control-toggle">
                            <span class="tt-control-label">"Order"</span>
                            <button
                                type="button"
                                class="tt-chip tt-chip-action"
                                on:click=move |_| rank_ascending.update(|value| *value = !*value)
                            >
                                {move || if rank_ascending.get() { "Lowest first" } else { "Highest first" }}
                            </button>
                        </label>
                    </div>
                    {move || dataset.with_value(|data| rank_chart(
                        data,
                        min_year,
                        max_year,
                        rank_year_chart.get(),
                        rank_count_chart.get(),
                        rank_ascending.get(),
                    ))}
                </LazySection>
            </section>
        </div>
    }
}

fn growth_chart(dataset: &ExploreData, year_start: i32, year_end: i32) -> impl IntoView {
    let growth_rows = renewable_growth_data(&dataset.records, year_start, year_end);
    let stat_cards = renewable_growth_stat_cards(&growth_rows, year_start, year_end)
        .into_iter()
        .map(|card| StatCardData {
            label: card.label,
            value: card.value,
            detail: card.detail,
        })
        .collect();
    let insights = renewable_growth_insights(&growth_rows, year_start, year_end);
    let leader_groups = renewable_growth_leaders(
        &dataset.records,
        &growth_rows,
        year_start,
        year_end,
        2,
        4,
    )
    .into_iter()
    .map(|group| GrowthLeaderGroup {
        technology: group.technology,
        color: group.color,
        leaders: group.leaders,
    })
    .collect();
    let chart_rows = growth_rows
        .into_iter()
        .map(|row| GrowthBarRow {
            label: row.label.to_string(),
            color: row.color,
            mean: row.mean_slope,
            median: row.median_slope,
            countries: row.country_count,
        })
        .collect();

    view! {
        <div class="tt-growth-panel-content">
            <StatCards cards=stat_cards />
            <RenewableGrowthChart
                rows=chart_rows
                leaders=leader_groups
                insights=insights
                year_start=year_start
                year_end=year_end
            />
        </div>
    }
}

fn mix_chart(
    dataset: &ExploreData,
    country: &str,
    year_start: i32,
    year_end: i32,
) -> impl IntoView {
    let mix = renewable_mix_series(&dataset.records, country, year_start, year_end);
    let has_data = mix.iter().any(|(_, values)| values.iter().any(|value| *value > 0.0));

    if !has_data {
        return view! {
            <p class="tt-chart-empty">{format!("No renewable mix data for {country}.")}</p>
        }.into_view();
    }

    let layers: Vec<StackLayerSeries> = RENEWABLE_MIX
        .iter()
        .map(|layer| StackLayerSeries {
            label: layer.label,
            color: layer.color,
            points: mix
                .iter()
                .map(|(year, values)| {
                    let index = RENEWABLE_MIX
                        .iter()
                        .position(|entry| entry.key == layer.key)
                        .unwrap_or(0);
                    (*year, values[index])
                })
                .collect(),
        })
        .collect();

    view! {
        <StackedAreaChart
            title=format!("Renewable technology mix: {country}")
            subtitle=Some(format!("Stacked TFEC shares · {year_start}–{year_end}"))
            y_label="Share of TFEC (%)"
            layers=layers
            year_start=year_start
            year_end=year_end
        />
    }
    .into_view()
}

fn scatter_chart(dataset: &ExploreData, year: i32) -> impl IntoView {
    let points: Vec<ScatterPointData> = access_renewables_scatter(&dataset.records, year)
        .into_iter()
        .map(|point| ScatterPointData {
            label: point.label,
            x: point.x,
            y: point.y,
        })
        .collect();

    view! {
        <ScatterChart
            title=format!("Electricity access vs. renewable TFEC ({year})")
            subtitle=Some(format!("{} countries with both metrics reported", points.len()))
            x_label="Electricity access (% population)"
            y_label="Renewable TFEC share (%)"
            points=points
        />
    }
}

fn trend_chart(
    dataset: &ExploreData,
    metric_key: &str,
    year_start: i32,
    year_end: i32,
    countries: &[String],
) -> impl IntoView {
    let metric_label = dataset
        .metrics
        .iter()
        .find(|metric| metric.key == metric_key)
        .map(|metric| metric.label.clone())
        .unwrap_or_else(|| "Value".to_string());
    let value_format = value_format_for_metric(metric_key);

    let series: Vec<LineSeries> = countries
        .iter()
        .enumerate()
        .map(|(index, country)| LineSeries {
            label: country.clone(),
            color: LINE_COLORS[index % LINE_COLORS.len()],
            points: country_series(&dataset.records, country, metric_key, year_start, year_end),
        })
        .collect();

    if series.iter().all(|entry| entry.points.is_empty()) {
        return view! {
            <p class="tt-chart-empty">"No data for the selected countries and year range."</p>
        }.into_view();
    }

    let country_list = if countries.len() <= 3 {
        countries.join(", ")
    } else {
        format!("{} countries", countries.len())
    };
    let subtitle = format!(
        "{country_list} · {metric_label} · {year_start}–{year_end}"
    );

    view! {
        <LineChart
            title="Country energy share over time"
            subtitle=Some(subtitle)
            y_label=metric_label
            series=series
            year_start=year_start
            year_end=year_end
            value_format=value_format
        />
    }
    .into_view()
}

fn heatmap_chart(
    dataset: &ExploreData,
    metric_key: &str,
    year_start: i32,
    year_end: i32,
) -> impl IntoView {
    let end_year = year_end;
    let metric_label = dataset
        .metrics
        .iter()
        .find(|metric| metric.key == metric_key)
        .map(|metric| metric.label.clone())
        .unwrap_or_else(|| "Value".to_string());
    let countries = top_countries_by_metric(&dataset.records, end_year, metric_key, 16);
    let years: Vec<i32> = (year_start..=year_end).collect();
    let values = heatmap_values(&dataset.records, &countries, metric_key, year_start, year_end);

    view! {
        <HeatmapChart
            title=format!("Top countries: {metric_label}")
            subtitle=Some(format!("Ranked by {end_year} level · darker = lower, brighter = higher"))
            countries=countries
            years=years
            values=values
        />
    }
}

fn histogram_chart(dataset: &ExploreData, metric_key: &str, year: i32) -> impl IntoView {
    let metric_label = dataset
        .metrics
        .iter()
        .find(|metric| metric.key == metric_key)
        .map(|metric| metric.label.clone())
        .unwrap_or_else(|| "Value".to_string());
    let bins = histogram_bins(&dataset.records, year, metric_key, 12);
    let total: usize = bins.iter().map(|(_, _, count)| count).sum();

    view! {
        <HistogramChart
            title=format!("Distribution of {metric_label} ({year})")
            subtitle=Some(format!("{total} countries reporting non-zero values"))
            x_label="TFEC share (%)"
            bins=bins
        />
    }
}

fn band_points(series: Vec<YearDistribution>) -> Vec<MedianBandPoint> {
    series
        .into_iter()
        .map(|entry| MedianBandPoint {
            year: entry.year,
            median: entry.median,
            q1: entry.q1,
            q3: entry.q3,
            count: entry.count,
        })
        .collect()
}

fn median_trends_chart(dataset: &ExploreData, year_start: i32, year_end: i32) -> impl IntoView {
    let trajectory = median_trajectory_data(&dataset.records, year_start, year_end);
    let insights = median_trajectory_insights(&trajectory);

    view! {
        <MedianTrendsChart
            access_snapshots=band_points(trajectory.access_snapshots)
            renewable=band_points(trajectory.renewable)
            solar_adopters=band_points(trajectory.solar_adopters)
            wind_adopters=band_points(trajectory.wind_adopters)
            insights=insights
            year_start=year_start
            year_end=year_end
        />
    }
    .into_view()
}

fn adoption_wave_chart(dataset: &ExploreData, year_start: i32, year_end: i32) -> impl IntoView {
    let points = adoption_wave_data(&dataset.records, year_start, year_end);
    let stat_cards = adoption_wave_stat_cards(&points)
        .into_iter()
        .map(|card| StatCardData {
            label: card.label,
            value: card.value,
            detail: card.detail,
        })
        .collect();
    let insights = adoption_wave_insights(&points);
    let milestone = adoption_wave_crossover(&points);
    let chart_points = points
        .into_iter()
        .map(|point| AdoptionWavePoint {
            year: point.year,
            solar: point.solar,
            wind: point.wind,
            solar_only: point.solar_only,
            wind_only: point.wind_only,
            both: point.both,
            either: point.either,
        })
        .collect();

    view! {
        <AdoptionWaveChart
            points=chart_points
            stat_cards=stat_cards
            insights=insights
            milestone=milestone
        />
    }
}

fn delta_chart(
    dataset: &ExploreData,
    metric_key: &str,
    year_start: i32,
    year_end: i32,
) -> impl IntoView {
    let use_percent = metric_key.ends_with("_pct") || metric_key.ends_with("_tfec");
    let (x_label, title_metric) = match metric_key {
        "elec_pct" => ("Electricity access (%)", "electricity access"),
        "renew_tfec" => ("Renewable TFEC (%)", "renewable TFEC share"),
        "solar_tfec" => ("Solar TFEC (%)", "solar TFEC share"),
        "wind_tfec" => ("Wind TFEC (%)", "wind TFEC share"),
        "solar_tj" => ("Solar terajoules", "solar consumption"),
        _ => ("Value", "metric"),
    };

    let rows: Vec<DumbbellRow> = top_country_deltas(
        &dataset.records,
        year_start,
        year_end,
        metric_key,
        12,
    )
    .into_iter()
    .map(|row| DumbbellRow {
        country: row.country,
        start: row.start,
        end: row.end,
    })
    .collect();

    view! {
        <DumbbellChart
            title=format!("Top gainers: {title_metric}")
            subtitle=Some(format!("{year_start} → {year_end} · blue = start, purple = end"))
            x_label=x_label
            rows=rows
            use_percent=use_percent
        />
    }
}

fn bump_chart(dataset: &ExploreData) -> impl IntoView {
    let snapshots = [1990, 1995, 2000, 2005, 2010];
    let top_n = 5_usize;
    let (race, max_rank) = solar_rank_race_series(&dataset.records, "solar_tj", &snapshots, top_n);
    let series: Vec<BumpSeriesData> = race
        .into_iter()
        .map(|entry| BumpSeriesData {
            label: entry.label,
            color: entry.color,
            points: entry
                .points
                .into_iter()
                .map(|point| BumpPointData {
                    year: point.year,
                    rank: point.rank,
                    value: point.value,
                })
                .collect(),
            highlight: entry.highlight,
        })
        .collect();

    let china_climb = series
        .iter()
        .find(|entry| entry.label == "China")
        .and_then(|entry| {
            let first = entry.points.first()?;
            let last = entry.points.last()?;
            Some(format!(
                "China climbs from #{} ({}) to #{} ({}).",
                first.rank, first.year, last.rank, last.year
            ))
        });

    view! {
        <BumpChart
            title="Solar consumption rank race".to_string()
            subtitle=china_climb.or_else(|| {
                Some("Five-year snapshots by terajoules. Line crossings are overtakes.".to_string())
            })
            series=series
            snapshot_years=snapshots.to_vec()
            max_rank=max_rank
            top_n=top_n
        />
    }
}

fn rank_chart(
    dataset: &ExploreData,
    min_year: i32,
    max_year: i32,
    year: i32,
    count: usize,
    ascending: bool,
) -> impl IntoView {
    let rows = solar_ranking(&dataset.records, year, count, ascending);

    if rows.is_empty() {
        return view! {
            <p class="tt-chart-empty">
                {format!(
                    "No solar consumption data for {year}. Dataset covers {min_year}–{max_year}."
                )}
            </p>
        }.into_view();
    }

    let labels: Vec<String> = rows.iter().map(|(country, _)| country.clone()).collect();
    let values: Vec<f64> = rows.iter().map(|(_, value)| *value).collect();
    let title = if ascending {
        format!("Lowest solar consumption in {year}")
    } else {
        format!("Highest solar consumption in {year}")
    };

    view! {
        <HorizontalBarChart
            title=title
            x_label="Solar energy consumption (terajoules)"
            labels=labels
            values=values
            color="#0072b2"
        />
    }
    .into_view()
}
