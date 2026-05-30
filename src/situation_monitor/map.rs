use std::collections::HashMap;

use js_sys::{Function, Object, Reflect};
use leptos::*;
use serde::Serialize;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::utils::script_loader;

use super::{region_label, FeedItem, ALL_REGION};

#[derive(Clone, Copy)]
pub struct RegionPin {
    pub id: &'static str,
    pub lat: f64,
    pub lon: f64,
}

pub const REGION_PINS: &[RegionPin] = &[
    RegionPin { id: "us", lat: 39.0, lon: -98.0 },
    RegionPin { id: "europe", lat: 50.0, lon: 10.0 },
    RegionPin { id: "middle-east", lat: 30.0, lon: 45.0 },
    RegionPin { id: "africa", lat: 0.0, lon: 20.0 },
    RegionPin { id: "russia", lat: 60.0, lon: 90.0 },
    RegionPin { id: "india", lat: 22.0, lon: 78.0 },
    RegionPin { id: "china", lat: 35.0, lon: 105.0 },
    RegionPin { id: "taiwan", lat: 24.0, lon: 121.0 },
    RegionPin { id: "japan", lat: 36.0, lon: 138.0 },
    RegionPin { id: "korea", lat: 37.0, lon: 127.0 },
    RegionPin { id: "se-asia", lat: 10.0, lon: 105.0 },
    RegionPin { id: "pacific", lat: -10.0, lon: 160.0 },
    RegionPin { id: "australia", lat: -25.0, lon: 133.0 },
    RegionPin { id: "nz", lat: -42.0, lon: 174.0 },
    RegionPin { id: "global", lat: 72.0, lon: 0.0 },
];

#[derive(Serialize)]
struct PinDef {
    id: &'static str,
    label: &'static str,
    lat: f64,
    lon: f64,
}

fn situation_map_api() -> Result<Object, String> {
    let window = web_sys::window().ok_or("window unavailable")?;
    Reflect::get(&window, &JsValue::from_str("SituationMap"))
        .map_err(|_| "SituationMap unavailable".to_string())?
        .dyn_into::<Object>()
        .map_err(|_| "SituationMap invalid".to_string())
}

fn call_map_method(
    api: &Object,
    method: &str,
    args: &[JsValue],
) -> Result<(), String> {
    let function = Reflect::get(api, &JsValue::from_str(method))
        .map_err(|_| format!("SituationMap.{method} missing"))?
        .dyn_into::<Function>()
        .map_err(|_| format!("SituationMap.{method} invalid"))?;
    function
        .apply(api, &js_sys::Array::from_iter(args.iter().cloned()))
        .map_err(|_| format!("SituationMap.{method} failed"))?;
    Ok(())
}

fn pin_defs() -> Vec<PinDef> {
    REGION_PINS
        .iter()
        .map(|pin| PinDef {
            id: pin.id,
            label: region_label(pin.id),
            lat: pin.lat,
            lon: pin.lon,
        })
        .collect()
}

fn to_js_value<T: Serialize>(value: &T) -> Result<JsValue, String> {
    let json = serde_json::to_string(value).map_err(|err| err.to_string())?;
    js_sys::JSON::parse(&json).map_err(|_| "failed to parse json".to_string())
}

fn dom_to_js(el: leptos::HtmlElement<leptos::html::Div>) -> JsValue {
    let element: &web_sys::Element = el.as_ref();
    JsValue::from(element)
}

