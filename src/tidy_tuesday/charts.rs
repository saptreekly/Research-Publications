use leptos::*;

const PAD_LEFT: f64 = 140.0;
const PAD_RIGHT: f64 = 24.0;
const PAD_TOP: f64 = 28.0;
const PAD_BOTTOM: f64 = 48.0;

fn format_percentage_points(value: f64) -> String {
    if value.abs() >= 0.01 {
        format!("{value:.2} pp/yr")
    } else if value.abs() >= 0.001 {
        format!("{value:.3} pp/yr")
    } else {
        format!("{value:.4} pp/yr")
    }
}

fn y_tick_values(max_value: f64) -> Vec<f64> {
    if max_value <= 0.0 {
        return vec![0.0];
    }
    (0..=4).map(|step| max_value * step as f64 / 4.0).collect()
}

fn nice_max(value: f64) -> f64 {
    if value <= 0.0 {
        return 1.0;
    }
    let magnitude = 10_f64.powf(value.log10().floor());
    let normalized = value / magnitude;
    let nice = if normalized <= 1.0 {
        1.0
    } else if normalized <= 2.0 {
        2.0
    } else if normalized <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice * magnitude
}

#[component]
pub fn HorizontalBarChart(
    title: String,
    x_label: &'static str,
    labels: Vec<String>,
    values: Vec<f64>,
    color: &'static str,
) -> impl IntoView {
    let width = 920.0;
    let row_height = 28.0;
    let height = PAD_TOP + PAD_BOTTOM + row_height * labels.len().max(1) as f64;
    let plot_width = width - PAD_LEFT - PAD_RIGHT;
    let max_value = nice_max(values.iter().copied().fold(0.0, f64::max));
    let title_for_label = title.clone();

    view! {
        <figure class="tt-chart">
            <figcaption class="tt-chart-title">{title}</figcaption>
            <svg
                class="tt-chart-svg"
                viewBox=format!("0 0 {width} {height}")
                role="img"
                aria-label=title_for_label
            >
                <text
                    x=width / 2.0
                    y=height - 10.0
                    text-anchor="middle"
                    class="tt-chart-axis-label"
                >
                    {x_label}
                </text>

                {labels.into_iter().enumerate().map(|(index, label)| {
                    let value = *values.get(index).unwrap_or(&0.0);
                    let y = PAD_TOP + row_height * index as f64 + row_height * 0.5;
                    let bar_width = if max_value > 0.0 {
                        (value / max_value) * plot_width
                    } else {
                        0.0
                    };
                    let display_label = if label.len() > 18 {
                        format!("{}…", &label[..17])
                    } else {
                        label.clone()
                    };

                    view! {
                        <g class="tt-bar-row">
                            <title>{format!("{label}: {value:.2}")}</title>
                            <text
                                x=PAD_LEFT - 10.0
                                y=y
                                text-anchor="end"
                                dominant-baseline="middle"
                                class="tt-chart-y-label"
                            >
                                {display_label}
                            </text>
                            <rect
                                x=PAD_LEFT
                                y=y - 10.0
                                width=bar_width
                                height=20.0
                                fill=color
                                rx=2.0
                            />
                            <text
                                x=PAD_LEFT + bar_width + 6.0
                                y=y
                                dominant-baseline="middle"
                                class="tt-chart-value-label"
                            >
                                {format!("{value:.1}")}
                            </text>
                        </g>
                    }
                }).collect_view()}
            </svg>
        </figure>
    }
}

#[component]
pub fn VerticalBarChart(
    title: &'static str,
    subtitle: Option<String>,
    y_label: &'static str,
    labels: Vec<String>,
    values: Vec<f64>,
    color: &'static str,
) -> impl IntoView {
    let width = 920.0;
    let height = 400.0;
    let plot_width = width - PAD_LEFT - PAD_RIGHT;
    let plot_height = height - PAD_TOP - PAD_BOTTOM;
    let count = labels.len().max(1);
    let bar_gap = 12.0;
    let bar_width = (plot_width / count as f64) - bar_gap;
    let max_value = nice_max(values.iter().copied().fold(0.0, f64::max));
    let baseline = PAD_TOP + plot_height;
    let ticks = y_tick_values(max_value);

    view! {
        <figure class="tt-chart">
            <figcaption class="tt-chart-title">{title}</figcaption>
            {subtitle.map(|text| view! {
                <p class="tt-chart-subtitle">{text}</p>
            })}
            <svg
                class="tt-chart-svg"
                viewBox=format!("0 0 {width} {height}")
                role="img"
                aria-label=title
            >
                {ticks.iter().map(|tick| {
                    let y = baseline - (tick / max_value) * plot_height;
                    view! {
                        <g class="tt-grid-line">
                            <line
                                x1=PAD_LEFT
                                y1=y
                                x2=PAD_LEFT + plot_width
                                y2=y
                                stroke="var(--line-color)"
                                stroke-width="1"
                            />
                            <text
                                x=PAD_LEFT - 8.0
                                y=y + 4.0
                                text-anchor="end"
                                class="tt-chart-y-label"
                            >
                                {format_percentage_points(*tick)}
                            </text>
                        </g>
                    }
                }).collect_view()}

                <line
                    x1=PAD_LEFT
                    y1=baseline
                    x2=PAD_LEFT + plot_width
                    y2=baseline
                    stroke="var(--text-muted)"
                    stroke-width="1.5"
                />

                <text
                    x=24.0
                    y=PAD_TOP + plot_height / 2.0
                    text-anchor="middle"
                    transform=format!("rotate(-90 24 {})", PAD_TOP + plot_height / 2.0)
                    class="tt-chart-axis-label"
                >
                    {y_label}
                </text>

                {labels.into_iter().enumerate().map(|(index, label)| {
                    let value = *values.get(index).unwrap_or(&0.0);
                    let x = PAD_LEFT + index as f64 * (bar_width + bar_gap);
                    let bar_height = if max_value > 0.0 {
                        (value / max_value) * plot_height
                    } else {
                        0.0
                    };
                    let y = baseline - bar_height;
                    let value_label = format_percentage_points(value);
                    let display_label = if label.len() > 12 {
                        format!("{}…", &label[..11])
                    } else {
                        label.clone()
                    };

                    view! {
                        <g class="tt-bar-column">
                            <title>{format!("{label}: {value_label}")}</title>
                            <rect
                                x=x
                                y=y
                                width=bar_width
                                height=bar_height.max(1.0)
                                fill=color
                                rx=2.0
                            />
                            <text
                                x=x + bar_width / 2.0
                                y=y - 6.0
                                text-anchor="middle"
                                class="tt-chart-value-label"
                            >
                                {value_label}
                            </text>
                            <text
                                x=x + bar_width / 2.0
                                y=height - 14.0
                                text-anchor="middle"
                                class="tt-chart-x-label"
                            >
                                {display_label}
                            </text>
                        </g>
                    }
                }).collect_view()}
            </svg>
        </figure>
    }
}

#[derive(Clone, Copy)]
pub enum ValueFormat {
    Percent,
    Terajoules,
}

pub fn value_format_for_metric(metric_key: &str) -> ValueFormat {
    if metric_key.ends_with("_tj") {
        ValueFormat::Terajoules
    } else {
        ValueFormat::Percent
    }
}

fn format_metric_value(value: f64, format: ValueFormat) -> String {
    match format {
        ValueFormat::Percent => format!("{value:.2}%"),
        ValueFormat::Terajoules => {
            if value >= 1_000_000.0 {
                format!("{:.1}M TJ", value / 1_000_000.0)
            } else if value >= 1_000.0 {
                format!("{:.1}k TJ", value / 1_000.0)
            } else {
                format!("{value:.0} TJ")
            }
        }
    }
}

fn y_domain(values: &[f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 1.0);
    }
    let data_min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let data_max = values.iter().copied().fold(0.0, f64::max);
    if data_max <= data_min {
        return (0.0, nice_max(data_max.max(1.0)));
    }
    let range = data_max - data_min;
    let y_min = (data_min - range * 0.08).max(0.0);
    let y_max = data_max + range * 0.12;
    (y_min, y_max.max(y_min + 0.001))
}

fn value_to_y(value: f64, y_min: f64, y_max: f64, baseline: f64, plot_height: f64) -> f64 {
    if y_max <= y_min {
        return baseline;
    }
    baseline - ((value - y_min) / (y_max - y_min)) * plot_height
}

fn y_tick_values_range(y_min: f64, y_max: f64) -> Vec<f64> {
    if y_max <= y_min {
        return vec![y_min];
    }
    (0..=4)
        .map(|step| y_min + (y_max - y_min) * step as f64 / 4.0)
        .collect()
}
#[derive(Clone)]
pub struct LineSeries {
    pub label: String,
    pub color: &'static str,
    pub points: Vec<(i32, f64)>,
}

#[derive(Clone)]
struct HoverPoint {
    label: String,
    year: i32,
    value: f64,
    x: f64,
    y: f64,
    color: &'static str,
}

