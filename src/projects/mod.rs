pub struct ProjectMeta {
    pub slug: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub tag: &'static str,
    pub date: &'static str,
    pub src: &'static str,
    pub repo_url: &'static str,
}

pub const PROJECTS: &[ProjectMeta] = &[
    ProjectMeta {
        slug: "hlidskjalf",
        title: "Project Hliðskjálf",
        subtitle: "Type-1.5 thin hypervisor for legacy x86_64 host hardening",
        tag: "Rust · Assembly",
        date: "2026-04",
        src: "research-docs/projects/hlidskjalf.md",
        repo_url: "https://github.com/saptreekly/Project-Hlidskjalf",
    },
    ProjectMeta {
        slug: "siem-ensemble",
        title: "SIEM Ensemble",
        subtitle: "Polyglot log ingestion and real-time analytics pipeline",
        tag: "Rust · Zig · Odin · Elixir · Assembly",
        date: "2026-04",
        src: "research-docs/projects/siem-ensemble.md",
        repo_url: "https://github.com/saptreekly/SIEM",
    },
    ProjectMeta {
        slug: "geospatial-intel",
        title: "Geospatial Intel Server",
        subtitle: "Viewport-filtered aircraft tracking over WebSocket with H3 indexing",
        tag: "Go · Rust · Wasm",
        date: "2026-04",
        src: "research-docs/projects/geospatial-intel.md",
        repo_url: "https://github.com/saptreekly/geospatial-intel",
    },
];

pub fn find_by_slug(slug: &str) -> Option<&'static ProjectMeta> {
    PROJECTS.iter().find(|project| project.slug == slug)
}
