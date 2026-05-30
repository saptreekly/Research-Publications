#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModuleMeta {
    pub id: &'static str,
    pub slug: &'static str,
    pub title: &'static str,
    pub section: &'static str,
    pub theory_src: &'static str,
    pub lab_src: &'static str,
}

pub const ALL_MODULES: &[ModuleMeta] = &[
    ModuleMeta {
        id: "mod_01",
        slug: "mod-01",
        title: "01.1 / Modular Inverses",
        section: "Number Theory",
        theory_src: "research-docs/julia-crypto/mod_01.md",
        lab_src: "research-docs/julia-crypto/mod_01_lab.md",
    },
    ModuleMeta {
        id: "mod_02",
        slug: "mod-02",
        title: "01.2 / Modular Exponentiation",
        section: "Number Theory",
        theory_src: "research-docs/julia-crypto/mod_02.md",
        lab_src: "research-docs/julia-crypto/mod_02_lab.md",
    },
    ModuleMeta {
        id: "mod_03",
        slug: "mod-03",
        title: "01.3 / Chinese Remainder Theorem",
        section: "Number Theory",
        theory_src: "research-docs/julia-crypto/mod_03.md",
        lab_src: "research-docs/julia-crypto/mod_03_lab.md",
    },
    ModuleMeta {
        id: "mod_04",
        slug: "mod-04",
        title: "01.4 / Fermat and Euler Theorems",
        section: "Number Theory",
        theory_src: "research-docs/julia-crypto/mod_04.md",
        lab_src: "research-docs/julia-crypto/mod_04_lab.md",
    },
    ModuleMeta {
        id: "mod_05",
        slug: "mod-05",
        title: "02.1 / Sieve of Eratosthenes",
        section: "Primes",
        theory_src: "research-docs/julia-crypto/mod_05.md",
        lab_src: "research-docs/julia-crypto/mod_05_lab.md",
    },
    ModuleMeta {
        id: "mod_06",
        slug: "mod-06",
        title: "02.2 / Primes.jl",
        section: "Primes",
        theory_src: "research-docs/julia-crypto/mod_06.md",
        lab_src: "research-docs/julia-crypto/mod_06_lab.md",
    },
    ModuleMeta {
        id: "mod_07",
        slug: "mod-07",
        title: "03.1 / RSA Key Generation",
        section: "RSA",
        theory_src: "research-docs/julia-crypto/mod_07.md",
        lab_src: "research-docs/julia-crypto/mod_07_lab.md",
    },
    ModuleMeta {
        id: "mod_08",
        slug: "mod-08",
        title: "03.2 / RSA Encryption and Decryption",
        section: "RSA",
        theory_src: "research-docs/julia-crypto/mod_08.md",
        lab_src: "research-docs/julia-crypto/mod_08_lab.md",
    },
];

pub fn find_by_slug(slug: &str) -> Option<&'static ModuleMeta> {
    ALL_MODULES.iter().find(|module| module.slug == slug)
}

pub fn prev_module(slug: &str) -> Option<&'static ModuleMeta> {
    let index = ALL_MODULES.iter().position(|module| module.slug == slug)?;
    index.checked_sub(1).map(|i| &ALL_MODULES[i])
}

pub fn next_module(slug: &str) -> Option<&'static ModuleMeta> {
    let index = ALL_MODULES.iter().position(|module| module.slug == slug)?;
    ALL_MODULES.get(index + 1)
}

pub fn sections() -> &'static [&'static str] {
    &["Number Theory", "Primes", "RSA"]
}

pub fn modules_in_section(section: &'static str) -> impl Iterator<Item = &'static ModuleMeta> {
    ALL_MODULES
        .iter()
        .filter(move |module| module.section == section)
}
