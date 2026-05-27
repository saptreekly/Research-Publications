use leptos::*;

const STORAGE_KEY: &str = "theme";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }

    pub fn from_dom() -> Self {
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
            .and_then(|html| html.get_attribute("data-theme"))
            .map(|value| {
                if value == "light" {
                    Theme::Light
                } else {
                    Theme::Dark
                }
            })
            .unwrap_or(Theme::Dark)
    }

    pub fn canvas_colors(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Theme::Dark => ("#000000", "#a855f7", "rgba(168, 85, 247, 0.85)"),
            Theme::Light => ("#f5f4f0", "#9333ea", "rgba(147, 51, 234, 0.55)"),
        }
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
    let theme = create_rw_signal(Theme::from_dom());

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
