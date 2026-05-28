use crate::tidy_tuesday::{metric_value, EnergyRecord};

const AGGREGATE_REGIONS: &[&str] = &[
    "World",
    "Europe",
    "Latin America and Caribbean",
    "Northern Africa",
    "Sub-Saharan Africa",
    "Middle income",
    "Low & middle income",
    "Low income",
    "Lower middle income",
    "Upper middle income",
    "High income",
    "High income: OECD",
    "High income: nonOECD",
    "Eastern Europe",
    "Western Asia",
    "Southern Asia",
    "South Eastern Asia",
    "Caucasus and Central Asia",
    "Eastern Asia (including Japan)",
    "Eastern Asia (not including Japan)",
    "Oceania",
    "Oceania (not including Australia and New Zealand)",
    "Nothern America",
];

pub fn is_country(name: &str) -> bool {
    !AGGREGATE_REGIONS.contains(&name)
}

#[derive(Clone, Debug)]
pub struct StatCard {
    pub label: String,
    pub value: String,
    pub detail: String,
}

pub fn median_at_year<F>(records: &[EnergyRecord], year: i32, value_fn: F) -> Option<f64>
where
    F: Fn(&EnergyRecord) -> Option<f64>,
{
    let mut values: Vec<f64> = records
        .iter()
        .filter(|record| record.y == year && is_country(&record.c))
        .filter_map(|record| value_fn(record))
        .collect();

    if values.is_empty() {
        return None;
    }

    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = values.len() / 2;
    Some(if values.len() % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[mid]
    })
}

pub fn count_with_positive(
    records: &[EnergyRecord],
    year: i32,
    metric_key: &str,
) -> usize {
    records
        .iter()
        .filter(|record| record.y == year && is_country(&record.c))
        .filter(|record| metric_value(record, metric_key).is_some_and(|value| value > 0.0))
        .count()
}

pub fn country_slope(
    records: &[EnergyRecord],
    country: &str,
    metric_key: &str,
    year_start: i32,
    year_end: i32,
) -> Option<f64> {
    let mut points: Vec<(i32, f64)> = records
        .iter()
        .filter(|record| {
            record.c == country && record.y >= year_start && record.y <= year_end
        })
        .filter_map(|record| metric_value(record, metric_key).map(|value| (record.y, value)))
        .collect();

    if points.len() < 5 {
        return None;
    }

    points.sort_by_key(|(year, _)| *year);
    let (x0, y0) = points[0];
    let (x1, y1) = points[points.len() - 1];
    if x1 == x0 {
        return None;
    }

    Some((y1 - y0) / (x1 - x0) as f64)
}

pub fn fastest_growing_country(
    records: &[EnergyRecord],
    metric_key: &str,
    year_start: i32,
    year_end: i32,
) -> Option<(String, f64)> {
    let mut best: Option<(String, f64)> = None;

    for country in country_names(records) {
        if let Some(slope) = country_slope(records, &country, metric_key, year_start, year_end) {
            if best.as_ref().is_none_or(|(_, current)| slope > *current) {
                best = Some((country, slope));
            }
        }
    }

    best
}

