pub mod home;
pub mod curriculum;
pub mod contact;
pub mod layout;
pub mod module;
pub mod project;
pub mod report;

#[cfg(feature = "lab")]
pub mod lab;

#[cfg(feature = "malware-traffic")]
pub mod malware_traffic;

#[cfg(feature = "situation-monitor")]
pub mod situation_monitor;

#[cfg(feature = "tidy-tuesday")]
pub mod tidy_tuesday;
