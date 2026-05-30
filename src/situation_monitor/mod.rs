use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod map;
pub mod storage;

pub const FEED_LOCAL_URL: &str = "static/situation-monitor/feed.json";
pub const FEED_RAW_URL: &str =
    "https://raw.githubusercontent.com/saptreekly/Research-Publications/main/static/situation-monitor/feed.json";
/// Browser poll interval (5 minutes). Matches server aggregation cadence.
pub const FEED_POLL_MS: u32 = 300_000;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CategoryMeta {
    pub id: String,
    pub label: String,
    pub count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SourceMeta {
    pub id: String,
    pub name: String,
    pub category: String,
    pub url: String,
    pub item_count: usize,
    pub status: String,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FeedItem {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub summary: String,
    pub url: String,
    #[serde(default)]
    pub published_at: Option<String>,
    pub published_label: String,
    #[serde(default)]
    pub age_label: String,
    pub source_id: String,
    pub source_name: String,
    #[serde(default = "default_source_kind")]
    pub source_kind: String,
    pub category: String,
    #[serde(default)]
    pub regions: Vec<String>,
    #[serde(default)]
    pub priority: u8,
    #[serde(default)]
    pub cluster_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TrendMeta {
    pub term: String,
    pub count: usize,
}

fn default_source_kind() -> String {
    "rss".to_string()
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FeedSnapshot {
    pub updated_at: String,
    pub updated_label: String,
    pub categories: Vec<CategoryMeta>,
    pub sources: Vec<SourceMeta>,
    pub items: Vec<FeedItem>,
    #[serde(default)]
    pub trends: Vec<TrendMeta>,
}

pub const ALL_CATEGORY: &str = "all";
pub const ALL_REGION: &str = "all";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeBucket {
    Recent,
    Today,
    Yesterday,
    Earlier,
    Unknown,
}

impl TimeBucket {
    pub fn label(self) -> &'static str {
        match self {
            Self::Recent => "Last 6 hours",
            Self::Today => "Today",
            Self::Yesterday => "Yesterday",
            Self::Earlier => "Earlier",
            Self::Unknown => "Undated",
        }
    }

    pub fn order(self) -> u8 {
        match self {
            Self::Recent => 0,
            Self::Today => 1,
            Self::Yesterday => 2,
            Self::Earlier => 3,
            Self::Unknown => 4,
        }
    }
}

pub fn category_label(category: &str) -> &'static str {
    match category {
        "nz-pacific" => "NZ & Pacific",
        "apac-security" => "APAC Security",
        "cyber" => "Cyber",
        "global" => "Global",
        "osint" => "OSINT",
        ALL_CATEGORY => "All",
        _ => "Other",
    }
}

pub fn category_class(category: &str) -> &'static str {
    match category {
        "nz-pacific" => "sm-item-cat-nz-pacific",
        "apac-security" => "sm-item-cat-apac-security",
        "cyber" => "sm-item-cat-cyber",
        "global" => "sm-item-cat-global",
        "osint" => "sm-item-cat-osint",
        _ => "sm-item-cat-other",
    }
}

pub fn source_kind_label(kind: &str) -> &'static str {
    match kind {
        "social" => "X",
        _ => "RSS",
    }
}

pub fn item_age_label(item: &FeedItem) -> String {
    if !item.age_label.is_empty() {
        return item.age_label.clone();
    }
    item.published_label.clone()
}

pub fn time_bucket(published_at: Option<&str>) -> TimeBucket {
    let Some(raw) = published_at else {
        return TimeBucket::Unknown;
    };

    let published_ms = js_sys::Date::parse(raw);
    if published_ms.is_nan() {
        return TimeBucket::Unknown;
    }

    let seconds = ((js_sys::Date::now() - published_ms) / 1000.0) as i64;
    if seconds < 6 * 3600 {
        TimeBucket::Recent
    } else if seconds < 86_400 {
        TimeBucket::Today
    } else if seconds < 172_800 {
        TimeBucket::Yesterday
    } else {
        TimeBucket::Earlier
    }
}

pub fn region_label(region: &str) -> &'static str {
    match region {
        "nz" => "New Zealand",
        "pacific" => "Pacific",
        "australia" => "Australia",
        "china" => "China",
        "taiwan" => "Taiwan",
        "japan" => "Japan",
        "korea" => "Korea",
        "se-asia" => "SE Asia",
        "india" => "India",
        "middle-east" => "Middle East",
        "europe" => "Europe",
        "us" => "United States",
        "africa" => "Africa",
        "russia" => "Russia",
        "global" => "Global",
        ALL_REGION => "All regions",
        _ => "Other",
    }
}

pub fn priority_tier(priority: u8) -> &'static str {
    if priority >= 60 {
        "sm-priority-high"
    } else if priority >= 30 {
        "sm-priority-mid"
    } else {
        "sm-priority-low"
    }
}

