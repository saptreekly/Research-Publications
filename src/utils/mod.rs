pub fn resolve_asset_url(relative_path: &str) -> String {
    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");

    let base: String = document
        .base_uri()
        .ok()
        .flatten()
        .unwrap_or_else(|| {
            window
                .location()
                .href()
                .unwrap_or_else(|_| "/".to_string())
        });

    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        relative_path.trim_start_matches('/')
    )
}

pub fn is_html_content(content: &str) -> bool {
    let trimmed = content.trim_start();
    trimmed.starts_with("<!DOCTYPE")
        || trimmed.starts_with("<!doctype")
        || trimmed.starts_with("<html")
        || trimmed.starts_with("<HTML")
        || trimmed.contains("<head>")
        || trimmed.contains("TrunkApplicationStarted")
        || trimmed.contains("Build failure")
}

pub mod debounce;
pub mod markdown;
pub mod script_loader;

pub const APP_BASE: &str = "/Research-Publications";

pub fn home_href() -> String {
    format!("{APP_BASE}/")
}

pub fn curriculum_href() -> String {
    format!("{APP_BASE}/curriculum")
}

pub fn module_href(slug: &str) -> String {
    format!("{APP_BASE}/curriculum/{slug}")
}

pub fn lab_href(slug: &str) -> String {
    format!("{APP_BASE}/curriculum/lab/{slug}")
}

pub fn contact_href() -> String {
    format!("{APP_BASE}/contact")
}

pub fn report_href(slug: &str) -> String {
    format!("{APP_BASE}/research/{slug}")
}

pub fn project_href(slug: &str) -> String {
    format!("{APP_BASE}/projects/{slug}")
}

pub fn tidy_tuesday_href(slug: &str) -> String {
    format!("{APP_BASE}/tidy-tuesday/{slug}")
}

pub fn tidy_tuesday_index_href() -> String {
    format!("{APP_BASE}/tidy-tuesday")
}

pub fn situation_monitor_href() -> String {
    format!("{APP_BASE}/situation-monitor")
}