#[component]
pub fn WorldMap(
    region_counts: Memo<HashMap<String, usize>>,
    region_previews: Memo<HashMap<String, Vec<FeedItem>>>,
    active_region: ReadSignal<Option<String>>,
    on_select: impl Fn(String) + Clone + 'static,
) -> impl IntoView {
    let map_ref = create_node_ref::<html::Div>();
    let (hovered_region, set_hovered_region) = create_signal(None::<String>);
    let (map_ready, set_map_ready) = create_signal(false);
    let (map_error, set_map_error) = create_signal(None::<String>);
    let (mount_gen, set_mount_gen) = create_signal(0_u32);
    let last_flown = StoredValue::new(None::<String>);
    let on_select = StoredValue::new(on_select);

    create_effect(move |_| {
        let Some(el) = map_ref.get() else {
            return;
        };

        let select_handler = on_select.get_value();
        let generation = mount_gen.get_untracked().wrapping_add(1);
        set_mount_gen.set(generation);

        spawn_local(async move {
            script_loader::ensure_situation_map().await;

            if mount_gen.get_untracked() != generation {
                return;
            }

            let pins = pin_defs();
            let pins_js = match to_js_value(&pins) {
                Ok(value) => value,
                Err(message) => {
                    set_map_error.set(Some(message));
                    return;
                }
            };

            let callbacks = Object::new();
            let select_closure = Closure::wrap(Box::new(move |region: JsValue| {
                if let Some(value) = region.as_string() {
                    select_handler(value);
                }
            }) as Box<dyn Fn(JsValue)>);
            let hover_closure = Closure::wrap(Box::new(move |region: JsValue| {
                set_hovered_region.set(region.as_string());
            }) as Box<dyn Fn(JsValue)>);

            let _ = Reflect::set(
                &callbacks,
                &JsValue::from_str("onSelect"),
                select_closure.as_ref(),
            );
            let _ = Reflect::set(
                &callbacks,
                &JsValue::from_str("onHover"),
                hover_closure.as_ref(),
            );
            select_closure.forget();
            hover_closure.forget();

            if mount_gen.get_untracked() != generation {
                return;
            }

            match situation_map_api() {
                Ok(api) => {
                    if call_map_method(
                        &api,
                        "mount",
                        &[dom_to_js(el), pins_js, callbacks.into()],
                    )
                    .is_err()
                    {
                        set_map_error.set(Some("Unable to initialize map".to_string()));
                        return;
                    }
                    set_map_error.set(None);
                    set_map_ready.set(true);
                }
                Err(message) => set_map_error.set(Some(message)),
            }
        });

        on_cleanup(move || {
            set_mount_gen.update(|gen| *gen = gen.wrapping_add(1));
            set_map_ready.set(false);
            if let (Ok(api), Some(el)) = (situation_map_api(), map_ref.get_untracked()) {
                let _ = call_map_method(&api, "unmount", &[dom_to_js(el)]);
            }
        });
    });

    create_effect(move |_| {
        if !map_ready.get() {
            return;
        }
        let Some(el) = map_ref.get() else {
            return;
        };

        let counts = region_counts.get();
        let active = active_region.get();

        if let (Ok(api), Ok(counts_js)) = (situation_map_api(), to_js_value(&counts)) {
            let active_js = active
                .as_deref()
                .map(JsValue::from_str)
                .unwrap_or(JsValue::NULL);
            let _ = call_map_method(
                &api,
                "update",
                &[dom_to_js(el), counts_js, active_js],
            );
        }
    });

    create_effect(move |_| {
        if !map_ready.get() {
            return;
        }
        let Some(el) = map_ref.get() else {
            return;
        };

        let hovered = hovered_region.get();

        if let Ok(api) = situation_map_api() {
            let hovered_js = hovered
                .as_deref()
                .map(JsValue::from_str)
                .unwrap_or(JsValue::NULL);
            let _ = call_map_method(
                &api,
                "setHover",
                &[dom_to_js(el), hovered_js],
            );
        }
    });

    create_effect(move |_| {
        if !map_ready.get() {
            return;
        }
        let Some(region) = active_region.get() else {
            last_flown.set_value(None);
            return;
        };
        if region == ALL_REGION {
            return;
        }
        if last_flown.with_value(|last| last.as_deref() == Some(region.as_str())) {
            return;
        }
        last_flown.set_value(Some(region.clone()));
        let Some(el) = map_ref.get() else {
            return;
        };
        if let Ok(api) = situation_map_api() {
            let _ = call_map_method(
                &api,
                "flyTo",
                &[dom_to_js(el), JsValue::from_str(&region)],
            );
        }
    });

    let focus_region = create_memo(move |_| hovered_region.get().or_else(|| active_region.get()));
    let keep_hover = move |region: Option<String>| set_hovered_region.set(region);

    view! {
        <div class="sm-map-wrap">
            <div class="sm-map-toolbar">
                <span class="sm-map-toolbar-label">"OpenStreetMap · drag, scroll, or double-click pins · +/- keys"</span>
                <div class="sm-map-toolbar-actions">
                    <button
                        type="button"
                        class="sm-map-zoom-btn"
                        aria-label="Zoom in"
                        on:click=move |_| {
                            if let (Ok(api), Some(el)) = (situation_map_api(), map_ref.get()) {
                                let _ = call_map_method(&api, "zoomIn", &[dom_to_js(el)]);
                            }
                        }
                    >
                        "+"
                    </button>
                    <button
                        type="button"
                        class="sm-map-zoom-btn"
                        aria-label="Zoom out"
                        on:click=move |_| {
                            if let (Ok(api), Some(el)) = (situation_map_api(), map_ref.get()) {
                                let _ = call_map_method(&api, "zoomOut", &[dom_to_js(el)]);
                            }
                        }
                    >
                        "−"
                    </button>
                    <button
                        type="button"
                        class="sm-map-zoom-btn sm-map-reset-btn"
                        on:click=move |_| {
                            if let (Ok(api), Some(el)) = (situation_map_api(), map_ref.get()) {
                                let _ = call_map_method(&api, "resetView", &[dom_to_js(el)]);
                            }
                        }
                    >
                        "Reset"
                    </button>
                </div>
            </div>

            <div
                class="sm-map-stage sm-map-stage-osm"
                tabindex="0"
                on:keydown=move |ev: web_sys::KeyboardEvent| {
                    let Some(el) = map_ref.get() else { return };
                    let Ok(api) = situation_map_api() else { return };
                    match ev.key().as_str() {
                        "+" | "=" => { let _ = call_map_method(&api, "zoomIn", &[dom_to_js(el)]); }
                        "-" | "_" => { let _ = call_map_method(&api, "zoomOut", &[dom_to_js(el)]); }
                        "0" => { let _ = call_map_method(&api, "resetView", &[dom_to_js(el)]); }
                        "Escape" => on_select.with_value(|select| select(ALL_REGION.to_string())),
                        _ => return,
                    }
                    ev.prevent_default();
                }
            >
                <div node_ref=map_ref class="sm-map-leaflet" aria-label="OpenStreetMap regional activity"></div>

                <div class="sm-map-loading" class:sm-map-loading-hidden=move || map_ready.get() || map_error.get().is_some()>
                    <span class="sm-map-loading-spinner" aria-hidden="true"></span>
                    <span>"Loading map…"</span>
                </div>

                {move || map_error.get().map(|message| view! {
                    <p class="sm-map-error" role="alert">{message}</p>
                })}

                <MapPopover
                    region=focus_region
                    region_counts=region_counts
                    region_previews=region_previews
                    active_region=active_region
                    on_select=on_select
                    on_keep_hover=keep_hover
                />
            </div>

            <p class="sm-map-credit">
                "Map tiles "
                <a href="https://www.openstreetmap.org/copyright" target="_blank" rel="noopener noreferrer">
                    "© OpenStreetMap contributors"
                </a>
            </p>

            <div class="sm-map-chips" role="listbox" aria-label="Filter by region">
                <button
                    type="button"
                    class="sm-map-chip"
                    class:sm-map-chip-active=move || active_region.get().is_none()
                    on:click=move |_| on_select.with_value(|select| select(ALL_REGION.to_string()))
                    on:pointerenter=move |_| set_hovered_region.set(None)
                >
                    "All regions"
                </button>
                {REGION_PINS.iter().filter(|pin| pin.id != "global").map(|pin| {
                    let active_check = pin.id.to_string();
                    let click_select = pin.id.to_string();
                    let hover_check = pin.id.to_string();
                    let hover_enter = pin.id.to_string();
                    let hover_focus = pin.id.to_string();
                    let label = region_label(pin.id);
                    view! {
                        <button
                            type="button"
                            class="sm-map-chip"
                            class:sm-map-chip-active=move || active_region.get().as_deref() == Some(active_check.as_str())
                            class:sm-map-chip-hover=move || hovered_region.get().as_deref() == Some(hover_check.as_str())
                            on:click=move |_| on_select.with_value(|select| select(click_select.to_string()))
                            on:pointerenter=move |_| set_hovered_region.set(Some(hover_enter.to_string()))
                            on:focus=move |_| set_hovered_region.set(Some(hover_focus.to_string()))
                        >
                            <span>{label}</span>
                            <span class="sm-map-chip-count">
                                {move || region_counts.get().get(pin.id).copied().unwrap_or(0)}
                            </span>
                        </button>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn MapPopover(
    region: Memo<Option<String>>,
    region_counts: Memo<HashMap<String, usize>>,
    region_previews: Memo<HashMap<String, Vec<FeedItem>>>,
    active_region: ReadSignal<Option<String>>,
    on_select: StoredValue<impl Fn(String) + Clone + 'static>,
    on_keep_hover: impl Fn(Option<String>) + 'static + Clone,
) -> impl IntoView {
    let keep_hover = StoredValue::new(on_keep_hover);
    let label = move || {
        region
            .get()
            .as_deref()
            .map(region_label)
            .unwrap_or("Global")
    };
    let count = move || {
        region
            .get()
            .as_ref()
            .and_then(|id| region_counts.get().get(id).copied())
            .unwrap_or(0)
    };
    let items = move || {
        region
            .get()
            .as_ref()
            .and_then(|id| region_previews.get().get(id).cloned())
            .unwrap_or_default()
    };
    let is_active = move || {
        region.get().is_some_and(|id| active_region.get().as_deref() == Some(id.as_str()))
    };
    let region_for_hover = region.clone();

    view! {
        <aside
            class="sm-map-popover"
            class:sm-map-popover-visible=move || region.get().is_some()
            aria-hidden=move || region.get().is_none()
            on:pointerenter=move |_| {
                keep_hover.with_value(|keep| keep(region_for_hover.get()));
            }
        >
            {move || region.get().map(|region_id| view! {
                <div class="sm-map-popover-head">
                    <div>
                        <strong>{label()}</strong>
                        <span class="sm-map-popover-count">{format!("{} headlines", count())}</span>
                    </div>
                    {if is_active() {
                        view! {
                            <button
                                type="button"
                                class="sm-map-popover-filter sm-map-popover-clear"
                                on:click=move |_| on_select.with_value(|select| select(ALL_REGION.to_string()))
                            >
                                "Clear filter"
                            </button>
                        }.into_view()
                    } else {
                        let filter_id = region_id.clone();
                        view! {
                            <button
                                type="button"
                                class="sm-map-popover-filter"
                                on:click=move |_| on_select.with_value(|select| select(filter_id.clone()))
                            >
                                "Filter feed"
                            </button>
                        }.into_view()
                    }}
                </div>
                {if items().is_empty() {
                    view! {
                        <p class="sm-map-popover-empty">"No headlines tagged to this region yet."</p>
                    }.into_view()
                } else {
                    view! {
                        <ul class="sm-map-popover-list">
                            {items().into_iter().map(|item| view! {
                                <li>
                                    <a href=item.url.clone() target="_blank" rel="noopener noreferrer">
                                        {item.title.clone()}
                                    </a>
                                    <span class="sm-map-popover-meta">{item.source_name.clone()}</span>
                                </li>
                            }).collect_view()}
                        </ul>
                    }.into_view()
                }}
            })}
        </aside>
    }
}
