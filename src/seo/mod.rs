use crate::lab::modules::find_by_slug as find_module;
use crate::projects::find_by_slug as find_project;
use crate::reports::find_by_slug as find_report;
use crate::tidy_tuesday::find_by_slug as find_tidy_tuesday;
use crate::utils::APP_BASE;

pub const SITE_NAME: &str = "Jack Weekly";
pub const SITE_URL: &str = "https://saptreekly.github.io/Research-Publications";
pub const OG_IMAGE_URL: &str = "https://saptreekly.github.io/Research-Publications/static/og-card.png";
pub const DEFAULT_TITLE: &str = "Jack Weekly | Analysis & Security";
pub const DEFAULT_DESCRIPTION: &str = "Portfolio of Jack Weekly: malware forensics, strategic studies, security engineering, and applied cryptography research. Dual US & NZ citizen.";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeoMeta {
    pub title: String,
    pub description: String,
    pub canonical_path: String,
}

impl SeoMeta {
    pub fn canonical_url(&self) -> String {
        if self.canonical_path == "/" {
            format!("{SITE_URL}/")
        } else {
            format!("{SITE_URL}{}", self.canonical_path)
        }
    }
}

pub fn seo_for_path(pathname: &str) -> SeoMeta {
    let path = normalize_path(pathname);

    if path == "/" {
        return SeoMeta {
            title: DEFAULT_TITLE.into(),
            description: DEFAULT_DESCRIPTION.into(),
            canonical_path: "/".into(),
        };
    }

    if path == "/contact" {
        return page(
            "/contact",
            "Contact | Jack Weekly",
            "Contact Jack Weekly for national security, cyber, intelligence-adjacent, and technical analysis roles.",
        );
    }

    if path == "/curriculum" {
        return page(
            "/curriculum",
            "Julia Cryptography Curriculum | Jack Weekly",
            "Eight-module Julia cryptography curriculum with theory notes and interactive browser labs covering number theory, primes, and RSA.",
        );
    }

    if let Some(slug) = path.strip_prefix("/curriculum/lab/") {
        if let Some(module) = find_module(slug) {
            return page(
                &path,
                &format!("{} Lab | Jack Weekly", module.title),
                &format!(
                    "Interactive browser lab for {} in the Julia cryptography curriculum.",
                    module.title
                ),
            );
        }
    }

    if let Some(slug) = path.strip_prefix("/curriculum/") {
        if let Some(module) = find_module(slug) {
            return page(
                &path,
                &format!("{} | Jack Weekly", module.title),
                &format!(
                    "Theory module on {} from the Julia cryptography curriculum.",
                    module.title
                ),
            );
        }
    }

    if let Some(slug) = path.strip_prefix("/research/") {
        if let Some(report) = find_report(slug) {
            return page(
                &path,
                &format!("{} | Jack Weekly", report.title),
                &format!("{} · {}", report.subtitle, report.tag),
            );
        }
    }

    if let Some(slug) = path.strip_prefix("/projects/") {
        if let Some(project) = find_project(slug) {
            return page(
                &path,
                &format!("{} | Jack Weekly", project.title),
                &format!("{} · {}", project.subtitle, project.tag),
            );
        }
    }

    if path == "/tidy-tuesday" {
        return page(
            "/tidy-tuesday",
            "Tidy Tuesday | Jack Weekly",
            "Weekly Julia data explorations using TidyTuesday community datasets.",
        );
    }

    if let Some(slug) = path.strip_prefix("/tidy-tuesday/") {
        if let Some(entry) = find_tidy_tuesday(slug) {
            return page(
                &path,
                &format!("{} | Jack Weekly", entry.title),
                &format!("{} · {}", entry.subtitle, entry.tag),
            );
        }
    }

    SeoMeta {
        title: DEFAULT_TITLE.into(),
        description: DEFAULT_DESCRIPTION.into(),
        canonical_path: path,
    }
}

fn page(canonical_path: &str, title: &str, description: &str) -> SeoMeta {
    SeoMeta {
        title: title.into(),
        description: description.into(),
        canonical_path: canonical_path.into(),
    }
}

fn normalize_path(pathname: &str) -> String {
    let stripped = pathname
        .strip_prefix(APP_BASE)
        .unwrap_or(pathname)
        .trim_end_matches('/');

    if stripped.is_empty() {
        "/".to_string()
    } else {
        format!("/{stripped}")
    }
}