pub fn compute_stat_cards(
    records: &[EnergyRecord],
    min_year: i32,
    max_year: i32,
) -> Vec<StatCard> {
    let mut cards = Vec::new();
    let mid_start = 2000.max(min_year);

    if let (Some(start), Some(end)) = (
        median_at_year(records, min_year, |record| record.elec_pct),
        median_at_year(records, max_year, |record| record.elec_pct),
    ) {
        let delta = end - start;
        cards.push(StatCard {
            label: "Median electricity access".to_string(),
            value: format!("{end:.0}%"),
            detail: format!(
                "Up from {start:.0}% in {min_year} (+{delta:.0} pp across countries)"
            ),
        });
    }

    let solar_start = count_with_positive(records, mid_start, "solar_tfec");
    let solar_end = count_with_positive(records, max_year, "solar_tfec");
    cards.push(StatCard {
        label: "Countries with solar TFEC".to_string(),
        value: solar_end.to_string(),
        detail: format!(
            "Reporting solar share > 0 in {max_year}, vs {solar_start} in {mid_start}"
        ),
    });

    if let Some((country, slope)) =
        fastest_growing_country(records, "wind_tfec", mid_start, max_year)
    {
        cards.push(StatCard {
            label: "Fastest wind adoption".to_string(),
            value: format!("{slope:.3} pp/yr"),
            detail: format!("{country}, {mid_start}–{max_year}"),
        });
    }

    if let (Some(low), Some(high)) = (
        records
            .iter()
            .filter(|record| record.y == max_year && is_country(&record.c))
            .filter_map(|record| record.renew_tfec)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
        records
            .iter()
            .filter(|record| record.y == max_year && is_country(&record.c))
            .filter_map(|record| record.renew_tfec)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
    ) {
        cards.push(StatCard {
            label: "Renewable TFEC spread".to_string(),
            value: format!("{high:.0}%"),
            detail: format!("Top country in {max_year}; bottom reports {low:.1}%"),
        });
    }

    if let Some((country, value)) = records
        .iter()
        .filter(|record| record.y == max_year && is_country(&record.c))
        .filter_map(|record| record.solar_tj.map(|value| (record.c.clone(), value)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    {
        cards.push(StatCard {
            label: "Largest solar consumer".to_string(),
            value: format!("{value:.0} TJ"),
            detail: format!("{country} in {max_year}"),
        });
    }

    if let (Some(start), Some(end)) = (
        median_at_year(records, min_year, |record| record.renew_tfec),
        median_at_year(records, max_year, |record| record.renew_tfec),
    ) {
        cards.push(StatCard {
            label: "Median renewable share".to_string(),
            value: format!("{end:.1}%"),
            detail: format!("Median TFEC from renewables: {start:.1}% → {end:.1}%"),
        });
    }

    cards
}

pub fn country_names(records: &[EnergyRecord]) -> Vec<String> {
    let mut names: Vec<String> = records
        .iter()
        .filter(|record| is_country(&record.c))
        .map(|record| record.c.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    names.sort();
    names
}

pub struct MixLayer {
    pub label: &'static str,
    pub key: &'static str,
    pub color: &'static str,
}

pub const RENEWABLE_MIX: [MixLayer; 5] = [
    MixLayer {
        label: "Hydro",
        key: "hydro_tfec",
        color: "#009e73",
    },
    MixLayer {
        label: "Biomass",
        key: "biomass_tfec",
        color: "#a855f7",
    },
    MixLayer {
        label: "Wind",
        key: "wind_tfec",
        color: "#56b4e9",
    },
    MixLayer {
        label: "Solar",
        key: "solar_tfec",
        color: "#f0e442",
    },
    MixLayer {
        label: "Geothermal",
        key: "geothermal_tfec",
        color: "#d55e00",
    },
];

pub fn renewable_mix_series(
    records: &[EnergyRecord],
    country: &str,
    year_start: i32,
    year_end: i32,
) -> Vec<(i32, Vec<f64>)> {
    (year_start..=year_end)
        .map(|year| {
            let values = RENEWABLE_MIX
                .iter()
                .map(|layer| {
                    records
                        .iter()
                        .find(|record| record.c == country && record.y == year)
                        .and_then(|record| metric_value(record, layer.key))
                        .unwrap_or(0.0)
                })
                .collect();
            (year, values)
        })
        .collect()
}

#[derive(Clone)]
pub struct ScatterPoint {
    pub label: String,
    pub x: f64,
    pub y: f64,
}

pub fn access_renewables_scatter(
    records: &[EnergyRecord],
    year: i32,
) -> Vec<ScatterPoint> {
    records
        .iter()
        .filter(|record| record.y == year && is_country(&record.c))
        .filter_map(|record| {
            let x = record.elec_pct?;
            let y = record.renew_tfec?;
            Some(ScatterPoint {
                label: record.c.clone(),
                x,
                y,
            })
        })
        .collect()
}

pub fn top_countries_by_metric(
    records: &[EnergyRecord],
    year: i32,
    metric_key: &str,
    count: usize,
) -> Vec<String> {
    let mut rows: Vec<(String, f64)> = records
        .iter()
        .filter(|record| record.y == year && is_country(&record.c))
        .filter_map(|record| {
            metric_value(record, metric_key).map(|value| (record.c.clone(), value))
        })
        .collect();

    rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    rows.truncate(count);
    rows.into_iter().map(|(country, _)| country).collect()
}

pub fn heatmap_values(
    records: &[EnergyRecord],
    countries: &[String],
    metric_key: &str,
    year_start: i32,
    year_end: i32,
) -> Vec<Vec<Option<f64>>> {
    countries
        .iter()
        .map(|country| {
            (year_start..=year_end)
                .map(|year| {
                    records
                        .iter()
                        .find(|record| record.c == *country && record.y == year)
                        .and_then(|record| metric_value(record, metric_key))
                })
                .collect()
        })
        .collect()
}

pub fn histogram_bins(
    records: &[EnergyRecord],
    year: i32,
    metric_key: &str,
    bin_count: usize,
) -> Vec<(f64, f64, usize)> {
    let values: Vec<f64> = records
        .iter()
        .filter(|record| record.y == year && is_country(&record.c))
        .filter_map(|record| metric_value(record, metric_key))
        .filter(|value| *value > 0.0)
        .collect();

    if values.is_empty() {
        return Vec::new();
    }

    let max_value = values.iter().copied().fold(0.0, f64::max);
    let bin_width = (max_value / bin_count as f64).max(0.001);
    let mut bins = vec![0_usize; bin_count];

    for value in values {
        let index = ((value / bin_width).floor() as usize).min(bin_count - 1);
        bins[index] += 1;
    }

    (0..bin_count)
        .map(|index| {
            let start = index as f64 * bin_width;
            let end = start + bin_width;
            (start, end, bins[index])
        })
        .filter(|(_, _, count)| *count > 0)
        .collect()
}

#[derive(Clone, Debug)]
pub struct InsightCard {
    pub title: String,
    pub highlight: String,
    pub body: String,
    pub accent: &'static str,
}

pub fn compute_era_insights(
    records: &[EnergyRecord],
    min_year: i32,
    max_year: i32,
) -> Vec<InsightCard> {
    let mut insights = Vec::new();
    let mid_start = 2000.max(min_year);

    let solar_early = count_with_positive(records, min_year, "solar_tfec");
    let solar_late = count_with_positive(records, max_year, "solar_tfec");
    if solar_late > solar_early {
        insights.push(InsightCard {
            title: "Solar went from niche to normal".to_string(),
            highlight: format!("{solar_early} → {solar_late} countries"),
            body: format!(
                "In {min_year}, only {solar_early} countries reported any solar TFEC share. By {max_year} that more than doubled. Solar stopped being a rounding error in the data."
            ),
            accent: "#f0e442",
        });
    }

    if let Some((country, start, end, delta)) =
        biggest_delta(records, min_year, max_year, |record| record.elec_pct)
    {
        insights.push(InsightCard {
            title: "The biggest grid expansion".to_string(),
            highlight: format!("+{delta:.0} pp access"),
            body: format!(
                "{country} moved from {start:.0}% to {end:.0}% electricity access between {min_year} and {max_year}, one of the clearest access success stories in the dataset."
            ),
            accent: "#56b4e9",
        });
    }

    if let Some((country, slope)) =
        fastest_growing_country(records, "wind_tfec", mid_start, max_year)
    {
        let runners = top_slopes(records, "wind_tfec", mid_start, max_year, 3);
        let also = runners
            .iter()
            .filter(|(name, _)| name != &country)
            .map(|(name, s)| format!("{name} ({s:.2} pp/yr)"))
            .collect::<Vec<_>>()
            .join(", ");
        insights.push(InsightCard {
            title: "Europe's wind sprint".to_string(),
            highlight: format!("{country} +{slope:.2} pp/yr"),
            body: format!(
                "From {mid_start}–{max_year}, wind TFEC share grew fastest in {country}. Also accelerating: {also}. Portugal and Spain led in absolute slope, the pre-offshore boom era."
            ),
            accent: "#009e73",
        });
    }

    let grid_rich_clean_poor = records
        .iter()
        .filter(|record| record.y == max_year && is_country(&record.c))
        .filter(|record| {
            record.elec_pct.is_some_and(|access| access > 90.0)
                && record.renew_tfec.is_some_and(|renew| renew < 5.0)
        })
        .count();
    insights.push(InsightCard {
        title: "Grid access ≠ green energy".to_string(),
        highlight: format!("{grid_rich_clean_poor} countries"),
        body: format!(
            "In {max_year}, {grid_rich_clean_poor} countries had >90% electricity access but <5% renewable TFEC (Turkmenistan, Malta, Hong Kong, Algeria among them). Universal access and clean share were decoupled."
        ),
        accent: "#d55e00",
    });

    if let Some((country, start, end, delta)) =
        biggest_delta(records, mid_start, max_year, |record| record.solar_tj)
    {
        insights.push(InsightCard {
            title: "Solar scaled in terajoules".to_string(),
            highlight: format!("+{delta:.0} TJ"),
            body: format!(
                "{country} added the most absolute solar consumption ({start:.0} → {end:.0} TJ) from {mid_start}–{max_year}. China and Germany dominate total solar by {max_year}, but the growth story is concentrated in a handful of large economies."
            ),
            accent: "#a855f7",
        });
    }

    insights.push(InsightCard {
        title: "Data stops where SE4ALL starts".to_string(),
        highlight: "1990–2010 only".to_string(),
        body: "The UN launched SE4ALL in 2010 with 2030 targets. This snapshot captures the baseline decade before, useful for seeing what the world looked like going in, not progress since.".to_string(),
        accent: "#0072b2",
    });

    insights
}

fn biggest_delta<F>(
    records: &[EnergyRecord],
    year_start: i32,
    year_end: i32,
    value_fn: F,
) -> Option<(String, f64, f64, f64)>
where
    F: Fn(&EnergyRecord) -> Option<f64>,
{
    let mut best: Option<(String, f64, f64, f64)> = None;

    for country in country_names(records) {
        let start = records
            .iter()
            .find(|record| record.c == country && record.y == year_start)
            .and_then(&value_fn)?;
        let end = records
            .iter()
            .find(|record| record.c == country && record.y == year_end)
            .and_then(&value_fn)?;
        let delta = end - start;
        if best.as_ref().is_none_or(|(_, _, _, current)| delta > *current) {
            best = Some((country, start, end, delta));
        }
    }

    best
}

fn top_slopes(
    records: &[EnergyRecord],
    metric_key: &str,
    year_start: i32,
    year_end: i32,
    count: usize,
) -> Vec<(String, f64)> {
    let mut slopes: Vec<(String, f64)> = country_names(records)
        .into_iter()
        .filter_map(|country| {
            country_slope(records, &country, metric_key, year_start, year_end)
                .map(|slope| (country, slope))
        })
        .collect();
    slopes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    slopes.truncate(count);
    slopes
}

pub fn top_countries_by_slope(
    records: &[EnergyRecord],
    metric_key: &str,
    year_start: i32,
    year_end: i32,
    count: usize,
) -> Vec<(String, f64)> {
    top_slopes(records, metric_key, year_start, year_end, count)
}

pub fn collect_country_slopes(
    records: &[EnergyRecord],
    metric_key: &str,
    year_start: i32,
    year_end: i32,
) -> Vec<f64> {
    country_names(records)
        .into_iter()
        .filter_map(|country| country_slope(records, &country, metric_key, year_start, year_end))
        .collect()
}

#[derive(Clone, Debug)]
pub struct TechnologyGrowthRow {
    pub key: &'static str,
    pub label: &'static str,
    pub color: &'static str,
    pub mean_slope: f64,
    pub median_slope: f64,
    pub country_count: usize,
    pub top_country: String,
    pub top_slope: f64,
}

pub fn renewable_growth_data(
    records: &[EnergyRecord],
    year_start: i32,
    year_end: i32,
) -> Vec<TechnologyGrowthRow> {
    let mut rows: Vec<TechnologyGrowthRow> = RENEWABLE_MIX
        .iter()
        .map(|layer| {
            let mut slopes = collect_country_slopes(records, layer.key, year_start, year_end);
            let country_count = slopes.len();
            let mean_slope = if slopes.is_empty() {
                0.0
            } else {
                slopes.iter().sum::<f64>() / slopes.len() as f64
            };

            slopes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median_slope = if slopes.is_empty() {
                0.0
            } else {
                percentile(&slopes, 0.5)
            };

            let top = top_slopes(records, layer.key, year_start, year_end, 1);
            let (top_country, top_slope) = top
                .first()
                .cloned()
                .unwrap_or_else(|| ("—".to_string(), 0.0));

            TechnologyGrowthRow {
                key: layer.key,
                label: layer.label,
                color: layer.color,
                mean_slope,
                median_slope,
                country_count,
                top_country,
                top_slope,
            }
        })
        .collect();

    rows.sort_by(|a, b| {
        b.mean_slope
            .partial_cmp(&a.mean_slope)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    rows
}

#[derive(Clone, Debug)]
pub struct GrowthLeaderGroup {
    pub technology: String,
    pub color: &'static str,
    pub leaders: Vec<(String, f64)>,
}

pub fn renewable_growth_leaders(
    records: &[EnergyRecord],
    rows: &[TechnologyGrowthRow],
    year_start: i32,
    year_end: i32,
    technology_count: usize,
    leaders_per_technology: usize,
) -> Vec<GrowthLeaderGroup> {
    rows.iter()
        .take(technology_count)
        .map(|row| GrowthLeaderGroup {
            technology: row.label.to_string(),
            color: row.color,
            leaders: top_countries_by_slope(
                records,
                row.key,
                year_start,
                year_end,
                leaders_per_technology,
            ),
        })
        .collect()
}

pub fn renewable_growth_stat_cards(
    rows: &[TechnologyGrowthRow],
    year_start: i32,
    year_end: i32,
) -> Vec<StatCard> {
    let Some(fastest) = rows.first() else {
        return Vec::new();
    };

    let solar = rows.iter().find(|row| row.key == "solar_tfec");
    let wind = rows.iter().find(|row| row.key == "wind_tfec");
    let hydro = rows.iter().find(|row| row.key == "hydro_tfec");

    let mut cards = vec![
        StatCard {
            label: "Fastest technology".to_string(),
            value: fastest.label.to_string(),
            detail: format!(
                "{slope:.3} pp/yr mean slope, {year_start}–{year_end}",
                slope = fastest.mean_slope,
                year_start = year_start,
                year_end = year_end
            ),
        },
        StatCard {
            label: "Country leader".to_string(),
            value: fastest.top_country.clone(),
            detail: format!(
                "{slope:.3} pp/yr on {tech}",
                slope = fastest.top_slope,
                tech = fastest.label
            ),
        },
    ];

    if let (Some(solar), Some(wind)) = (solar, wind) {
        cards.push(StatCard {
            label: "Solar vs wind".to_string(),
            value: format!("{:.3} vs {:.3}", solar.mean_slope, wind.mean_slope),
            detail: "Mean pp/yr across reporting countries".to_string(),
        });
    }

    if let Some(hydro) = hydro {
        let sample = rows
            .iter()
            .map(|row| row.country_count)
            .max()
            .unwrap_or(0);
        cards.push(StatCard {
            label: "Hydro baseline".to_string(),
            value: format!("{:.3} pp/yr", hydro.mean_slope),
            detail: format!(
                "Mature tech for comparison · up to {sample} countries in sample"
            ),
        });
    }

    cards
}

pub fn renewable_growth_insights(
    rows: &[TechnologyGrowthRow],
    year_start: i32,
    year_end: i32,
) -> Vec<String> {
    let mut insights = Vec::new();
    let Some(fastest) = rows.first() else {
        return insights;
    };

    if fastest.country_count == 0 {
        insights.push(format!(
            "Not enough country histories with 5+ reporting years between {year_start} and {year_end}. Widen the year range to compare technologies."
        ));
        return insights;
    }

    insights.push(format!(
        "{tech} led with a mean slope of {mean:.3} percentage points per year ({start}–{end}), based on {n} country trajectories.",
        tech = fastest.label,
        mean = fastest.mean_slope,
        start = year_start,
        end = year_end,
        n = fastest.country_count
    ));

    if fastest.top_country != "—" {
        insights.push(format!(
            "{country} posted the steepest {tech} trajectory at {slope:.3} pp/yr, well above the cross-country mean.",
            country = fastest.top_country,
            tech = fastest.label,
            slope = fastest.top_slope
        ));
    }

    if let (Some(solar), Some(wind)) = (
        rows.iter().find(|row| row.key == "solar_tfec"),
        rows.iter().find(|row| row.key == "wind_tfec"),
    ) {
        if wind.mean_slope > solar.mean_slope {
            insights.push(format!(
                "Wind outpaced solar on average ({:.3} vs {:.3} pp/yr). Onshore wind scaled across more markets before solar TFEC reporting became widespread.",
                wind.mean_slope,
                solar.mean_slope
            ));
        } else {
            insights.push(format!(
                "Solar outpaced wind on average ({:.3} vs {:.3} pp/yr) in this window, though both remain small shares of total TFEC.",
                solar.mean_slope,
                wind.mean_slope
            ));
        }
    }

    if let Some(hydro) = rows.iter().find(|row| row.key == "hydro_tfec") {
        if fastest.mean_slope > hydro.mean_slope * 3.0 && hydro.mean_slope.abs() < 0.05 {
            insights.push(format!(
                "Hydro moved slowly ({:.3} pp/yr mean), underscoring that the growth story is in newer technologies rather than legacy renewables.",
                hydro.mean_slope
            ));
        }
    }

    for row in rows.iter().take(3) {
        if row.country_count >= 5
            && (row.mean_slope - row.median_slope).abs() > row.mean_slope.abs() * 0.35
            && row.mean_slope > 0.0
        {
            insights.push(format!(
                "{tech} mean ({mean:.3} pp/yr) runs ahead of its median ({median:.3}), suggesting a few fast movers pull the average up.",
                tech = row.label,
                mean = row.mean_slope,
                median = row.median_slope
            ));
            break;
        }
    }

    insights
}

#[derive(Clone, Debug)]
pub struct YearDistribution {
    pub year: i32,
    pub median: f64,
    pub q1: f64,
    pub q3: f64,
    pub count: usize,
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }

    let position = p * (sorted.len() - 1) as f64;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    if lower == upper {
        return sorted[lower];
    }

    let weight = position - lower as f64;
    sorted[lower] * (1.0 - weight) + sorted[upper] * weight
}

pub fn year_distribution<F>(records: &[EnergyRecord], year: i32, value_fn: F) -> Option<YearDistribution>
where
    F: Fn(&EnergyRecord) -> Option<f64>,
{
    let mut values: Vec<f64> = records
        .iter()
        .filter(|record| record.y == year && is_country(&record.c))
        .filter_map(|record| value_fn(record))
        .collect();

    if values.is_empty() {
        return None;
    }

    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    Some(YearDistribution {
        year,
        median: percentile(&values, 0.5),
        q1: percentile(&values, 0.25),
        q3: percentile(&values, 0.75),
        count: values.len(),
    })
}

pub fn distribution_series<F>(
    records: &[EnergyRecord],
    year_start: i32,
    year_end: i32,
    value_fn: F,
) -> Vec<YearDistribution>
where
    F: Fn(&EnergyRecord) -> Option<f64>,
{
    (year_start..=year_end)
        .filter_map(|year| year_distribution(records, year, &value_fn))
        .collect()
}

pub fn adopters_distribution_series(
    records: &[EnergyRecord],
    year_start: i32,
    year_end: i32,
    metric_key: &str,
) -> Vec<YearDistribution> {
    (year_start..=year_end)
        .filter_map(|year| {
            let mut values: Vec<f64> = records
                .iter()
                .filter(|record| record.y == year && is_country(&record.c))
                .filter_map(|record| metric_value(record, metric_key))
                .filter(|value| *value > 0.0)
                .collect();

            if values.is_empty() {
                return None;
            }

            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            Some(YearDistribution {
                year,
                median: percentile(&values, 0.5),
                q1: percentile(&values, 0.25),
                q3: percentile(&values, 0.75),
                count: values.len(),
            })
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct MedianTrajectoryData {
    pub access_snapshots: Vec<YearDistribution>,
    pub renewable: Vec<YearDistribution>,
    pub solar_adopters: Vec<YearDistribution>,
    pub wind_adopters: Vec<YearDistribution>,
}

pub fn median_trajectory_data(
    records: &[EnergyRecord],
    year_start: i32,
    year_end: i32,
) -> MedianTrajectoryData {
    MedianTrajectoryData {
        access_snapshots: distribution_series(records, year_start, year_end, |record| {
            record.elec_pct
        }),
        renewable: distribution_series(records, year_start, year_end, |record| {
            record.renew_tfec
        }),
        solar_adopters: adopters_distribution_series(records, year_start, year_end, "solar_tfec"),
        wind_adopters: adopters_distribution_series(records, year_start, year_end, "wind_tfec"),
    }
}

pub fn median_trajectory_insights(data: &MedianTrajectoryData) -> Vec<String> {
    let mut insights = Vec::new();

    if let (Some(first), Some(last)) = (data.access_snapshots.first(), data.access_snapshots.last())
    {
        if data.access_snapshots.len() <= 4 {
            let years: Vec<String> = data
                .access_snapshots
                .iter()
                .map(|entry| entry.year.to_string())
                .collect();
            insights.push(format!(
                "Electricity access is only reported in {} in this dataset, not every year. Median access rose from {:.0}% ({}) to {:.0}% ({}).",
                years.join(", "),
                first.median,
                first.year,
                last.median,
                last.year
            ));
        } else {
            insights.push(format!(
                "Median electricity access rose from {:.0}% ({}) to {:.0}% ({}).",
                first.median, first.year, last.median, last.year
            ));
        }
    }

    if let (Some(first), Some(last)) = (data.renewable.first(), data.renewable.last()) {
        let delta = last.median - first.median;
        let trend = if delta.abs() < 0.5 {
            format!(" stayed near {:.1}%", last.median)
        } else if delta > 0.0 {
            format!(" rose {:.1} pp to {:.1}%", delta, last.median)
        } else {
            format!(" fell {:.1} pp to {:.1}%", delta.abs(), last.median)
        };
        insights.push(format!(
            "Median renewable TFEC share{trend} ({start}–{end}). The aggregate median masks rapid solar/wind growth among adopters.",
            trend = trend,
            start = first.year,
            end = last.year
        ));
    }

    if let (Some(first), Some(last)) = (data.solar_adopters.first(), data.solar_adopters.last()) {
        insights.push(format!(
            "Among countries reporting solar, the median TFEC share grew {:.3} pp to {:.3}% by {end} ({n} countries).",
            last.median - first.median,
            last.median,
            end = last.year,
            n = last.count
        ));
    }

    if let (Some(first), Some(last)) = (data.wind_adopters.first(), data.wind_adopters.last()) {
        insights.push(format!(
            "Wind tells a sharper story: median adopter share went from {:.3}% to {:.3}% ({start}–{end}, {n} countries).",
            first.median,
            last.median,
            start = first.year,
            end = last.year,
            n = last.count
        ));
    }

    if let (Some(renew), Some(access)) = (data.renewable.last(), data.access_snapshots.last()) {
        if renew.count < access.count {
            insights.push(format!(
                "Coverage matters: {renew_n} countries reported renewable TFEC in {year} vs {access_n} for electricity access.",
                renew_n = renew.count,
                year = renew.year,
                access_n = access.count
            ));
        }
    }

    insights
}

#[derive(Clone, Debug)]
pub struct AdoptionYearPoint {
    pub year: i32,
    pub solar: usize,
    pub wind: usize,
    pub solar_only: usize,
    pub wind_only: usize,
    pub both: usize,
    pub either: usize,
}

pub fn adoption_year_breakdown(records: &[EnergyRecord], year: i32) -> AdoptionYearPoint {
    let mut solar = 0_usize;
    let mut wind = 0_usize;
    let mut both = 0_usize;

    for record in records
        .iter()
        .filter(|record| record.y == year && is_country(&record.c))
    {
        let has_solar = record.solar_tfec.is_some_and(|value| value > 0.0);
        let has_wind = record.wind_tfec.is_some_and(|value| value > 0.0);
        if has_solar {
            solar += 1;
        }
        if has_wind {
            wind += 1;
        }
        if has_solar && has_wind {
            both += 1;
        }
    }

    AdoptionYearPoint {
        year,
        solar,
        wind,
        solar_only: solar - both,
        wind_only: wind - both,
        both,
        either: solar + wind - both,
    }
}

pub fn adoption_wave_data(
    records: &[EnergyRecord],
    year_start: i32,
    year_end: i32,
) -> Vec<AdoptionYearPoint> {
    (year_start..=year_end)
        .map(|year| adoption_year_breakdown(records, year))
        .collect()
}

pub fn adoption_wave_crossover(points: &[AdoptionYearPoint]) -> Option<(i32, String)> {
    points
        .iter()
        .find(|point| point.wind > point.solar)
        .map(|point| (point.year, format!("Wind > solar ({})", point.year)))
}

pub fn adoption_wave_stat_cards(points: &[AdoptionYearPoint]) -> Vec<StatCard> {
    let Some(first) = points.first() else {
        return Vec::new();
    };
    let Some(last) = points.last() else {
        return Vec::new();
    };

    let solar_delta = last.solar as i32 - first.solar as i32;
    let wind_delta = last.wind as i32 - first.wind as i32;
    let both_delta = last.both as i32 - first.both as i32;
    let either_delta = last.either as i32 - first.either as i32;

    vec![
        StatCard {
            label: "Solar reporters".to_string(),
            value: format!("{} → {}", first.solar, last.solar),
            detail: format!(
                "+{solar_delta} countries from {start} to {end}",
                start = first.year,
                end = last.year
            ),
        },
        StatCard {
            label: "Wind reporters".to_string(),
            value: format!("{} → {}", first.wind, last.wind),
            detail: format!(
                "+{wind_delta} countries from {start} to {end}",
                start = first.year,
                end = last.year
            ),
        },
        StatCard {
            label: "Both technologies".to_string(),
            value: format!("{} → {}", first.both, last.both),
            detail: format!(
                "+{both_delta} countries reporting solar and wind",
            ),
        },
        StatCard {
            label: "Either technology".to_string(),
            value: format!("{} → {}", first.either, last.either),
            detail: format!(
                "+{either_delta} unique countries reporting at least one",
            ),
        },
    ]
}

pub fn adoption_wave_insights(points: &[AdoptionYearPoint]) -> Vec<String> {
    let mut insights = Vec::new();
    let Some(first) = points.first() else {
        return insights;
    };
    let Some(last) = points.last() else {
        return insights;
    };

    if first.solar > 0 {
        let multiplier = last.solar as f64 / first.solar as f64;
        insights.push(format!(
            "Solar reporting grew from {start} countries in {start_year} to {end} in {end_year} ({mult:.1}x). Most of the expansion happened after 2000.",
            start = first.solar,
            start_year = first.year,
            end = last.solar,
            end_year = last.year,
            mult = multiplier
        ));
    }

    if first.wind > 0 {
        let multiplier = last.wind as f64 / first.wind as f64;
        insights.push(format!(
            "Wind spread even faster: {start} → {end} countries ({mult:.1}x){tail}.",
            start = first.wind,
            end = last.wind,
            mult = multiplier,
            tail = if last.wind > last.solar {
                format!(", and by {end_year} wind had broader reporting than solar", end_year = last.year)
            } else {
                String::new()
            }
        ));
    } else if last.wind > first.wind {
        insights.push(format!(
            "Wind went from {start} to {end} reporting countries between {start_year} and {end_year}, a technology that barely registered at the start of the series.",
            start = first.wind,
            end = last.wind,
            start_year = first.year,
            end_year = last.year
        ));
    }

    if let Some((year, _)) = adoption_wave_crossover(points) {
        insights.push(format!(
            "Wind overtook solar in country count in {year}, a sign that onshore wind scaled across more economies before solar TFEC reporting became widespread."
        ));
    }

    if last.both > first.both {
        let solo_solar = last.solar_only;
        let solo_wind = last.wind_only;
        insights.push(format!(
            "In {end_year}, {both} countries reported both technologies, {solo_solar} solar only, and {solo_wind} wind only. Overlap grew as early adopters diversified their renewable mix.",
            end_year = last.year,
            both = last.both,
            solo_solar = solo_solar,
            solo_wind = solo_wind
        ));
    }

    if points.len() >= 11 {
        let mid_year = first.year + (last.year - first.year) / 2;
        let early = points
            .iter()
            .find(|point| point.year == mid_year)
            .or_else(|| points.get(points.len() / 2));
        if let Some(mid) = early {
            let early_growth = mid.either as i32 - first.either as i32;
            let late_growth = last.either as i32 - mid.either as i32;
            if late_growth > early_growth {
                insights.push(format!(
                    "Adoption accelerated in the second half: +{early} unique reporters through {mid_year}, then +{late} more by {end_year}.",
                    early = early_growth,
                    mid_year = mid.year,
                    late = late_growth,
                    end_year = last.year
                ));
            }
        }
    }

    insights
}

#[derive(Clone)]
pub struct DeltaRow {
    pub country: String,
    pub start: f64,
    pub end: f64,
}

pub fn top_country_deltas(
    records: &[EnergyRecord],
    year_start: i32,
    year_end: i32,
    metric_key: &str,
    count: usize,
) -> Vec<DeltaRow> {
    let mut rows: Vec<DeltaRow> = country_names(records)
        .into_iter()
        .filter_map(|country| {
            let start = records
                .iter()
                .find(|record| record.c == country && record.y == year_start)
                .and_then(|record| metric_value(record, metric_key))?;
            let end = records
                .iter()
                .find(|record| record.c == country && record.y == year_end)
                .and_then(|record| metric_value(record, metric_key))?;
            Some(DeltaRow {
                country,
                start,
                end,
            })
        })
        .collect();

    rows.sort_by(|a, b| {
        (b.end - b.start)
            .partial_cmp(&(a.end - a.start))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    rows.truncate(count);
    rows
}

#[derive(Clone)]
pub struct BumpPoint {
    pub year: i32,
    pub rank: usize,
    pub value: f64,
}

#[derive(Clone)]
pub struct BumpSeries {
    pub label: String,
    pub color: &'static str,
    pub points: Vec<BumpPoint>,
    pub highlight: bool,
}

const BUMP_COLORS: [&str; 8] = [
    "#f0e442", "#56b4e9", "#a855f7", "#009e73", "#d55e00", "#0072b2", "#cc79a7", "#e69f00",
];

fn rank_at_year(
    records: &[EnergyRecord],
    year: i32,
    metric_key: &str,
    country: &str,
) -> Option<(usize, f64)> {
    let mut ranked: Vec<(String, f64)> = records
        .iter()
        .filter(|record| record.y == year && is_country(&record.c))
        .filter_map(|record| {
            metric_value(record, metric_key).map(|value| (record.c.clone(), value))
        })
        .filter(|(_, value)| *value > 0.0)
        .collect();

    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    ranked
        .iter()
        .position(|(name, _)| name == country)
        .map(|index| (index + 1, ranked[index].1))
}

pub fn solar_rank_race_series(
    records: &[EnergyRecord],
    metric_key: &str,
    snapshot_years: &[i32],
    final_top_n: usize,
) -> (Vec<BumpSeries>, usize) {
    let Some(final_year) = snapshot_years.last().copied() else {
        return (Vec::new(), 5);
    };

    let final_leaders = top_countries_by_metric(records, final_year, metric_key, final_top_n);
    let mut focus: std::collections::HashSet<String> = final_leaders.iter().cloned().collect();

    for year in snapshot_years {
        if let Some(leader) = top_countries_by_metric(records, *year, metric_key, 1).first() {
            focus.insert(leader.clone());
        }
    }

    let mut countries: Vec<String> = focus.into_iter().collect();
    countries.sort_by(|a, b| {
        let rank_a = rank_at_year(records, final_year, metric_key, a)
            .map(|(rank, _)| rank)
            .unwrap_or(usize::MAX);
        let rank_b = rank_at_year(records, final_year, metric_key, b)
            .map(|(rank, _)| rank)
            .unwrap_or(usize::MAX);
        rank_a.cmp(&rank_b).then_with(|| a.cmp(b))
    });

    let max_rank = snapshot_years
        .iter()
        .flat_map(|year| {
            countries.iter().filter_map(|country| {
                rank_at_year(records, *year, metric_key, country).map(|(rank, _)| rank)
            })
        })
        .max()
        .unwrap_or(final_top_n)
        .max(final_top_n);

    let final_leader_set: std::collections::HashSet<_> = final_leaders.iter().cloned().collect();

    let series = countries
        .into_iter()
        .enumerate()
        .filter_map(|(index, country)| {
            let points: Vec<BumpPoint> = snapshot_years
                .iter()
                .filter_map(|year| {
                    rank_at_year(records, *year, metric_key, &country).map(|(rank, value)| {
                        BumpPoint {
                            year: *year,
                            rank,
                            value,
                        }
                    })
                })
                .collect();

            if points.len() < 2 {
                return None;
            }

            Some(BumpSeries {
                label: country.clone(),
                color: BUMP_COLORS[index % BUMP_COLORS.len()],
                highlight: final_leader_set.contains(&country),
                points,
            })
        })
        .collect();

    (series, max_rank)
}

