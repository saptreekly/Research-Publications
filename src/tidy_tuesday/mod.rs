use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MetricDef {
    pub id: String,
    pub label: String,
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnergyRecord {
    pub c: String,
    pub y: i32,
    #[serde(default)]
    pub solar_tfec: Option<f64>,
    #[serde(default)]
    pub wind_tfec: Option<f64>,
    #[serde(default)]
    pub hydro_tfec: Option<f64>,
    #[serde(default)]
    pub geothermal_tfec: Option<f64>,
    #[serde(default)]
    pub biomass_tfec: Option<f64>,
    #[serde(default)]
    pub solar_tj: Option<f64>,
    #[serde(default)]
    pub elec_pct: Option<f64>,
    #[serde(default)]
    pub renew_tfec: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExploreData {
    pub metrics: Vec<MetricDef>,
    pub records: Vec<EnergyRecord>,
}

pub fn year_bounds(records: &[EnergyRecord]) -> (i32, i32) {
    let mut min_year = i32::MAX;
    let mut max_year = i32::MIN;

    for record in records {
        min_year = min_year.min(record.y);
        max_year = max_year.max(record.y);
    }

    if min_year == i32::MAX {
        (1990, 2010)
    } else {
        (min_year, max_year)
    }
}

pub fn metric_value(record: &EnergyRecord, key: &str) -> Option<f64> {
    match key {
        "solar_tfec" => record.solar_tfec,
        "wind_tfec" => record.wind_tfec,
        "hydro_tfec" => record.hydro_tfec,
        "geothermal_tfec" => record.geothermal_tfec,
        "biomass_tfec" => record.biomass_tfec,
        "solar_tj" => record.solar_tj,
        "elec_pct" => record.elec_pct,
        "renew_tfec" => record.renew_tfec,
        _ => None,
    }
}

pub fn country_series(
    records: &[EnergyRecord],
    country: &str,
    metric_key: &str,
    year_start: i32,
    year_end: i32,
) -> Vec<(i32, f64)> {
    let mut points: Vec<(i32, f64)> = records
        .iter()
        .filter(|record| record.c == country && record.y >= year_start && record.y <= year_end)
        .filter_map(|record| metric_value(record, metric_key).map(|value| (record.y, value)))
        .collect();
    points.sort_by_key(|(year, _)| *year);
    points
}

pub fn solar_ranking(
    records: &[EnergyRecord],
    year: i32,
    count: usize,
    ascending: bool,
) -> Vec<(String, f64)> {
    let mut rows: Vec<(String, f64)> = records
        .iter()
        .filter(|record| record.y == year)
        .filter_map(|record| record.solar_tj.map(|value| (record.c.clone(), value)))
        .filter(|(_, value)| *value > 0.0)
        .collect();

    if ascending {
        rows.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    }

    rows.truncate(count);
    rows
}

pub mod analysis;
pub mod charts;
pub mod controls;
pub mod se4all;

pub struct TidyTuesdayMeta {
    pub slug: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub tag: &'static str,
    pub date: &'static str,
    pub dataset_date: &'static str,
    pub src: &'static str,
    pub julia_src: &'static str,
    pub explore_data: Option<&'static str>,
}

pub const ENTRIES: &[TidyTuesdayMeta] = &[TidyTuesdayMeta {
    slug: "se4all-2026-05-26",
    title: "Sustainable Energy for All",
    subtitle: "Tidy Tuesday · SE4ALL country-level energy metrics · Julia analysis",
    tag: "Data analysis",
    date: "2026-05",
    dataset_date: "2026-05-26",
    src: "research-docs/tidy-tuesday/se4all-2026-05-26.md",
    julia_src: "research-docs/tidy-tuesday/analysis-2026-05-26.jl",
    explore_data: Some("static/tidy-tuesday/2026-05-26/explore.json"),
}];

pub fn find_by_slug(slug: &str) -> Option<&'static TidyTuesdayMeta> {
    ENTRIES.iter().find(|entry| entry.slug == slug)
}
