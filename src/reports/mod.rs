pub struct ReportMeta {
    pub slug: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub tag: &'static str,
    pub date: &'static str,
    pub src: &'static str,
    pub sigma_src: Option<&'static str>,
    pub ioc_src: Option<&'static str>,
}

pub const REPORTS: &[ReportMeta] = &[
    ReportMeta {
        slug: "conti-locker",
        title: "Conti Locker v2",
        subtitle: "Malware analysis report · leaked ransomware build tree · Vanguard-RE",
        tag: "Malware analysis",
        date: "2026-07",
        src: "research-docs/reports/conti-locker.md",
        sigma_src: Some("research-docs/reports/conti-locker-sigma.md"),
        ioc_src: Some("research-docs/reports/conti-locker-ioc.md"),
    },
    ReportMeta {
        slug: "lovely-malware",
        title: "LovelyMalware",
        subtitle: "Malware analysis report · HackTheBox Insane",
        tag: "Malware analysis",
        date: "2026-04",
        src: "research-docs/reports/lovely-malware.md",
        sigma_src: Some("research-docs/reports/lovely-malware-sigma.md"),
        ioc_src: Some("research-docs/reports/lovely-malware-ioc.md"),
    },
    ReportMeta {
        slug: "thesis-summary",
        title: "5th-Gen Fighter Exports as Strategic Competition",
        subtitle: "Strategic studies writing sample · Victoria University of Wellington · MStrat with Merit",
        tag: "Strategic analysis",
        date: "2024",
        src: "research-docs/thesis-summary.md",
        sigma_src: None,
        ioc_src: None,
    },
];

pub fn find_by_slug(slug: &str) -> Option<&'static ReportMeta> {
    REPORTS.iter().find(|report| report.slug == slug)
}
