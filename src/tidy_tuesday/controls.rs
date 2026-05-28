use leptos::html::Div;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;

#[derive(Clone)]
pub struct YearRangePreset {
    pub start: i32,
    pub end: i32,
    pub label: String,
}

pub fn year_range_presets(min_year: i32, max_year: i32) -> Vec<YearRangePreset> {
    let mid_start = 2000.max(min_year);
    let late_start = (max_year - 10).max(min_year);

    let candidates = [
        (min_year, max_year),
        (mid_start, max_year),
        (late_start, max_year),
    ];

    let mut presets = Vec::new();
    for (start, end) in candidates {
        if end <= start {
            continue;
        }
        if presets
            .iter()
            .any(|preset: &YearRangePreset| preset.start == start && preset.end == end)
        {
            continue;
        }
        presets.push(YearRangePreset {
            start,
            end,
            label: format!("{start}–{end}"),
        });
    }
    presets
}

fn fill_style(min_year: i32, span: f64, start: i32, end: i32) -> String {
    let start_pct = (start - min_year) as f64 / span * 100.0;
    let end_pct = (end - min_year) as f64 / span * 100.0;
    format!("left: {start_pct}%; width: {}%;", end_pct - start_pct)
}

fn clamp_shifted_range(
    min_year: i32,
    max_year: i32,
    orig_start: i32,
    orig_end: i32,
    delta_years: i32,
) -> (i32, i32) {
    let range = orig_end - orig_start;
    let mut new_start = orig_start + delta_years;
    let mut new_end = orig_end + delta_years;

    if new_start < min_year {
        new_start = min_year;
        new_end = min_year + range;
    }
    if new_end > max_year {
        new_end = max_year;
        new_start = max_year - range;
    }

    (new_start, new_end)
}

#[component]
pub fn YearRangePicker(
    min_year: i32,
    max_year: i32,
    start: RwSignal<i32>,
    end: RwSignal<i32>,
    presets: Vec<YearRangePreset>,
) -> impl IntoView {
    let span = (max_year - min_year).max(1) as f64;
    let track_ref = NodeRef::<Div>::new();
    let dragging = RwSignal::new(false);
    let drag_origin = RwSignal::new((0.0_f64, 0_i32, 0_i32));

    let release_pointer = |ev: &PointerEvent| {
        if let Some(target) = ev.current_target() {
            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                let _ = element.release_pointer_capture(ev.pointer_id());
            }
        }
    };

    view! {
        <div class="tt-year-range">
            <div class="tt-year-range-header">
                <span class="tt-control-label">"Year range"</span>
                <span class="tt-year-range-value">
                    {move || format!("{} – {}", start.get(), end.get())}
                </span>
            </div>

            <div
                class="tt-year-range-track"
                class:tt-year-range-dragging=move || dragging.get()
                node_ref=track_ref
            >
                <div
                    class="tt-year-range-fill-handle"
                    style=move || fill_style(min_year, span, start.get(), end.get())
                    on:pointerdown=move |ev: PointerEvent| {
                        ev.prevent_default();
                        dragging.set(true);
                        drag_origin.set((ev.client_x() as f64, start.get(), end.get()));
                        if let Some(target) = ev.current_target() {
                            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                let _ = element.set_pointer_capture(ev.pointer_id());
                            }
                        }
                    }
                    on:pointermove=move |ev: PointerEvent| {
                        if !dragging.get() {
                            return;
                        }

                        let Some(track) = track_ref.get() else {
                            return;
                        };

                        let rect = track.get_bounding_client_rect();
                        if rect.width() <= 0.0 {
                            return;
                        }

                        let (origin_x, orig_start, orig_end) = drag_origin.get();
                        let delta_x = ev.client_x() as f64 - origin_x;
                        let delta_years = ((delta_x / rect.width()) * span).round() as i32;
                        let (new_start, new_end) =
                            clamp_shifted_range(min_year, max_year, orig_start, orig_end, delta_years);
                        start.set(new_start);
                        end.set(new_end);
                    }
                    on:pointerup=move |ev: PointerEvent| {
                        dragging.set(false);
                        release_pointer(&ev);
                    }
                    on:pointercancel=move |ev: PointerEvent| {
                        dragging.set(false);
                        release_pointer(&ev);
                    }
                ></div>
                <input
                    class="tt-year-range-input tt-year-range-start"
                    type="range"
                    min=min_year.to_string()
                    max=max_year.to_string()
                    prop:value=move || start.get().to_string()
                    on:input=move |ev| {
                        let value = event_target_value(&ev).parse::<i32>().unwrap_or(min_year);
                        start.set(value.min(end.get() - 1).max(min_year));
                    }
                />
                <input
                    class="tt-year-range-input tt-year-range-end"
                    type="range"
                    min=min_year.to_string()
                    max=max_year.to_string()
                    prop:value=move || end.get().to_string()
                    on:input=move |ev| {
                        let value = event_target_value(&ev).parse::<i32>().unwrap_or(max_year);
                        end.set(value.max(start.get() + 1).min(max_year));
                    }
                />
            </div>

            <div class="tt-year-range-bounds">
                <span>{min_year}</span>
                <span>{max_year}</span>
            </div>

            {(!presets.is_empty()).then(|| {
                let presets = presets.clone();
                view! {
                <div class="tt-year-range-presets">
                    {presets.into_iter().map(|preset| {
                        let preset_start = preset.start;
                        let preset_end = preset.end;
                        let preset_label = preset.label;
                        let is_active = move || {
                            start.get() == preset_start && end.get() == preset_end
                        };
                        view! {
                            <button
                                type="button"
                                class="tt-chip tt-chip-action"
                                class:tt-chip-active=is_active
                                on:click=move |_| {
                                    start.set(preset_start);
                                    end.set(preset_end);
                                }
                            >
                                {preset_label}
                            </button>
                        }
                    }).collect_view()}
                </div>
            }})}
        </div>
    }
}