#[component]
pub fn LineChart(
    title: &'static str,
    subtitle: Option<String>,
    y_label: String,
    series: Vec<LineSeries>,
    year_start: i32,
    year_end: i32,
    value_format: ValueFormat,
) -> impl IntoView {
    let width = 920.0;
    let height = 440.0;
    let label_pad = 100.0;
    let plot_width = width - PAD_LEFT - PAD_RIGHT - label_pad;
    let plot_height = height - PAD_TOP - PAD_BOTTOM;
    let year_span = (year_end - year_start).max(1) as f64;
    let baseline = PAD_TOP + plot_height;

    let all_values: Vec<f64> = series
        .iter()
        .flat_map(|entry| entry.points.iter().map(|(_, value)| *value))
        .collect();
    let (y_min, y_max) = y_domain(&all_values);
    let y_ticks = y_tick_values_range(y_min, y_max);
    let hover = create_rw_signal(None::<HoverPoint>);
    let series = series.clone();

    view! {
        <figure class="tt-chart tt-line-chart">
            <figcaption class="tt-chart-title">{title}</figcaption>
            {subtitle.map(|text| view! {
                <p class="tt-chart-subtitle">{text}</p>
            })}
            <div class="tt-line-legend">
                {series.iter().map(|entry| view! {
                    <span class="tt-legend-item">
                        <span
                            class="tt-legend-swatch"
                            style=format!("background: {}", entry.color)
                        ></span>
                        {entry.label.clone()}
                    </span>
                }).collect_view()}
            </div>
            <svg
                class="tt-chart-svg"
                viewBox=format!("0 0 {width} {height}")
                role="img"
                aria-label=title
            >
                {y_ticks.iter().map(|tick| {
                    let y = value_to_y(*tick, y_min, y_max, baseline, plot_height);
                    view! {
                        <g class="tt-grid-line">
                            <line
                                x1=PAD_LEFT
                                y1=y
                                x2=PAD_LEFT + plot_width
                                y2=y
                                stroke="var(--line-color)"
                                stroke-width="1"
                            />
                            <text
                                x=PAD_LEFT - 8.0
                                y=y + 4.0
                                text-anchor="end"
                                class="tt-chart-y-label"
                            >
                                {format_metric_value(*tick, value_format)}
                            </text>
                        </g>
                    }
                }).collect_view()}

                <line
                    x1=PAD_LEFT
                    y1=baseline
                    x2=PAD_LEFT + plot_width
                    y2=baseline
                    stroke="var(--text-muted)"
                    stroke-width="1.5"
                />

                {(year_start..=year_end)
                    .step_by(((year_end - year_start) / 4).max(1) as usize)
                    .map(|year| {
                        let x = PAD_LEFT
                            + ((year - year_start) as f64 / year_span) * plot_width;
                        view! {
                            <g class="tt-grid-line">
                                <line
                                    x1=x
                                    y1=PAD_TOP
                                    x2=x
                                    y2=baseline
                                    stroke="var(--line-color)"
                                    stroke-width="1"
                                    stroke-dasharray="4 4"
                                />
                                <text
                                    x=x
                                    y=baseline + 18.0
                                    text-anchor="middle"
                                    class="tt-chart-x-label"
                                >
                                    {year.to_string()}
                                </text>
                            </g>
                        }
                    })
                    .collect_view()}

                <text
                    x=24.0
                    y=PAD_TOP + plot_height / 2.0
                    text-anchor="middle"
                    transform=format!("rotate(-90 24 {})", PAD_TOP + plot_height / 2.0)
                    class="tt-chart-axis-label"
                >
                    {y_label.clone()}
                </text>

                {series.clone().into_iter().map(|entry| {
                    if entry.points.len() < 2 {
                        return view! { <g></g> }.into_view();
                    }

                    let line_path = entry
                        .points
                        .iter()
                        .enumerate()
                        .map(|(index, (year, value))| {
                            let x = PAD_LEFT
                                + ((*year - year_start) as f64 / year_span) * plot_width;
                            let y = value_to_y(*value, y_min, y_max, baseline, plot_height);
                            if index == 0 {
                                format!("M {x} {y}")
                            } else {
                                format!("L {x} {y}")
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    let first_x = PAD_LEFT
                        + ((entry.points[0].0 - year_start) as f64 / year_span) * plot_width;
                    let last = entry.points[entry.points.len() - 1];
                    let last_x = PAD_LEFT
                        + ((last.0 - year_start) as f64 / year_span) * plot_width;
                    let last_y = value_to_y(last.1, y_min, y_max, baseline, plot_height);
                    let area_path = format!(
                        "{line_path} L {last_x} {baseline} L {first_x} {baseline} Z"
                    );

                    let end_label = format_metric_value(last.1, value_format);
                    let short_label = if entry.label.len() > 14 {
                        format!("{}…", &entry.label[..13])
                    } else {
                        entry.label.clone()
                    };

                    view! {
                        <g class="tt-line-series">
                            <path
                                d=area_path
                                fill=entry.color
                                fill-opacity="0.12"
                                stroke="none"
                            />
                            <path
                                d=line_path
                                fill="none"
                                stroke=entry.color
                                stroke-width="2.5"
                                stroke-linejoin="round"
                                stroke-linecap="round"
                            />
                            {entry.points.iter().map(|(year, value)| {
                                let x = PAD_LEFT
                                    + ((*year - year_start) as f64 / year_span) * plot_width;
                                let y = value_to_y(*value, y_min, y_max, baseline, plot_height);
                                let label = entry.label.clone();
                                let color = entry.color;
                                let point_year = *year;
                                let point_value = *value;
                                view! {
                                    <circle
                                        cx=x
                                        cy=y
                                        r=8.0
                                        fill="transparent"
                                        class="tt-line-hit"
                                        on:mouseenter=move |_| {
                                            hover.set(Some(HoverPoint {
                                                label: label.clone(),
                                                year: point_year,
                                                value: point_value,
                                                x,
                                                y,
                                                color,
                                            }));
                                        }
                                        on:mouseleave=move |_| hover.set(None)
                                    />
                                    <circle
                                        cx=x
                                        cy=y
                                        r=3.5
                                        fill=color
                                        class="tt-line-point"
                                    />
                                }
                            }).collect_view()}
                            <text
                                x=last_x + 8.0
                                y=last_y + 4.0
                                class="tt-line-end-label"
                                fill=entry.color
                            >
                                {format!("{short_label} ({end_label})")}
                            </text>
                        </g>
                    }.into_view()
                }).collect_view()}

                {move || hover.get().map(|point| {
                    let label = format_metric_value(point.value, value_format);
                    let text = format!("{} · {} · {}", point.label, point.year, label);
                    let tooltip_width = (text.len() as f64 * 6.2).clamp(120.0, 280.0);
                    let half_width = tooltip_width / 2.0;
                    let tooltip_x = point.x.clamp(PAD_LEFT + half_width, PAD_LEFT + plot_width - half_width);
                    let tooltip_y = (point.y - 14.0).max(PAD_TOP + 12.0);
                    view! {
                        <g class="tt-line-tooltip" transform=format!("translate({tooltip_x},{tooltip_y})")>
                            <rect
                                x=-half_width
                                y=-22.0
                                width=tooltip_width
                                height=20.0
                                rx=3.0
                                fill="var(--panel-surface)"
                                stroke=point.color
                                stroke-width="1.5"
                            />
                            <text
                                text-anchor="middle"
                                y=-8.0
                                class="tt-line-tooltip-text"
                                fill="var(--text-color)"
                            >
                                {text}
                            </text>
                        </g>
                    }
                })}
            </svg>
        </figure>
    }
}

#[derive(Clone)]
pub struct StatCardData {
    pub label: String,
    pub value: String,
    pub detail: String,
}

#[component]
pub fn StatCards(cards: Vec<StatCardData>) -> impl IntoView {
    view! {
        <div class="tt-stat-grid">
            {cards.into_iter().map(|card| view! {
                <div class="tt-stat-card">
                    <span class="tt-stat-label">{card.label}</span>
                    <span class="tt-stat-value">{card.value}</span>
                    <span class="tt-stat-detail">{card.detail}</span>
                </div>
            }).collect_view()}
        </div>
    }
}

#[derive(Clone)]
pub struct MedianBandPoint {
    pub year: i32,
    pub median: f64,
    pub q1: f64,
    pub q3: f64,
    pub count: usize,
}

#[component]
pub fn MedianTrendsChart(
    access_snapshots: Vec<MedianBandPoint>,
    renewable: Vec<MedianBandPoint>,
    solar_adopters: Vec<MedianBandPoint>,
    wind_adopters: Vec<MedianBandPoint>,
    insights: Vec<String>,
    year_start: i32,
    year_end: i32,
) -> impl IntoView {
    let width = 920.0;
    let plot_height = 180.0;
    let panel_svg_height = plot_height + PAD_TOP + PAD_BOTTOM;
    let plot_width = width - PAD_LEFT - PAD_RIGHT;
    let year_span = (year_end - year_start).max(1) as f64;

    let x_for_year = move |year: i32| {
        PAD_LEFT + ((year - year_start) as f64 / year_span) * plot_width
    };

    if access_snapshots.is_empty() && renewable.is_empty() {
        return view! {
            <p class="tt-chart-empty">"Not enough median data across years."</p>
        }.into_view();
    }

    view! {
        <figure class="tt-chart tt-median-chart">
            <p class="tt-chart-subtitle">
                "Medians across countries with data each year, not population-weighted. Shaded bands show the middle 50% (IQR)."
            </p>

            <div class="tt-median-stack">
                <section class="tt-median-panel-block">
                    <header class="tt-median-panel-header">
                        <h4 class="tt-median-panel-title">"Electricity access"</h4>
                        <p class="tt-median-panel-note">
                            "Reported in 1990, 2000, and 2010 only in this extract. Bars show snapshots; the dashed line connects them."
                        </p>
                    </header>
                    <svg
                        class="tt-chart-svg tt-median-panel-svg"
                        viewBox=format!("0 0 {width} {panel_svg_height}")
                        role="img"
                        aria-label="Electricity access median snapshots"
                    >
                        {render_access_panel(
                            plot_height,
                            &access_snapshots,
                            x_for_year,
                            year_start,
                            year_end,
                            plot_width,
                        )}
                    </svg>
                </section>

                <section class="tt-median-panel-block">
                    <header class="tt-median-panel-header">
                        <h4 class="tt-median-panel-title">"Renewable TFEC share"</h4>
                        <p class="tt-median-panel-note">
                            "Annual median renewable share of total final energy consumption, with the spread across countries."
                        </p>
                    </header>
                    <svg
                        class="tt-chart-svg tt-median-panel-svg"
                        viewBox=format!("0 0 {width} {panel_svg_height}")
                        role="img"
                        aria-label="Renewable TFEC median trajectory"
                    >
                        {render_band_panel(
                            plot_height,
                            &renewable,
                            "#009e73",
                            x_for_year,
                            year_start,
                            year_end,
                            plot_width,
                            true,
                        )}
                    </svg>
                </section>

                <section class="tt-median-panel-block">
                    <header class="tt-median-panel-header">
                        <h4 class="tt-median-panel-title">"Solar and wind among adopters"</h4>
                        <p class="tt-median-panel-note">
                            "Median TFEC share in countries that report each technology above zero."
                        </p>
                    </header>
                    <svg
                        class="tt-chart-svg tt-median-panel-svg"
                        viewBox=format!("0 0 {width} {panel_svg_height}")
                        role="img"
                        aria-label="Solar and wind adopter medians"
                    >
                        {render_dual_band_panel(
                            plot_height,
                            &solar_adopters,
                            &wind_adopters,
                            x_for_year,
                            year_start,
                            year_end,
                            plot_width,
                        )}
                    </svg>
                </section>
            </div>

            {(!insights.is_empty()).then(|| view! {
                <div class="tt-median-findings">
                    <h4 class="tt-median-findings-title">"Key findings"</h4>
                    <ul class="tt-median-insights">
                        {insights.into_iter().map(|line| view! {
                            <li>{line}</li>
                        }).collect_view()}
                    </ul>
                </div>
            })}
        </figure>
    }
    .into_view()
}

fn render_access_panel(
    plot_height: f64,
    snapshots: &[MedianBandPoint],
    x_for_year: impl Fn(i32) -> f64 + Copy,
    year_start: i32,
    year_end: i32,
    plot_width: f64,
) -> impl IntoView {
    let baseline = PAD_TOP + plot_height;
    let max_value = nice_max(
        snapshots
            .iter()
            .map(|point| point.median)
            .fold(100.0, f64::max),
    );
    let bar_width = 36.0;

    view! {
        <g class="tt-median-panel">
            {(year_start..=year_end)
                .step_by(((year_end - year_start) / 4).max(1) as usize)
                .map(|year| {
                    let x = x_for_year(year);
                    view! {
                        <line
                            x1=x
                            y1=PAD_TOP
                            x2=x
                            y2=baseline
                            stroke="var(--line-color)"
                            stroke-width="1"
                            stroke-opacity="0.25"
                        />
                    }
                })
                .collect_view()}

            {snapshots.iter().map(|point| {
                let x = x_for_year(point.year);
                let bar_height = (point.median / max_value.max(0.001)) * plot_height;
                let y = baseline - bar_height;
                view! {
                    <g class="tt-median-access-bar">
                        <title>{format!(
                            "{}: {:.0}% median ({} countries)",
                            point.year,
                            point.median,
                            point.count
                        )}</title>
                        <rect
                            x=x - bar_width / 2.0
                            y=y
                            width=bar_width
                            height=bar_height.max(1.0)
                            fill="#56b4e9"
                            rx=2.0
                        />
                        <text x=x y=y - 6.0 text-anchor="middle" class="tt-chart-value-label">
                            {format!("{:.0}%", point.median)}
                        </text>
                        <text x=x y=baseline + 16.0 text-anchor="middle" class="tt-chart-x-label">
                            {point.year.to_string()}
                        </text>
                    </g>
                }
            }).collect_view()}

            {snapshots.windows(2).map(|pair| {
                let x0 = x_for_year(pair[0].year);
                let y0 = baseline - (pair[0].median / max_value.max(0.001)) * plot_height;
                let x1 = x_for_year(pair[1].year);
                let y1 = baseline - (pair[1].median / max_value.max(0.001)) * plot_height;
                view! {
                    <line
                        x1=x0
                        y1=y0
                        x2=x1
                        y2=y1
                        stroke="#56b4e9"
                        stroke-width="2"
                        stroke-dasharray="6 5"
                        stroke-opacity="0.7"
                    />
                }
            }).collect_view()}

            <line
                x1=PAD_LEFT
                y1=baseline
                x2=PAD_LEFT + plot_width
                y2=baseline
                stroke="var(--text-muted)"
                stroke-width="1.5"
            />
            <text
                x=24.0
                y=PAD_TOP + plot_height / 2.0
                text-anchor="middle"
                transform=format!("rotate(-90 24 {})", PAD_TOP + plot_height / 2.0)
                class="tt-chart-axis-label"
            >
                "Access %"
            </text>
        </g>
    }
}

fn render_band_panel(
    plot_height: f64,
    series: &[MedianBandPoint],
    color: &'static str,
    x_for_year: impl Fn(i32) -> f64 + Copy,
    year_start: i32,
    year_end: i32,
    plot_width: f64,
    show_x_labels: bool,
) -> impl IntoView {
    if series.is_empty() {
        return view! { <g></g> }.into_view();
    }

    let baseline = PAD_TOP + plot_height;
    let max_value = nice_max(
        series
            .iter()
            .map(|point| point.q3)
            .fold(0.0, f64::max) * 1.08,
    );

    let band_path = |points: &[MedianBandPoint], upper: bool| -> String {
        if points.is_empty() {
            return String::new();
        }
        let mut path = String::new();
        for (index, point) in points.iter().enumerate() {
            let x = x_for_year(point.year);
            let y = if upper {
                baseline - (point.q3 / max_value.max(0.001)) * plot_height
            } else {
                baseline - (point.q1 / max_value.max(0.001)) * plot_height
            };
            if index == 0 {
                path.push_str(&format!("M {x} {y}"));
            } else {
                path.push_str(&format!(" L {x} {y}"));
            }
        }
        if !upper {
            for point in points.iter().rev() {
                let x = x_for_year(point.year);
                let y = baseline - (point.q3 / max_value.max(0.001)) * plot_height;
                path.push_str(&format!(" L {x} {y}"));
            }
            path.push('Z');
        }
        path
    };

    let median_path = series
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let x = x_for_year(point.year);
            let y = baseline - (point.median / max_value.max(0.001)) * plot_height;
            if index == 0 {
                format!("M {x} {y}")
            } else {
                format!(" L {x} {y}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let lower_band = band_path(series, false);

    view! {
        <g class="tt-median-panel">
            {(year_start..=year_end)
                .step_by(((year_end - year_start) / 4).max(1) as usize)
                .map(|year| {
                    let x = x_for_year(year);
                    view! {
                        <line
                            x1=x
                            y1=PAD_TOP
                            x2=x
                            y2=baseline
                            stroke="var(--line-color)"
                            stroke-width="1"
                            stroke-opacity="0.25"
                        />
                    }
                })
                .collect_view()}

            <path d=lower_band fill=color fill-opacity="0.15" stroke="none" />
            <path d=median_path fill="none" stroke=color stroke-width="2.5" />

            {series.iter().map(|point| {
                let x = x_for_year(point.year);
                let y = baseline - (point.median / max_value.max(0.001)) * plot_height;
                view! {
                    <circle cx=x cy=y r=3.5 fill=color>
                        <title>{format!(
                            "{}: {:.1}% median, IQR {:.1}-{:.1}% ({} countries)",
                            point.year,
                            point.median,
                            point.q1,
                            point.q3,
                            point.count
                        )}</title>
                    </circle>
                }
            }).collect_view()}

            {show_x_labels.then(|| view! {
                {(year_start..=year_end)
                    .step_by(((year_end - year_start) / 4).max(1) as usize)
                    .map(|year| {
                        let x = x_for_year(year);
                        view! {
                            <text x=x y=baseline + 16.0 text-anchor="middle" class="tt-chart-x-label">
                                {year.to_string()}
                            </text>
                        }
                    })
                    .collect_view()}
            })}

            <line
                x1=PAD_LEFT
                y1=baseline
                x2=PAD_LEFT + plot_width
                y2=baseline
                stroke="var(--text-muted)"
                stroke-width="1.5"
            />
            <text
                x=24.0
                y=PAD_TOP + plot_height / 2.0
                text-anchor="middle"
                transform=format!("rotate(-90 24 {})", PAD_TOP + plot_height / 2.0)
                class="tt-chart-axis-label"
            >
                "TFEC %"
            </text>
        </g>
    }
    .into_view()
}

fn render_adopter_line(
    points: &[MedianBandPoint],
    color: &'static str,
    baseline: f64,
    max_value: f64,
    plot_height: f64,
    x_for_year: impl Fn(i32) -> f64 + Copy,
) -> impl IntoView {
    let path = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let x = x_for_year(point.year);
            let y = baseline - (point.median / max_value.max(0.001)) * plot_height;
            if index == 0 {
                format!("M {x} {y}")
            } else {
                format!(" L {x} {y}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    view! {
        <path d=path fill="none" stroke=color stroke-width="2.5" />
        {points.iter().map(|point| {
            let x = x_for_year(point.year);
            let y = baseline - (point.median / max_value.max(0.001)) * plot_height;
            view! {
                <circle cx=x cy=y r=3.5 fill=color>
                    <title>{format!(
                        "{}: {:.3}% median ({} countries)",
                        point.year,
                        point.median,
                        point.count
                    )}</title>
                </circle>
            }
        }).collect_view()}
    }
}

fn render_dual_band_panel(
    plot_height: f64,
    solar: &[MedianBandPoint],
    wind: &[MedianBandPoint],
    x_for_year: impl Fn(i32) -> f64 + Copy,
    year_start: i32,
    year_end: i32,
    plot_width: f64,
) -> impl IntoView {
    let baseline = PAD_TOP + plot_height;
    let max_value = nice_max(
        solar
            .iter()
            .chain(wind.iter())
            .map(|point| point.median)
            .fold(0.0, f64::max)
            * 1.15,
    );

    view! {
        <g class="tt-median-panel">
            {(year_start..=year_end)
                .step_by(((year_end - year_start) / 4).max(1) as usize)
                .map(|year| {
                    let x = x_for_year(year);
                    view! {
                        <line
                            x1=x
                            y1=PAD_TOP
                            x2=x
                            y2=baseline
                            stroke="var(--line-color)"
                            stroke-width="1"
                            stroke-opacity="0.25"
                        />
                    }
                })
                .collect_view()}

            {render_adopter_line(solar, "#f0e442", baseline, max_value, plot_height, x_for_year)}
            {render_adopter_line(wind, "#56b4e9", baseline, max_value, plot_height, x_for_year)}

            {(year_start..=year_end)
                .step_by(((year_end - year_start) / 4).max(1) as usize)
                .map(|year| {
                    let x = x_for_year(year);
                    view! {
                        <text x=x y=baseline + 16.0 text-anchor="middle" class="tt-chart-x-label">
                            {year.to_string()}
                        </text>
                    }
                })
                .collect_view()}

            <line
                x1=PAD_LEFT
                y1=baseline
                x2=PAD_LEFT + plot_width
                y2=baseline
                stroke="var(--text-muted)"
                stroke-width="1.5"
            />
            <text
                x=24.0
                y=PAD_TOP + plot_height / 2.0
                text-anchor="middle"
                transform=format!("rotate(-90 24 {})", PAD_TOP + plot_height / 2.0)
                class="tt-chart-axis-label"
            >
                "TFEC %"
            </text>

            <g class="tt-median-mini-legend" transform=format!("translate({}, 24)", PAD_LEFT + plot_width - 180.0)>
                <rect x=0.0 y=0.0 width=170.0 height=34.0 fill="var(--panel-surface)" stroke="var(--line-color)" rx=2.0 />
                <circle cx=12.0 cy=12.0 r=4.0 fill="#f0e442" />
                <text x=22.0 y=15.0 class="tt-chart-y-label">"Solar adopters"</text>
                <circle cx=12.0 cy=26.0 r=4.0 fill="#56b4e9" />
                <text x=22.0 y=29.0 class="tt-chart-y-label">"Wind adopters"</text>
            </g>
        </g>
    }
}

#[derive(Clone)]
pub struct StackLayerSeries {
    pub label: &'static str,
    pub color: &'static str,
    pub points: Vec<(i32, f64)>,
}

#[component]
pub fn StackedAreaChart(
    title: String,
    subtitle: Option<String>,
    y_label: &'static str,
    layers: Vec<StackLayerSeries>,
    year_start: i32,
    year_end: i32,
) -> impl IntoView {
    let width = 920.0;
    let height = 420.0;
    let plot_width = width - PAD_LEFT - PAD_RIGHT;
    let plot_height = height - PAD_TOP - PAD_BOTTOM;
    let year_span = (year_end - year_start).max(1) as f64;
    let baseline = PAD_TOP + plot_height;
    let title_for_label = title.clone();

    let years: Vec<i32> = (year_start..=year_end).collect();
    let totals: Vec<f64> = years
        .iter()
        .map(|year| {
            layers
                .iter()
                .map(|layer| {
                    layer
                        .points
                        .iter()
                        .find(|(point_year, _)| point_year == year)
                        .map(|(_, value)| *value)
                        .unwrap_or(0.0)
                })
                .sum()
        })
        .collect();
    let max_total = nice_max(totals.iter().copied().fold(0.0, f64::max));

    let layer_paths: Vec<(String, String, &'static str)> = layers
        .iter()
        .enumerate()
        .map(|(layer_index, layer)| {
            let upper: Vec<f64> = years
                .iter()
                .map(|year| {
                    (0..=layer_index)
                        .map(|index| {
                            layers[index]
                                .points
                                .iter()
                                .find(|(point_year, _)| point_year == year)
                                .map(|(_, value)| *value)
                                .unwrap_or(0.0)
                        })
                        .sum()
                })
                .collect();
            let lower: Vec<f64> = years
                .iter()
                .map(|year| {
                    (0..layer_index)
                        .map(|index| {
                            layers[index]
                                .points
                                .iter()
                                .find(|(point_year, _)| point_year == year)
                                .map(|(_, value)| *value)
                                .unwrap_or(0.0)
                        })
                        .sum()
                })
                .collect();

            let mut path = String::new();
            for (index, year) in years.iter().enumerate() {
                let x = PAD_LEFT + ((*year - year_start) as f64 / year_span) * plot_width;
                let y_upper =
                    baseline - (upper[index] / max_total.max(0.001)) * plot_height;
                if index == 0 {
                    path.push_str(&format!("M {x} {y_upper}"));
                } else {
                    path.push_str(&format!(" L {x} {y_upper}"));
                }
            }
            for (index, year) in years.iter().enumerate().rev() {
                let x = PAD_LEFT + ((*year - year_start) as f64 / year_span) * plot_width;
                let y_lower =
                    baseline - (lower[years.len() - 1 - index] / max_total.max(0.001)) * plot_height;
                path.push_str(&format!(" L {x} {y_lower}"));
            }
            path.push_str(" Z");
            (layer.label.to_string(), path, layer.color)
        })
        .collect();

    view! {
        <figure class="tt-chart">
            <figcaption class="tt-chart-title">{title}</figcaption>
            {subtitle.map(|text| view! {
                <p class="tt-chart-subtitle">{text}</p>
            })}
            <div class="tt-line-legend">
                {layers.iter().map(|layer| view! {
                    <span class="tt-legend-item">
                        <span class="tt-legend-swatch" style=format!("background: {}", layer.color)></span>
                        {layer.label}
                    </span>
                }).collect_view()}
            </div>
            <svg
                class="tt-chart-svg"
                viewBox=format!("0 0 {width} {height}")
                role="img"
                aria-label=title_for_label
            >
                {y_tick_values(max_total).iter().map(|tick| {
                    let y = baseline - (tick / max_total.max(0.001)) * plot_height;
                    view! {
                        <g class="tt-grid-line">
                            <line
                                x1=PAD_LEFT
                                y1=y
                                x2=PAD_LEFT + plot_width
                                y2=y
                                stroke="var(--line-color)"
                                stroke-width="1"
                            />
                            <text
                                x=PAD_LEFT - 8.0
                                y=y + 4.0
                                text-anchor="end"
                                class="tt-chart-y-label"
                            >
                                {format!("{tick:.1}%")}
                            </text>
                        </g>
                    }
                }).collect_view()}

                {layer_paths.into_iter().map(|(label, path, color)| view! {
                    <g class="tt-stack-layer">
                        <title>{label.clone()}</title>
                        <path d=path fill=color fill-opacity="0.82" stroke=color stroke-width="0.5" />
                    </g>
                }).collect_view()}

                {(year_start..=year_end)
                    .step_by(((year_end - year_start) / 4).max(1) as usize)
                    .map(|year| {
                        let x = PAD_LEFT + ((year - year_start) as f64 / year_span) * plot_width;
                        view! {
                            <text x=x y=baseline + 18.0 text-anchor="middle" class="tt-chart-x-label">
                                {year.to_string()}
                            </text>
                        }
                    })
                    .collect_view()}

                <text
                    x=24.0
                    y=PAD_TOP + plot_height / 2.0
                    text-anchor="middle"
                    transform=format!("rotate(-90 24 {})", PAD_TOP + plot_height / 2.0)
                    class="tt-chart-axis-label"
                >
                    {y_label}
                </text>
            </svg>
        </figure>
    }
}

#[derive(Clone)]
pub struct ScatterPointData {
    pub label: String,
    pub x: f64,
    pub y: f64,
}

fn heat_color(value: f64, max_value: f64) -> String {
    let t = (value / max_value.max(0.001)).clamp(0.0, 1.0);
    let r = (26.0 + 142.0 * t) as u8;
    let g = (26.0 + 58.0 * t) as u8;
    let b = (46.0 + 201.0 * t) as u8;
    format!("rgb({r},{g},{b})")
}

#[component]
pub fn ScatterChart(
    title: String,
    subtitle: Option<String>,
    x_label: &'static str,
    y_label: &'static str,
    points: Vec<ScatterPointData>,
) -> impl IntoView {
    let width = 920.0;
    let height = 440.0;
    let plot_width = width - PAD_LEFT - PAD_RIGHT;
    let plot_height = height - PAD_TOP - PAD_BOTTOM;
    let baseline = PAD_TOP + plot_height;
    let title_for_label = title.clone();
    let hover = create_rw_signal(None::<ScatterPointData>);

    if points.is_empty() {
        return view! {
            <p class="tt-chart-empty">"No countries with both electricity access and renewable share for this year."</p>
        }.into_view();
    }

    let x_max = nice_max(points.iter().map(|point| point.x).fold(0.0, f64::max));
    let y_max = nice_max(points.iter().map(|point| point.y).fold(0.0, f64::max));
    let x_median = {
        let mut xs: Vec<f64> = points.iter().map(|point| point.x).collect();
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        xs[xs.len() / 2]
    };
    let y_median = {
        let mut ys: Vec<f64> = points.iter().map(|point| point.y).collect();
        ys.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        ys[ys.len() / 2]
    };

    view! {
        <figure class="tt-chart">
            <figcaption class="tt-chart-title">{title}</figcaption>
            {subtitle.map(|text| view! {
                <p class="tt-chart-subtitle">{text}</p>
            })}
            <svg
                class="tt-chart-svg"
                viewBox=format!("0 0 {width} {height}")
                role="img"
                aria-label=title_for_label
            >
                <line
                    x1=PAD_LEFT + (x_median / x_max.max(0.001)) * plot_width
                    y1=PAD_TOP
                    x2=PAD_LEFT + (x_median / x_max.max(0.001)) * plot_width
                    y2=baseline
                    stroke="var(--line-color)"
                    stroke-width="1"
                    stroke-dasharray="4 4"
                />
                <line
                    x1=PAD_LEFT
                    y1=baseline - (y_median / y_max.max(0.001)) * plot_height
                    x2=PAD_LEFT + plot_width
                    y2=baseline - (y_median / y_max.max(0.001)) * plot_height
                    stroke="var(--line-color)"
                    stroke-width="1"
                    stroke-dasharray="4 4"
                />

                {points.iter().cloned().map(|point| {
                    let x = PAD_LEFT + (point.x / x_max.max(0.001)) * plot_width;
                    let y = baseline - (point.y / y_max.max(0.001)) * plot_height;
                    let hover_point = point.clone();
                    view! {
                        <g class="tt-scatter-point">
                            <title>{format!("{}: {:.0}% access, {:.1}% renewable", point.label, point.x, point.y)}</title>
                            <circle
                                cx=x
                                cy=y
                                r=7.0
                                fill="#a855f7"
                                fill-opacity="0.35"
                                stroke="#a855f7"
                                stroke-width="1.5"
                                class="tt-scatter-hit"
                                on:mouseenter=move |_| hover.set(Some(hover_point.clone()))
                                on:mouseleave=move |_| hover.set(None)
                            />
                        </g>
                    }
                }).collect_view()}

                {move || hover.get().map(|point| {
                    let x = PAD_LEFT + (point.x / x_max.max(0.001)) * plot_width;
                    let y = baseline - (point.y / y_max.max(0.001)) * plot_height;
                    let text = format!(
                        "{} · {:.0}% access · {:.1}% renewable",
                        point.label, point.x, point.y
                    );
                    view! {
                        <g class="tt-line-tooltip" transform=format!("translate({x},{})", y - 12.0)>
                            <rect x=-90.0 y=-22.0 width=180.0 height=20.0 rx=3.0 fill="var(--panel-surface)" stroke="#a855f7" stroke-width="1.5" />
                            <text text-anchor="middle" y=-8.0 class="tt-line-tooltip-text" fill="var(--text-color)">
                                {text}
                            </text>
                        </g>
                    }
                })}

                <text x=PAD_LEFT + plot_width / 2.0 y=height - 10.0 text-anchor="middle" class="tt-chart-axis-label">
                    {x_label}
                </text>
                <text
                    x=24.0
                    y=PAD_TOP + plot_height / 2.0
                    text-anchor="middle"
                    transform=format!("rotate(-90 24 {})", PAD_TOP + plot_height / 2.0)
                    class="tt-chart-axis-label"
                >
                    {y_label}
                </text>
            </svg>
        </figure>
    }
    .into_view()
}

#[component]
pub fn HeatmapChart(
    title: String,
    subtitle: Option<String>,
    countries: Vec<String>,
    years: Vec<i32>,
    values: Vec<Vec<Option<f64>>>,
) -> impl IntoView {
    let width = 920.0;
    let cell_width = 28.0;
    let cell_height = 18.0;
    let label_width = 150.0;
    let top_pad = 48.0;
    let height = top_pad + cell_height * countries.len().max(1) as f64 + 24.0;
    let plot_width = years.len() as f64 * cell_width;
    let title_for_label = title.clone();

    let max_value = values
        .iter()
        .flat_map(|row| row.iter().filter_map(|value| *value))
        .fold(0.0_f64, f64::max);

    if max_value <= 0.0 {
        return view! {
            <p class="tt-chart-empty">"No data for this heatmap selection."</p>
        }.into_view();
    }

    view! {
        <figure class="tt-chart">
            <figcaption class="tt-chart-title">{title}</figcaption>
            {subtitle.map(|text| view! {
                <p class="tt-chart-subtitle">{text}</p>
            })}
            <svg
                class="tt-chart-svg"
                viewBox=format!("0 0 {width} {height}")
                role="img"
                aria-label=title_for_label
            >
                {years.iter().enumerate().map(|(index, year)| {
                    let x = label_width + index as f64 * cell_width + cell_width / 2.0;
                    view! {
                        <text x=x y=24.0 text-anchor="middle" class="tt-chart-x-label">
                            {if index % 2 == 0 { year.to_string() } else { String::new() }}
                        </text>
                    }
                }).collect_view()}

                {countries.iter().enumerate().map(|(row_index, country)| {
                    let display = if country.len() > 20 {
                        format!("{}…", &country[..19])
                    } else {
                        country.clone()
                    };
                    let y = top_pad + row_index as f64 * cell_height + cell_height / 2.0;
                    view! {
                        <g class="tt-heatmap-row">
                            <text
                                x=label_width - 8.0
                                y=y
                                text-anchor="end"
                                dominant-baseline="middle"
                                class="tt-chart-y-label"
                            >
                                {display}
                            </text>
                            {(0..years.len()).map(|col_index| {
                                let value = values
                                    .get(row_index)
                                    .and_then(|row| row.get(col_index))
                                    .copied()
                                    .flatten()
                                    .unwrap_or(0.0);
                                let x = label_width + col_index as f64 * cell_width;
                                let fill = if value > 0.0 {
                                    heat_color(value, max_value)
                                } else {
                                    "var(--surface-inset)".to_string()
                                };
                                view! {
                                    <rect
                                        x=x
                                        y=top_pad + row_index as f64 * cell_height
                                        width=cell_width - 1.0
                                        height=cell_height - 1.0
                                        fill=fill
                                        rx=1.0
                                    >
                                        <title>{format!("{country}, {}: {value:.2}", years[col_index])}</title>
                                    </rect>
                                }
                            }).collect_view()}
                        </g>
                    }
                }).collect_view()}

                <text
                    x=label_width + plot_width + 12.0
                    y=top_pad + 20.0
                    class="tt-chart-y-label"
                >
                    {format!("max {:.1}", max_value)}
                </text>
            </svg>
        </figure>
    }
    .into_view()
}

#[component]
pub fn HistogramChart(
    title: String,
    subtitle: Option<String>,
    x_label: &'static str,
    bins: Vec<(f64, f64, usize)>,
) -> impl IntoView {
    let width = 920.0;
    let height = 360.0;
    let plot_width = width - PAD_LEFT - PAD_RIGHT;
    let plot_height = height - PAD_TOP - PAD_BOTTOM;
    let baseline = PAD_TOP + plot_height;
    let title_for_label = title.clone();

    if bins.is_empty() {
        return view! {
            <p class="tt-chart-empty">"No non-zero values to plot for this metric and year."</p>
        }.into_view();
    }

    let max_count = bins.iter().map(|(_, _, count)| *count).max().unwrap_or(1) as f64;
    let max_end = bins.iter().map(|(_, end, _)| *end).fold(0.0, f64::max);
    let bar_gap = 2.0;
    let bar_width = (plot_width / bins.len() as f64) - bar_gap;

    view! {
        <figure class="tt-chart">
            <figcaption class="tt-chart-title">{title}</figcaption>
            {subtitle.map(|text| view! {
                <p class="tt-chart-subtitle">{text}</p>
            })}
            <svg
                class="tt-chart-svg"
                viewBox=format!("0 0 {width} {height}")
                role="img"
                aria-label=title_for_label
            >
                {bins.iter().enumerate().map(|(index, (start, end, count))| {
                    let bar_height = (*count as f64 / max_count) * plot_height;
                    let x = PAD_LEFT + index as f64 * (bar_width + bar_gap);
                    let y = baseline - bar_height;
                    view! {
                        <g class="tt-hist-bar">
                            <title>{format!("{start:.2}–{end:.2}: {count} countries")}</title>
                            <rect
                                x=x
                                y=y
                                width=bar_width
                                height=bar_height.max(1.0)
                                fill="#0072b2"
                                fill-opacity="0.85"
                                rx=1.0
                            />
                            <text
                                x=x + bar_width / 2.0
                                y=y - 4.0
                                text-anchor="middle"
                                class="tt-chart-value-label"
                            >
                                {count.to_string()}
                            </text>
                        </g>
                    }
                }).collect_view()}

                <line
                    x1=PAD_LEFT
                    y1=baseline
                    x2=PAD_LEFT + plot_width
                    y2=baseline
                    stroke="var(--text-muted)"
                    stroke-width="1.5"
                />

                <text x=PAD_LEFT + plot_width / 2.0 y=height - 10.0 text-anchor="middle" class="tt-chart-axis-label">
                    {x_label}
                </text>
                <text
                    x=24.0
                    y=PAD_TOP + plot_height / 2.0
                    text-anchor="middle"
                    transform=format!("rotate(-90 24 {})", PAD_TOP + plot_height / 2.0)
                    class="tt-chart-axis-label"
                >
                    "Countries"
                </text>
                <text x=PAD_LEFT + plot_width y=baseline + 16.0 text-anchor="end" class="tt-chart-x-label">
                    {format!("0 – {max_end:.1}")}
                </text>
            </svg>
        </figure>
    }
    .into_view()
}

#[derive(Clone)]
pub struct InsightCardData {
    pub title: String,
    pub highlight: String,
    pub body: String,
    pub accent: &'static str,
}

#[component]
pub fn InsightCards(insights: Vec<InsightCardData>) -> impl IntoView {
    view! {
        <div class="tt-insight-grid">
            {insights.into_iter().map(|insight| view! {
                <article class="tt-insight-card" style=format!("border-left-color: {}", insight.accent)>
                    <span class="tt-insight-highlight" style=format!("color: {}", insight.accent)>
                        {insight.highlight}
                    </span>
                    <h4 class="tt-insight-title">{insight.title}</h4>
                    <p class="tt-insight-body">{insight.body}</p>
                </article>
            }).collect_view()}
        </div>
    }
}

#[derive(Clone)]
pub struct AdoptionWavePoint {
    pub year: i32,
    pub solar: usize,
    pub wind: usize,
    pub solar_only: usize,
    pub wind_only: usize,
    pub both: usize,
    pub either: usize,
}

fn y_tick_values_count(max_count: usize) -> Vec<usize> {
    if max_count == 0 {
        return vec![0];
    }

    let step = match max_count {
        0..=20 => 5,
        21..=60 => 10,
        61..=120 => 20,
        _ => 25,
    };

    let mut ticks = vec![0];
    let mut value = step;
    while value < max_count {
        ticks.push(value);
        value += step;
    }
    if *ticks.last().unwrap_or(&0) != max_count {
        ticks.push(max_count);
    }
    ticks
}

#[component]
pub fn AdoptionWaveChart(
    points: Vec<AdoptionWavePoint>,
    stat_cards: Vec<StatCardData>,
    insights: Vec<String>,
    milestone: Option<(i32, String)>,
) -> impl IntoView {
    let width = 920.0;
    let height = 420.0;
    let label_room = 130.0;
    let plot_width = width - PAD_LEFT - PAD_RIGHT - label_room;
    let plot_height = height - PAD_TOP - PAD_BOTTOM;
    let baseline = PAD_TOP + plot_height;
    let plot_left = PAD_LEFT;

    if points.is_empty() {
        return view! {
            <p class="tt-chart-empty">"No adoption data for this period."</p>
        }.into_view();
    }

    let year_start = points[0].year;
    let year_end = points[points.len() - 1].year;
    let year_span = (year_end - year_start).max(1) as f64;
    let max_total = points
        .iter()
        .map(|point| point.either)
        .max()
        .unwrap_or(1)
        .max(1);

    let x_for_year = move |year: i32| {
        plot_left + ((year - year_start) as f64 / year_span) * plot_width
    };

    let layer_defs: [(&'static str, fn(&AdoptionWavePoint) -> usize, &'static str); 3] = [
        ("Solar only", |point| point.solar_only, "#f0e442"),
        ("Both", |point| point.both, "#a855f7"),
        ("Wind only", |point| point.wind_only, "#56b4e9"),
    ];

    let layer_paths: Vec<(&'static str, String, &'static str)> = layer_defs
        .iter()
        .enumerate()
        .map(|(layer_index, (label, _, color))| {
            let upper: Vec<usize> = points
                .iter()
                .map(|point| {
                    layer_defs[..=layer_index]
                        .iter()
                        .map(|(_, layer_fn, _)| layer_fn(point))
                        .sum()
                })
                .collect();
            let lower: Vec<usize> = points
                .iter()
                .map(|point| {
                    layer_defs[..layer_index]
                        .iter()
                        .map(|(_, layer_fn, _)| layer_fn(point))
                        .sum()
                })
                .collect();

            let mut path = String::new();
            for (index, point) in points.iter().enumerate() {
                let x = x_for_year(point.year);
                let y_upper =
                    baseline - (upper[index] as f64 / max_total as f64) * plot_height;
                if index == 0 {
                    path.push_str(&format!("M {x} {y_upper}"));
                } else {
                    path.push_str(&format!(" L {x} {y_upper}"));
                }
            }
            for (index, point) in points.iter().enumerate().rev() {
                let x = x_for_year(point.year);
                let lower_index = points.len() - 1 - index;
                let y_lower =
                    baseline - (lower[lower_index] as f64 / max_total as f64) * plot_height;
                path.push_str(&format!(" L {x} {y_lower}"));
            }
            path.push('Z');
            (*label, path, *color)
        })
        .collect();

    let total_path = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let x = x_for_year(point.year);
            let y = baseline - (point.either as f64 / max_total as f64) * plot_height;
            if index == 0 {
                format!("M {x} {y}")
            } else {
                format!(" L {x} {y}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let solar_path = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let x = x_for_year(point.year);
            let y = baseline - (point.solar as f64 / max_total as f64) * plot_height;
            if index == 0 {
                format!("M {x} {y}")
            } else {
                format!(" L {x} {y}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let wind_path = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let x = x_for_year(point.year);
            let y = baseline - (point.wind as f64 / max_total as f64) * plot_height;
            if index == 0 {
                format!("M {x} {y}")
            } else {
                format!(" L {x} {y}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let last = points.last().cloned().unwrap();
    let end_x = x_for_year(last.year);
    let end_y = baseline - (last.either as f64 / max_total as f64) * plot_height;

    view! {
        <figure class="tt-chart tt-adoption-chart">
            <p class="tt-chart-subtitle">
                "Stacked by reporting type: solar only, both, or wind only. Dashed lines trace total solar and wind counts (which can overlap)."
            </p>

            {(!stat_cards.is_empty()).then(|| view! {
                <StatCards cards=stat_cards />
            })}

            <div class="tt-adoption-legend">
                {layer_defs.iter().map(|(label, _, color)| view! {
                    <span class="tt-legend-item">
                        <span class="tt-legend-swatch" style=format!("background: {color}")></span>
                        {*label}
                    </span>
                }).collect_view()}
                <span class="tt-legend-item tt-legend-item-dashed">
                    <span class="tt-legend-swatch tt-legend-swatch-dashed" style="border-color: #f0e442"></span>
                    "Solar total"
                </span>
                <span class="tt-legend-item tt-legend-item-dashed">
                    <span class="tt-legend-swatch tt-legend-swatch-dashed" style="border-color: #56b4e9"></span>
                    "Wind total"
                </span>
            </div>

            <svg
                class="tt-chart-svg tt-adoption-svg"
                viewBox=format!("0 0 {width} {height}")
                role="img"
                aria-label="Technology adoption wave"
            >
                <defs>
                    <linearGradient id="tt-adoption-solar" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stop-color="#f0e442" stop-opacity="0.95" />
                        <stop offset="100%" stop-color="#f0e442" stop-opacity="0.55" />
                    </linearGradient>
                    <linearGradient id="tt-adoption-both" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stop-color="#a855f7" stop-opacity="0.92" />
                        <stop offset="100%" stop-color="#a855f7" stop-opacity="0.5" />
                    </linearGradient>
                    <linearGradient id="tt-adoption-wind" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stop-color="#56b4e9" stop-opacity="0.95" />
                        <stop offset="100%" stop-color="#56b4e9" stop-opacity="0.55" />
                    </linearGradient>
                </defs>

                {y_tick_values_count(max_total).iter().map(|tick| {
                    let y = baseline - (*tick as f64 / max_total as f64) * plot_height;
                    view! {
                        <g class="tt-grid-line">
                            <line
                                x1=plot_left
                                y1=y
                                x2=plot_left + plot_width
                                y2=y
                                stroke="var(--line-color)"
                                stroke-width="1"
                                stroke-opacity="0.45"
                            />
                            <text
                                x=plot_left - 8.0
                                y=y + 4.0
                                text-anchor="end"
                                class="tt-chart-y-label"
                            >
                                {tick.to_string()}
                            </text>
                        </g>
                    }
                }).collect_view()}

                {(year_start..=year_end)
                    .step_by(((year_end - year_start) / 5).max(1) as usize)
                    .map(|year| {
                        let x = x_for_year(year);
                        view! {
                            <line
                                x1=x
                                y1=PAD_TOP
                                x2=x
                                y2=baseline
                                stroke="var(--line-color)"
                                stroke-width="1"
                                stroke-opacity="0.2"
                            />
                        }
                    })
                    .collect_view()}

                {layer_paths.into_iter().enumerate().map(|(index, (label, path, _color))| {
                    let fill = match index {
                        0 => "url(#tt-adoption-solar)",
                        1 => "url(#tt-adoption-both)",
                        _ => "url(#tt-adoption-wind)",
                    };
                    view! {
                        <path d=path fill=fill stroke="none">
                            <title>{label}</title>
                        </path>
                    }
                }).collect_view()}

                <path
                    d=total_path
                    fill="none"
                    stroke="var(--text-color)"
                    stroke-width="2.5"
                    stroke-opacity="0.35"
                />

                <path
                    d=solar_path
                    fill="none"
                    stroke="#f0e442"
                    stroke-width="2"
                    stroke-dasharray="7 5"
                    stroke-opacity="0.9"
                />
                <path
                    d=wind_path
                    fill="none"
                    stroke="#56b4e9"
                    stroke-width="2"
                    stroke-dasharray="7 5"
                    stroke-opacity="0.9"
                />

                {points.iter().map(|point| {
                    let x = x_for_year(point.year);
                    let y = baseline - (point.either as f64 / max_total as f64) * plot_height;
                    view! {
                        <circle cx=x cy=y r=4.0 fill="var(--text-color)" fill-opacity="0.55">
                            <title>{format!(
                                "{year}: {either} countries ({solar_only} solar only, {both} both, {wind_only} wind only)",
                                year = point.year,
                                either = point.either,
                                solar_only = point.solar_only,
                                both = point.both,
                                wind_only = point.wind_only,
                            )}</title>
                        </circle>
                    }
                }).collect_view()}

                {milestone.map(|(year, label)| {
                    let x = x_for_year(year);
                    view! {
                        <g class="tt-adoption-milestone">
                            <line
                                x1=x
                                y1=PAD_TOP
                                x2=x
                                y2=baseline
                                stroke="#a855f7"
                                stroke-width="1.5"
                                stroke-dasharray="4 4"
                                stroke-opacity="0.75"
                            />
                            <text
                                x=x + 6.0
                                y=PAD_TOP + 14.0
                                class="tt-adoption-milestone-label"
                            >
                                {label}
                            </text>
                        </g>
                    }
                })}

                <g class="tt-adoption-end-label" transform=format!("translate({}, {})", end_x + 10.0, end_y - 28.0)>
                    <rect x=0.0 y=0.0 width=118.0 height=52.0 rx=3.0 fill="var(--panel-surface)" stroke="var(--line-color)" />
                    <text x=8.0 y=16.0 class="tt-adoption-end-title">{format!("{}", last.year)}</text>
                    <text x=8.0 y=30.0 class="tt-adoption-end-line" fill="#f0e442">{format!("Solar: {}", last.solar)}</text>
                    <text x=8.0 y=44.0 class="tt-adoption-end-line" fill="#56b4e9">{format!("Wind: {}", last.wind)}</text>
                </g>

                {(year_start..=year_end)
                    .step_by(((year_end - year_start) / 5).max(1) as usize)
                    .map(|year| {
                        let x = x_for_year(year);
                        view! {
                            <text x=x y=baseline + 18.0 text-anchor="middle" class="tt-chart-x-label">
                                {year.to_string()}
                            </text>
                        }
                    })
                    .collect_view()}

                <line
                    x1=plot_left
                    y1=baseline
                    x2=plot_left + plot_width
                    y2=baseline
                    stroke="var(--text-muted)"
                    stroke-width="1.5"
                />
                <text
                    x=24.0
                    y=PAD_TOP + plot_height / 2.0
                    text-anchor="middle"
                    transform=format!("rotate(-90 24 {})", PAD_TOP + plot_height / 2.0)
                    class="tt-chart-axis-label"
                >
                    "Unique countries"
                </text>
            </svg>

            {(!insights.is_empty()).then(|| view! {
                <div class="tt-median-findings">
                    <h4 class="tt-median-findings-title">"Key findings"</h4>
                    <ul class="tt-median-insights">
                        {insights.into_iter().map(|line| view! {
                            <li>{line}</li>
                        }).collect_view()}
                    </ul>
                </div>
            })}
        </figure>
    }
    .into_view()
}

#[derive(Clone)]
pub struct GrowthBarRow {
    pub label: String,
    pub color: &'static str,
    pub mean: f64,
    pub median: f64,
    pub countries: usize,
}

#[derive(Clone)]
pub struct GrowthLeaderGroup {
    pub technology: String,
    pub color: &'static str,
    pub leaders: Vec<(String, f64)>,
}

#[component]
pub fn RenewableGrowthChart(
    rows: Vec<GrowthBarRow>,
    leaders: Vec<GrowthLeaderGroup>,
    insights: Vec<String>,
    year_start: i32,
    year_end: i32,
) -> impl IntoView {
    let width = 920.0;
    let row_height = 44.0;
    let bar_height = 22.0;
    let chart_height = PAD_TOP + PAD_BOTTOM + row_height * rows.len().max(1) as f64;
    let plot_width = width - PAD_LEFT - PAD_RIGHT - 48.0;
    let max_value = nice_max(rows.iter().map(|row| row.mean).fold(0.0, f64::max) * 1.12);

    if rows.is_empty() || rows.iter().all(|row| row.countries == 0) {
        return view! {
            <p class="tt-chart-empty">
                {format!(
                    "Not enough country histories with 5+ reporting years between {year_start} and {year_end}. Try widening the range."
                )}
            </p>
        }.into_view();
    }

    view! {
        <figure class="tt-chart tt-growth-chart">
            <p class="tt-growth-method">
                {format!(
                    "Endpoint slopes averaged across countries with 5+ data points ({year_start}–{year_end}). Bar length = mean pp/yr; diamond = median."
                )}
            </p>

            <div class="tt-growth-bars">
                <svg
                    class="tt-growth-svg"
                    viewBox=format!("0 0 {width} {chart_height}")
                    role="img"
                    aria-label="Renewable technology growth rates"
                >
                        {(0..=4).map(|step| {
                            let tick = max_value * step as f64 / 4.0;
                            let x = PAD_LEFT + (tick / max_value.max(0.0001)) * plot_width;
                            view! {
                                <g class="tt-grid-line">
                                    <line
                                        x1=x
                                        y1=PAD_TOP - 4.0
                                        x2=x
                                        y2=chart_height - PAD_BOTTOM
                                        stroke="var(--line-color)"
                                        stroke-width="1"
                                        stroke-opacity="0.35"
                                    />
                                    <text
                                        x=x
                                        y=chart_height - PAD_BOTTOM + 16.0
                                        text-anchor="middle"
                                        class="tt-chart-x-label"
                                    >
                                        {format_percentage_points(tick)}
                                    </text>
                                </g>
                            }
                        }).collect_view()}

                        {rows.iter().enumerate().map(|(index, row)| {
                            let y = PAD_TOP + row_height * index as f64 + row_height * 0.5;
                            let bar_width = (row.mean / max_value.max(0.0001)) * plot_width;
                            let median_x = PAD_LEFT + (row.median / max_value.max(0.0001)) * plot_width;
                            view! {
                                <g class="tt-growth-row">
                                    <title>{format!(
                                        "{label}: {mean} mean, {median} median ({countries} countries)",
                                        label = row.label,
                                        mean = format_percentage_points(row.mean),
                                        median = format_percentage_points(row.median),
                                        countries = row.countries,
                                    )}</title>
                                    <text
                                        x=PAD_LEFT - 10.0
                                        y=y
                                        text-anchor="end"
                                        dominant-baseline="middle"
                                        class="tt-growth-row-label"
                                    >
                                        {row.label.clone()}
                                    </text>
                                    <rect
                                        x=PAD_LEFT
                                        y=y - bar_height / 2.0
                                        width=bar_width.max(1.0)
                                        height=bar_height
                                        fill=row.color
                                        fill-opacity="0.88"
                                        rx=3.0
                                    />
                                    <rect
                                        x=PAD_LEFT
                                        y=y - bar_height / 2.0
                                        width=bar_width.max(1.0)
                                        height=bar_height
                                        fill="none"
                                        stroke=row.color
                                        stroke-width="1"
                                        stroke-opacity="0.35"
                                        rx=3.0
                                    />
                                    <polygon
                                        points=format!(
                                            "{mx},{my} {mx2},{my2} {mx3},{my3}",
                                            mx = median_x,
                                            my = y - 5.0,
                                            mx2 = median_x - 4.5,
                                            my2 = y + 5.0,
                                            mx3 = median_x + 4.5,
                                            my3 = y + 5.0,
                                        )
                                        fill="var(--text-color)"
                                        fill-opacity="0.85"
                                    />
                                    <text
                                        x=PAD_LEFT + bar_width + 8.0
                                        y=y
                                        dominant-baseline="middle"
                                        class="tt-chart-value-label"
                                    >
                                        {format!("{} · n={}", format_percentage_points(row.mean), row.countries)}
                                    </text>
                                </g>
                            }
                        }).collect_view()}

                        <text
                            x=PAD_LEFT + plot_width / 2.0
                            y=chart_height - 8.0
                            text-anchor="middle"
                            class="tt-chart-axis-label"
                        >
                            "Percentage points per year"
                        </text>
                    </svg>
                </div>

                {(!leaders.is_empty()).then(|| view! {
                    <div class="tt-growth-leaders-section">
                        <div class="tt-growth-leaders-header">
                            <h4 class="tt-growth-leaders-title">"Country leaders"</h4>
                            <p class="tt-growth-leaders-note">
                                "Steepest slopes in this window for the two fastest-moving technologies."
                            </p>
                        </div>
                        <div class="tt-growth-leaders-grid">
                            {leaders.into_iter().map(|group| view! {
                                <div class="tt-growth-leader-block">
                                    <h5 class="tt-growth-leader-tech">
                                        <span
                                            class="tt-growth-leader-swatch"
                                            style=format!("background: {}", group.color)
                                        ></span>
                                        {group.technology}
                                    </h5>
                                    {if group.leaders.is_empty() {
                                        view! {
                                            <p class="tt-growth-leader-empty">"No qualifying countries."</p>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <ol class="tt-growth-leader-list">
                                                {group.leaders.into_iter().enumerate().map(|(index, (country, slope))| view! {
                                                    <li>
                                                        <span class="tt-growth-leader-rank">{index + 1}</span>
                                                        <span class="tt-growth-leader-country">{country}</span>
                                                        <span class="tt-growth-leader-slope">{format_percentage_points(slope)}</span>
                                                    </li>
                                                }).collect_view()}
                                            </ol>
                                        }.into_view()
                                    }}
                                </div>
                            }).collect_view()}
                        </div>
                    </div>
                })}

            {(!insights.is_empty()).then(|| view! {
                <div class="tt-median-findings">
                    <h4 class="tt-median-findings-title">"Key findings"</h4>
                    <ul class="tt-median-insights">
                        {insights.into_iter().map(|line| view! {
                            <li>{line}</li>
                        }).collect_view()}
                    </ul>
                </div>
            })}
        </figure>
    }
    .into_view()
}

#[derive(Clone)]
pub struct DumbbellRow {
    pub country: String,
    pub start: f64,
    pub end: f64,
}

#[component]
pub fn DumbbellChart(
    title: String,
    subtitle: Option<String>,
    x_label: &'static str,
    rows: Vec<DumbbellRow>,
    use_percent: bool,
) -> impl IntoView {
    let width = 920.0;
    let row_height = 30.0;
    let height = PAD_TOP + PAD_BOTTOM + row_height * rows.len().max(1) as f64;
    let plot_width = width - PAD_LEFT - PAD_RIGHT;
    let title_for_label = title.clone();

    if rows.is_empty() {
        return view! {
            <p class="tt-chart-empty">"No comparable start/end values for this metric."</p>
        }.into_view();
    }

    let max_value = nice_max(
        rows.iter()
            .flat_map(|row| [row.start, row.end])
            .fold(0.0, f64::max),
    );

    let format_val = |value: f64| {
        if use_percent {
            format!("{value:.1}%")
        } else if value >= 1000.0 {
            format!("{:.0}", value)
        } else {
            format!("{value:.2}")
        }
    };

    view! {
        <figure class="tt-chart">
            <figcaption class="tt-chart-title">{title}</figcaption>
            {subtitle.map(|text| view! {
                <p class="tt-chart-subtitle">{text}</p>
            })}
            <svg class="tt-chart-svg" viewBox=format!("0 0 {width} {height}") role="img" aria-label=title_for_label>
                <text x=PAD_LEFT + plot_width / 2.0 y=height - 8.0 text-anchor="middle" class="tt-chart-axis-label">
                    {x_label}
                </text>
                {rows.into_iter().enumerate().map(|(index, row)| {
                    let y = PAD_TOP + row_height * index as f64 + row_height / 2.0;
                    let x_start = PAD_LEFT + (row.start / max_value.max(0.001)) * plot_width;
                    let x_end = PAD_LEFT + (row.end / max_value.max(0.001)) * plot_width;
                    let label = if row.country.len() > 16 {
                        format!("{}…", &row.country[..15])
                    } else {
                        row.country.clone()
                    };
                    let delta = row.end - row.start;
                    view! {
                        <g class="tt-dumbbell-row">
                            <title>{format!(
                                "{}: {} → {} ({:+.1})",
                                row.country,
                                format_val(row.start),
                                format_val(row.end),
                                delta
                            )}</title>
                            <text x=PAD_LEFT - 8.0 y=y text-anchor="end" dominant-baseline="middle" class="tt-chart-y-label">
                                {label}
                            </text>
                            <line x1=x_start y1=y x2=x_end y2=y stroke="var(--line-color)" stroke-width="3" />
                            <circle cx=x_start cy=y r=5.0 fill="var(--panel-surface)" stroke="#56b4e9" stroke-width="2" />
                            <circle cx=x_end cy=y r=5.0 fill="#a855f7" stroke="#a855f7" stroke-width="2" />
                            <text x=x_end + 8.0 y=y + 4.0 class="tt-chart-value-label">
                                {format!("{:+.1}", delta)}
                            </text>
                        </g>
                    }
                }).collect_view()}
            </svg>
        </figure>
    }
    .into_view()
}

#[derive(Clone)]
pub struct BumpPointData {
    pub year: i32,
    pub rank: usize,
    pub value: f64,
}

#[derive(Clone)]
pub struct BumpSeriesData {
    pub label: String,
    pub color: &'static str,
    pub points: Vec<BumpPointData>,
    pub highlight: bool,
}

#[derive(Clone)]
struct BumpHover {
    label: String,
    year: i32,
    rank: usize,
    value: f64,
    x: f64,
    y: f64,
    color: &'static str,
}

fn format_terajoules(value: f64) -> String {
    if value >= 1000.0 {
        format!("{:.0} TJ", value)
    } else {
        format!("{value:.0} TJ")
    }
}

fn bump_curve_path(points: &[(f64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }
    if points.len() == 1 {
        let (x, y) = points[0];
        return format!("M {x} {y}");
    }

    let mut path = format!("M {} {}", points[0].0, points[0].1);
    for index in 0..points.len() - 1 {
        let (x0, y0) = points[index];
        let (x1, y1) = points[index + 1];
        let mid_x = (x0 + x1) / 2.0;
        path.push_str(&format!(" C {mid_x} {y0}, {mid_x} {y1}, {x1} {y1}"));
    }
    path
}

#[component]
pub fn BumpChart(
    title: String,
    subtitle: Option<String>,
    series: Vec<BumpSeriesData>,
    snapshot_years: Vec<i32>,
    max_rank: usize,
    top_n: usize,
) -> impl IntoView {
    let label_width = 36.0;
    let plot_width = 720.0;
    let right_pad = 24.0;
    let width = label_width + plot_width + right_pad;
    let row_height = 32.0;
    let top_pad = 52.0;
    let bottom_pad = 48.0;
    let rank_count = max_rank.max(top_n);
    let height = top_pad + row_height * rank_count as f64 + bottom_pad;
    let title_for_label = title.clone();
    let hover = create_rw_signal(None::<BumpHover>);
    let top_band_bottom = top_pad + row_height * top_n as f64;

    if series.is_empty() || snapshot_years.len() < 2 {
        return view! {
            <p class="tt-chart-empty">"Not enough ranking snapshots for a bump chart."</p>
        }.into_view();
    }

    let year_start = *snapshot_years.first().unwrap_or(&1990);
    let year_end = *snapshot_years.last().unwrap_or(&2010);
    let year_span = (year_end - year_start).max(1) as f64;

    let x_for_year = move |year: i32| {
        label_width + ((year - year_start) as f64 / year_span) * plot_width
    };

    let y_for_rank = move |rank: usize| top_pad + (rank - 1) as f64 * row_height + row_height / 2.0;

    view! {
        <figure class="tt-chart tt-bump-chart">
            <figcaption class="tt-chart-title">{title}</figcaption>
            {subtitle.map(|text| view! {
                <p class="tt-chart-subtitle">{text}</p>
            })}
            <div class="tt-line-legend">
                {series.iter().map(|entry| view! {
                    <span
                        class="tt-legend-item"
                        class:tt-legend-muted=!entry.highlight
                    >
                        <span class="tt-legend-swatch" style=format!("background: {}", entry.color)></span>
                        {format!(
                            "{}{}",
                            entry.label,
                            entry.points.last().map(|point| format!(" (#{} in {})", point.rank, point.year)).unwrap_or_default()
                        )}
                    </span>
                }).collect_view()}
            </div>
            <div class="tt-bump-scroll">
            <svg class="tt-chart-svg tt-bump-svg" viewBox=format!("0 0 {width} {height}") role="img" aria-label=title_for_label>
                <rect
                    x=label_width
                    y=top_pad
                    width=plot_width
                    height=top_band_bottom - top_pad
                    fill="var(--purple)"
                    fill-opacity="0.06"
                />

                {(1..=rank_count).map(|rank| {
                    let y = y_for_rank(rank);
                    let in_top = rank <= top_n;
                    view! {
                        <g class="tt-bump-rank">
                            <line
                                x1=label_width
                                y1=y
                                x2=label_width + plot_width
                                y2=y
                                stroke=if in_top { "var(--line-color)" } else { "var(--line-color)" }
                                stroke-width=if in_top { "1" } else { "0.5" }
                                stroke-dasharray=if in_top { "3 4" } else { "2 6" }
                                stroke-opacity=if in_top { "1" } else { "0.45" }
                            />
                            <text x=label_width - 8.0 y=y + 4.0 text-anchor="end" class="tt-chart-y-label">
                                {rank.to_string()}
                            </text>
                        </g>
                    }
                }).collect_view()}

                {snapshot_years.iter().map(|year| {
                    let x = x_for_year(*year);
                    view! {
                        <g class="tt-bump-year">
                            <line
                                x1=x
                                y1=top_pad
                                x2=x
                                y2=top_pad + row_height * rank_count as f64
                                stroke="var(--line-color)"
                                stroke-width="1"
                                stroke-opacity="0.35"
                            />
                            <text x=x y=24.0 text-anchor="middle" class="tt-chart-x-label">{year.to_string()}</text>
                        </g>
                    }
                }).collect_view()}

                <text x=label_width + 6.0 y=top_pad + 12.0 class="tt-bump-band-label">
                    {format!("Top {top_n}")}
                </text>

                {series.iter().map(|entry| {
                    let coords: Vec<(f64, f64)> = entry
                        .points
                        .iter()
                        .map(|point| (x_for_year(point.year), y_for_rank(point.rank)))
                        .collect();
                    let path = bump_curve_path(&coords);
                    let stroke_width = if entry.highlight { 3.0 } else { 1.75 };
                    let opacity = if entry.highlight { 1.0 } else { 0.55 };

                    view! {
                        <g
                            class="tt-bump-series"
                            class:tt-bump-series-muted=!entry.highlight
                        >
                            <path
                                d=path
                                fill="none"
                                stroke=entry.color
                                stroke-width=stroke_width
                                stroke-opacity=opacity
                                stroke-linecap="round"
                            />
                            {entry.points.iter().map(|point| {
                                let x = x_for_year(point.year);
                                let y = y_for_rank(point.rank);
                                let label = entry.label.clone();
                                let color = entry.color;
                                let hover_point = BumpHover {
                                    label: label.clone(),
                                    year: point.year,
                                    rank: point.rank,
                                    value: point.value,
                                    x,
                                    y,
                                    color,
                                };
                                view! {
                                    <g class="tt-bump-point">
                                        <circle
                                            cx=x
                                            cy=y
                                            r=12.0
                                            fill="transparent"
                                            class="tt-bump-hit"
                                            on:mouseenter=move |_| hover.set(Some(hover_point.clone()))
                                            on:mouseleave=move |_| hover.set(None)
                                        />
                                        <circle
                                            cx=x
                                            cy=y
                                            r=if entry.highlight { 6.0 } else { 4.5 }
                                            fill="var(--panel-surface)"
                                            stroke=entry.color
                                            stroke-width=if entry.highlight { 2.5 } else { 1.5 }
                                            stroke-opacity=opacity
                                        />
                                        <text
                                            x=x
                                            y=y + 3.5
                                            text-anchor="middle"
                                            class="tt-bump-rank-label"
                                            fill=entry.color
                                            fill-opacity=opacity
                                        >
                                            {point.rank.to_string()}
                                        </text>
                                    </g>
                                }
                            }).collect_view()}
                        </g>
                    }
                }).collect_view()}

                {move || hover.get().map(|point| {
                    let text = format!(
                        "{} · {} · #{} · {}",
                        point.label,
                        point.year,
                        point.rank,
                        format_terajoules(point.value)
                    );
                    let tooltip_width = (text.len() as f64 * 6.0).clamp(140.0, 300.0);
                    let half_width = tooltip_width / 2.0;
                    let tooltip_x = point.x.clamp(label_width + half_width, label_width + plot_width - half_width);
                    let tooltip_y = (point.y - 16.0).max(top_pad + 8.0);
                    view! {
                        <g class="tt-line-tooltip" transform=format!("translate({tooltip_x},{tooltip_y})")>
                            <rect
                                x=-half_width
                                y=-22.0
                                width=tooltip_width
                                height=20.0
                                rx=3.0
                                fill="var(--panel-surface)"
                                stroke=point.color
                                stroke-width="1.5"
                            />
                            <text
                                text-anchor="middle"
                                y=-8.0
                                class="tt-line-tooltip-text"
                                fill="var(--text-color)"
                            >
                                {text}
                            </text>
                        </g>
                    }
                })}
            </svg>
            </div>
        </figure>
    }
    .into_view()
}
