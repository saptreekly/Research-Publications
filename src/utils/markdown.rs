use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub fn rendered_html_path(md_path: &str) -> String {
    let relative = md_path
        .strip_prefix("research-docs/")
        .unwrap_or(md_path)
        .trim_end_matches(".md");
    format!("static/rendered/{relative}.html")
}

pub fn rendered_lab_cache_path(module_id: &str) -> String {
    format!("static/rendered/lab/{module_id}.json")
}

pub fn rendered_malware_report_path(stem: &str) -> String {
    format!("static/rendered/malware-traffic/{stem}.report.json")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SigmaRulePreview {
    pub name: String,
    pub title: String,
    pub level: String,
    pub mitre: String,
    pub yaml: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MalwareReportCache {
    pub intro_html: String,
    pub phase_supplements: HashMap<String, String>,
    pub detections: Vec<(String, String)>,
    pub references: Vec<(String, Option<String>)>,
    pub sigma_rules: Vec<SigmaRulePreview>,
}
