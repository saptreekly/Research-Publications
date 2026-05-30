use serde::{Deserialize, Serialize};

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
    pub source_id: String,
    pub source_name: String,
    pub category: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FeedSnapshot {
    pub updated_at: String,
    pub updated_label: String,
    pub categories: Vec<CategoryMeta>,
    pub sources: Vec<SourceMeta>,
    pub items: Vec<FeedItem>,
}

pub const ALL_CATEGORY: &str = "all";

pub fn category_label(category: &str) -> &'static str {
    match category {
        "nz-pacific" => "NZ & Pacific",
        "apac-security" => "APAC Security",
        "cyber" => "Cyber",
        "global" => "Global",
        ALL_CATEGORY => "All",
        _ => "Other",
    }
}

pub fn filter_items<'a>(
    items: &'a [FeedItem],
    category: &str,
    query: &str,
) -> Vec<&'a FeedItem> {
    let normalized_query = query.trim().to_lowercase();

    items
        .iter()
        .filter(|item| category == ALL_CATEGORY || item.category == category)
        .filter(|item| {
            if normalized_query.is_empty() {
                return true;
            }

            item.title.to_lowercase().contains(&normalized_query)
                || item.summary.to_lowercase().contains(&normalized_query)
                || item.source_name.to_lowercase().contains(&normalized_query)
        })
        .collect()
}

pub fn active_source_count(snapshot: &FeedSnapshot) -> usize {
    snapshot
        .sources
        .iter()
        .filter(|source| source.status == "ok")
        .count()
}