pub fn matches_watch(item: &FeedItem, terms: &[String]) -> bool {
    if terms.is_empty() {
        return false;
    }
    let haystack = format!(
        "{} {} {}",
        item.title.to_lowercase(),
        item.summary.to_lowercase(),
        item.source_name.to_lowercase()
    );
    terms.iter().any(|term| haystack.contains(term))
}

pub fn filter_items(
    items: &[FeedItem],
    category: &str,
    query: &str,
    region: Option<&str>,
    watch_terms: &[String],
    watch_only: bool,
) -> Vec<FeedItem> {
    let normalized_query = query.trim().to_lowercase();

    items
        .iter()
        .filter(|item| category == ALL_CATEGORY || item.category == category)
        .filter(|item| {
            region.is_none_or(|active| active == ALL_REGION || item_matches_region(item, active))
        })
        .filter(|item| {
            if watch_only {
                return matches_watch(item, watch_terms);
            }
            true
        })
        .filter(|item| {
            if normalized_query.is_empty() {
                return true;
            }

            item.title.to_lowercase().contains(&normalized_query)
                || item.summary.to_lowercase().contains(&normalized_query)
                || item.source_name.to_lowercase().contains(&normalized_query)
        })
        .cloned()
        .collect()
}

pub fn region_counts(items: &[FeedItem]) -> HashMap<String, usize> {
    build_feed_index(items).region_counts
}

const REGION_KEYWORDS: &[(&str, &[&str])] = &[
    ("nz", &["new zealand", "wellington", "auckland", "christchurch", "nzdf", "rnz"]),
    (
        "pacific",
        &[
            "pacific", "fiji", "samoa", "tonga", "vanuatu", "papua", "solomon", "micronesia",
            "polynesia", "guam",
        ],
    ),
    (
        "australia",
        &["australia", "sydney", "canberra", "melbourne", "australian"],
    ),
    (
        "china",
        &["china", "beijing", "shanghai", "chinese", "prc", "south china sea"],
    ),
    ("taiwan", &["taiwan", "taipei", "strait"]),
    ("japan", &["japan", "tokyo", "japanese", "okinawa"]),
    ("korea", &["korea", "seoul", "pyongyang", "dprk"]),
    (
        "se-asia",
        &[
            "indonesia", "malaysia", "singapore", "philippines", "vietnam", "thailand", "asean",
            "myanmar", "cambodia",
        ],
    ),
    ("india", &["india", "delhi", "mumbai", "indian", "modi"]),
    (
        "middle-east",
        &[
            "iran", "israel", "gaza", "syria", "yemen", "saudi", "middle east", "red sea", "houthi",
        ],
    ),
    (
        "europe",
        &["europe", "ukraine", "nato", "european", "london", "berlin", "france", "eu "],
    ),
    (
        "us",
        &["united states", "u.s.", "washington", "pentagon", "american", "white house"],
    ),
    ("africa", &["africa", "sudan", "sahel", "nigeria", "congo"]),
    ("russia", &["russia", "moscow", "kremlin", "putin"]),
];

pub fn infer_regions_from_text(title: &str, summary: &str, category: &str) -> Vec<String> {
    let text = format!("{title} {summary}").to_lowercase();
    let mut regions: Vec<String> = REGION_KEYWORDS
        .iter()
        .filter(|(_, keywords)| keywords.iter().any(|keyword| text.contains(keyword)))
        .map(|(region, _)| (*region).to_string())
        .collect();

    if category == "nz-pacific"
        && !regions.iter().any(|r| r == "nz" || r == "pacific" || r == "australia")
    {
        regions.push("pacific".to_string());
    }
    if category == "apac-security" && regions.is_empty() {
        regions.push("se-asia".to_string());
    }
    if category == "global" && regions.is_empty() {
        regions.push("global".to_string());
    }
    if regions.is_empty() {
        regions.push("global".to_string());
    }
    regions
}

pub fn item_regions(item: &FeedItem) -> Vec<String> {
    if !item.regions.is_empty() {
        return item.regions.clone();
    }
    infer_regions_from_text(&item.title, &item.summary, &item.category)
}

pub fn enrich_item_regions(item: &FeedItem) -> FeedItem {
    if !item.regions.is_empty() {
        return item.clone();
    }
    let mut enriched = item.clone();
    enriched.regions = infer_regions_from_text(&item.title, &item.summary, &item.category);
    enriched
}

#[derive(Clone, Default, PartialEq)]
pub struct FeedIndex {
    pub items: Vec<FeedItem>,
    pub region_counts: HashMap<String, usize>,
    pub region_previews: HashMap<String, Vec<FeedItem>>,
}

