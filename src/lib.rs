pub mod components;
pub mod lab;
pub mod pages;
pub mod projects;
pub mod reports;
pub mod seo;
pub mod theme;
pub mod utils;

#[cfg(feature = "malware-traffic")]
pub mod malware_traffic;

#[cfg(feature = "situation-monitor")]
pub mod situation_monitor;

#[cfg(feature = "tidy-tuesday")]
pub mod tidy_tuesday;

pub const APP_BASE: &str = "/Research-Publications";
