use std::collections::HashSet;

const SEEN_KEY: &str = "sm_seen_ids";
const WATCH_KEY: &str = "sm_watch_terms";
const MAX_SEEN: usize = 500;

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

pub fn load_seen_ids() -> HashSet<String> {
    storage()
        .and_then(|store| store.get_item(SEEN_KEY).ok())
        .flatten()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

pub fn persist_seen_ids(ids: HashSet<String>) {
    if let Some(store) = storage() {
        let trimmed: Vec<String> = ids.into_iter().take(MAX_SEEN).collect();
        if let Ok(raw) = serde_json::to_string(&trimmed) {
            let _ = store.set_item(SEEN_KEY, &raw);
        }
    }
}

pub fn load_watch_terms() -> Vec<String> {
    storage()
        .and_then(|store| store.get_item(WATCH_KEY).ok())
        .flatten()
        .map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|term| !term.is_empty())
                .map(str::to_lowercase)
                .collect()
        })
        .unwrap_or_default()
}

pub fn save_watch_terms(terms: &[String]) {
    if let Some(store) = storage() {
        let raw = terms.join(", ");
        let _ = store.set_item(WATCH_KEY, &raw);
    }
}
