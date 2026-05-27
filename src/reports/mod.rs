pub struct ReportMeta {
    pub slug: &'static str,
    pub title: &'static str,
    pub tag: &'static str,
    pub date: &'static str,
    pub src: &'static str,
    pub sigma_src: Option<&'static str>,
    pub ioc_src: Option<&'static str>,
}

pub const REPORTS: &[ReportMeta] = &[ReportMeta {
    slug: "lovely-malware",
    title: "LovelyMalware",
    tag: "Malware analysis",
    date: "2026-04",
    src: "research-docs/reports/lovely-malware.md",
    sigma_src: Some("research-docs/reports/lovely-malware-sigma.md"),
    ioc_src: Some("research-docs/reports/lovely-malware-ioc.md"),
}];

pub fn find_by_slug(slug: &str) -> Option<&'static ReportMeta> {
    REPORTS.iter().find(|report| report.slug == slug)
}