pub fn build_feed_index(items: &[FeedItem]) -> FeedIndex {
    let items: Vec<FeedItem> = items.iter().map(enrich_item_regions).collect();
    let mut region_counts = HashMap::new();
    let mut region_buckets: HashMap<String, Vec<FeedItem>> = HashMap::new();

    for item in &items {
        for region in &item.regions {
            *region_counts.entry(region.clone()).or_insert(0) += 1;
            region_buckets
                .entry(region.clone())
                .or_default()
                .push(item.clone());
        }
    }

    let mut region_previews = HashMap::new();
    for (region, mut bucket) in region_buckets {
        bucket.sort_by(|a, b| b.priority.cmp(&a.priority));
        bucket.truncate(4);
        region_previews.insert(region, bucket);
    }

    FeedIndex {
        items,
        region_counts,
        region_previews,
    }
}

pub fn item_matches_region(item: &FeedItem, region: &str) -> bool {
    item.regions.iter().any(|r| r == region)
}

pub fn breaking_items(items: &[FeedItem]) -> Vec<FeedItem> {
    items
        .iter()
        .filter(|item| {
            time_bucket(item.published_at.as_deref()) == TimeBucket::Recent || item.priority >= 40
        })
        .cloned()
        .take(12)
        .collect()
}

pub fn cluster_size(items: &[FeedItem], item: &FeedItem) -> usize {
    cluster_size_by_key(items, &item.cluster_key)
}

pub fn cluster_size_by_key(items: &[FeedItem], cluster_key: &str) -> usize {
    if cluster_key.is_empty() {
        return 1;
    }
    items
        .iter()
        .filter(|candidate| candidate.cluster_key == cluster_key)
        .count()
}

pub fn top_items_for_category(items: &[FeedItem], category: &str, limit: usize) -> Vec<FeedItem> {
    items
        .iter()
        .filter(|item| item.category == category)
        .take(limit)
        .cloned()
        .collect()
}

pub fn top_items_for_region(items: &[FeedItem], region: &str, limit: usize) -> Vec<FeedItem> {
    items
        .iter()
        .filter(|item| item_matches_region(item, region))
        .take(limit)
        .cloned()
        .collect()
}

pub fn region_previews_for(items: &[FeedItem], limit: usize) -> HashMap<String, Vec<FeedItem>> {
    build_feed_index(items).region_previews.into_iter().map(|(region, mut bucket)| {
        bucket.truncate(limit);
        (region, bucket)
    }).collect()
}

pub fn format_utc_clock() -> String {
    let date = js_sys::Date::new_0();
    format!(
        "{:02}:{:02} UTC",
        date.get_utc_hours(),
        date.get_utc_minutes()
    )
}

pub fn format_nzst_clock() -> String {
    let date = js_sys::Date::new_0();
    let total_minutes = date.get_utc_hours() * 60 + date.get_utc_minutes() + 12 * 60;
    format!(
        "{:02}:{:02} NZST",
        (total_minutes / 60) % 24,
        total_minutes % 60
    )
}

pub fn group_by_time_bucket(items: &[FeedItem]) -> Vec<(TimeBucket, Vec<FeedItem>)> {
    let mut buckets = [
        (TimeBucket::Recent, Vec::new()),
        (TimeBucket::Today, Vec::new()),
        (TimeBucket::Yesterday, Vec::new()),
        (TimeBucket::Earlier, Vec::new()),
        (TimeBucket::Unknown, Vec::new()),
    ];

    for item in items {
        let bucket = time_bucket(item.published_at.as_deref());
        let index = bucket.order() as usize;
        buckets[index].1.push(item.clone());
    }

    buckets
        .into_iter()
        .filter(|(_, group)| !group.is_empty())
        .collect()
}

pub fn active_source_count(snapshot: &FeedSnapshot) -> usize {
    snapshot
        .sources
        .iter()
        .filter(|source| source.status == "ok")
        .count()
}

pub fn social_item_count(snapshot: &FeedSnapshot) -> usize {
    snapshot
        .items
        .iter()
        .filter(|item| item.source_kind == "social")
        .count()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoryCluster {
    pub lead: FeedItem,
    pub size: usize,
}

/// Collapse near-duplicate headlines into one row per story.
pub fn cluster_stories(items: &[FeedItem]) -> Vec<StoryCluster> {
    let mut clusters = Vec::new();
    let mut seen_keys = std::collections::HashSet::new();

    for item in items {
        let key = if item.cluster_key.is_empty() {
            item.id.clone()
        } else {
            item.cluster_key.clone()
        };
        if !seen_keys.insert(key.clone()) {
            continue;
        }
        let size = cluster_size_by_key(items, &key);
        clusters.push(StoryCluster {
            lead: item.clone(),
            size,
        });
    }

    clusters
}

pub fn format_checked_at(ms: f64) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms));
    format!(
        "{:02}:{:02}:{:02}",
        date.get_hours(),
        date.get_minutes(),
        date.get_seconds()
    )
}

pub const DIGEST_INITIAL: usize = 8;
