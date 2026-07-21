use leptos::*;

const STORAGE_KEY: &str = "theme";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Dark,
    /// Retained for API compatibility; the site is dark-only.
    Light,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        "dark"
    }

    pub fn toggle(self) -> Self {
        Theme::Dark
    }

    pub fn from_dom() -> Self {
        Theme::Dark
    }

    pub fn canvas_colors(self) -> (&'static str, &'static str, &'static str) {
        (
            "#0b0d11",
            "#8fa9bc",
            "rgba(143, 169, 188, 0.35)",
        )
    }
}

fn apply_theme(theme: Theme) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };

    if let Some(html) = document.document_element() {
        let _ = html.set_attribute("data-theme", theme.as_str());
    }
}

fn persist_theme(theme: Theme) {
    if let Some(storage) = web_sys::window()
        .and_then(|window| window.local_storage().ok())
        .flatten()
    {
        let _ = storage.set_item(STORAGE_KEY, theme.as_str());
    }
}

pub fn provide_theme() -> RwSignal<Theme> {
    let theme = create_rw_signal(Theme::Dark);

    create_effect(move |_| {
        let current = theme.get();
        apply_theme(current);
        persist_theme(current);
    });

    provide_context(theme);
    theme
}

pub fn use_theme() -> RwSignal<Theme> {
    use_context::<RwSignal<Theme>>().expect("ThemeProvider")
}
