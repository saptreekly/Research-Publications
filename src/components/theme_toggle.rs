use leptos::*;

use crate::theme::{Theme, use_theme};

#[component]
fn SunIcon() -> impl IntoView {
    view! {
        <svg class="theme-toggle-icon" viewBox="0 0 24 24" aria-hidden="true">
            <circle cx="12" cy="12" r="4.25" fill="currentColor" />
            <path
                fill="currentColor"
                d="M12 1.75a.75.75 0 0 1 .75.75V5a.75.75 0 0 1-1.5 0V2.5A.75.75 0 0 1 12 1.75Zm0 15a.75.75 0 0 1 .75.75v2.75a.75.75 0 0 1-1.5 0V17.5A.75.75 0 0 1 12 16.75ZM4.22 4.22a.75.75 0 0 1 1.06 0l1.94 1.94a.75.75 0 1 1-1.06 1.06L4.22 5.28a.75.75 0 0 1 0-1.06Zm13.56 13.56a.75.75 0 0 1 1.06 0l1.94 1.94a.75.75 0 0 1-1.06 1.06l-1.94-1.94a.75.75 0 0 1 0-1.06ZM1.75 12a.75.75 0 0 1 .75-.75H5a.75.75 0 0 1 0 1.5H2.5A.75.75 0 0 1 1.75 12Zm17.5 0a.75.75 0 0 1 .75-.75h2.75a.75.75 0 0 1 0 1.5H20a.75.75 0 0 1-.75-.75ZM6.16 17.84a.75.75 0 0 1 1.06 0l1.94 1.94a.75.75 0 1 1-1.06 1.06l-1.94-1.94a.75.75 0 0 1 0-1.06Zm11.68-11.68a.75.75 0 0 1 1.06 0l1.94 1.94a.75.75 0 0 1-1.06 1.06l-1.94-1.94a.75.75 0 0 1 0-1.06Z"
            />
        </svg>
    }
}

#[component]
fn MoonIcon() -> impl IntoView {
    view! {
        <svg class="theme-toggle-icon" viewBox="0 0 24 24" aria-hidden="true">
            <path
                fill="currentColor"
                d="M21 12.79A9 9 0 1 1 11.21 3a7 7 0 0 0 9.79 9.79Z"
            />
        </svg>
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = use_theme();

    view! {
        <button
            type="button"
            class="theme-toggle"
            aria-label=move || {
                if theme.get() == Theme::Light {
                    "Switch to dark mode"
                } else {
                    "Switch to light mode"
                }
            }
            title=move || {
                if theme.get() == Theme::Light {
                    "Switch to dark mode"
                } else {
                    "Switch to light mode"
                }
            }
            on:click=move |_| theme.update(|current| *current = current.toggle())
        >
            <Show when=move || theme.get() == Theme::Dark fallback=MoonIcon>
                <SunIcon />
            </Show>
        </button>
    }
}
