use leptos::*;
use std::cell::{Cell, RefCell};
use std::f64::consts::TAU;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::theme::{Theme, use_theme};

struct CanvasColors {
    bg: JsValue,
    ring: JsValue,
    fill: JsValue,
}

impl CanvasColors {
    fn for_theme(theme: Theme) -> Self {
        let (bg, ring, fill) = theme.canvas_colors();
        Self {
            bg: JsValue::from_str(bg),
            ring: JsValue::from_str(ring),
            fill: JsValue::from_str(fill),
        }
    }
}

#[component]
pub fn AnimatedBackground() -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();
    let initialized = store_value(false);
    let theme = use_theme();
    let colors = Rc::new(RefCell::new(CanvasColors::for_theme(theme.get())));

    create_effect({
        let colors = Rc::clone(&colors);
        move |_| {
            *colors.borrow_mut() = CanvasColors::for_theme(theme.get());
        }
    });

    create_effect(move |_| {
        if initialized.get_value() {
            return;
        }

        let Some(canvas) = canvas_ref.get() else {
            return;
        };

        initialized.set_value(true);

        let state = Rc::new(RefCell::new(AppState {
            width: 0.0,
            height: 0.0,
        }));

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let win = window();
        let perf = win.performance().expect("performance required");

        let profile = Rc::new(Cell::new(RenderProfile::for_width(
            win.inner_width().unwrap().as_f64().unwrap_or(1024.0),
        )));
        let initial_profile = profile.get();
        update_dimensions(
            &canvas,
            &ctx,
            &mut state.borrow_mut(),
            initial_profile.max_dpr,
        );

        let running = Rc::new(Cell::new(true));
        let animating = Rc::new(Cell::new(!document().hidden()));
        let frame_id = Rc::new(Cell::new(0i32));
        let frame_counter = Rc::new(Cell::new(0u32));

        let resize_listener = Rc::new(RefCell::new(None::<Closure<dyn FnMut(web_sys::Event)>>));
        {
            let state = Rc::clone(&state);
            let canvas = canvas.clone();
            let ctx = ctx.clone();
            let profile = Rc::clone(&profile);
            let resize_listener = Rc::clone(&resize_listener);
            *resize_listener.borrow_mut() = Some(Closure::wrap(Box::new(
                move |_e: web_sys::Event| {
                    let width = window()
                        .inner_width()
                        .unwrap()
                        .as_f64()
                        .unwrap_or(1024.0);
                    let next_profile = RenderProfile::for_width(width);
                    profile.set(next_profile);
                    update_dimensions(
                        &canvas,
                        &ctx,
                        &mut state.borrow_mut(),
                        next_profile.max_dpr,
                    );
                },
            ) as Box<dyn FnMut(_)>));

            let listener = resize_listener.borrow();
            win.add_event_listener_with_callback(
                "resize",
                listener
                    .as_ref()
                    .expect("resize listener")
                    .as_ref()
                    .unchecked_ref(),
            )
            .expect("resize listener should register");
        }

        let visibility_listener = Rc::new(RefCell::new(None::<Closure<dyn FnMut(web_sys::Event)>>));
        {
            let animating = Rc::clone(&animating);
            let visibility_listener = Rc::clone(&visibility_listener);
            *visibility_listener.borrow_mut() = Some(Closure::wrap(Box::new(
                move |_e: web_sys::Event| {
                    animating.set(!document().hidden());
                },
            ) as Box<dyn FnMut(_)>));

            let listener = visibility_listener.borrow();
            document()
                .add_event_listener_with_callback(
                    "visibilitychange",
                    listener
                        .as_ref()
                        .expect("visibility listener")
                        .as_ref()
                        .unchecked_ref(),
                )
                .expect("visibility listener should register");
        }

        let frame_closure = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
        {
            let state_inner = Rc::clone(&state);
            let frame_loop = Rc::clone(&frame_closure);
            let running_cb = Rc::clone(&running);
            let animating_cb = Rc::clone(&animating);
            let profile_cb = Rc::clone(&profile);
            let frame_id_cb = Rc::clone(&frame_id);
            let frame_counter_cb = Rc::clone(&frame_counter);
            let ctx = ctx.clone();
            let colors_cb = Rc::clone(&colors);

            *frame_closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                if !running_cb.get() {
                    return;
                }

                if animating_cb.get() {
                    let profile = profile_cb.get();
                    let frame = frame_counter_cb.get();
                    frame_counter_cb.set(frame.wrapping_add(1));

                    let should_draw = profile.frame_skip == 0 || frame % (profile.frame_skip + 1) == 0;

                    if should_draw {
                        let s = state_inner.borrow();
                        let time = perf.now();
                        let palette = colors_cb.borrow();

                        ctx.set_global_alpha(1.0);
                        ctx.set_fill_style(&palette.bg);
                        ctx.fill_rect(0.0, 0.0, s.width, s.height);

                        let origin_x = s.width / 2.0;
                        let origin_y = s.height / 2.0;

                        let mut x = 0.0;
                        while x < s.width {
                            let mut y = 0.0;
                            while y < s.height {
                                let dx = x - origin_x;
                                let dy = y - origin_y;
                                let dist_sq = dx * dx + dy * dy;
                                let dist = dist_sq.sqrt();

                                let wave =
                                    ((dist * profile.dist_scale) - (time * profile.time_scale)).sin();
                                let intensity = ((wave + 1.0) * 0.5).powf(profile.intensity_power);

                                if intensity > profile.min_intensity {
                                    let warp = (wave * profile.warp_scale) * intensity;
                                    let unit_x = if dist > 0.0 { dx / dist } else { 0.0 };
                                    let unit_y = if dist > 0.0 { dy / dist } else { 0.0 };

                                    let draw_x = x + (unit_x * warp);
                                    let draw_y = y + (unit_y * warp);
                                    let current_radius =
                                        profile.base_radius * (1.0 + intensity * profile.radius_boost);

                                    ctx.set_global_alpha(intensity * profile.alpha_scale);

                                    if profile.use_fill {
                                        ctx.set_fill_style(&palette.fill);
                                        ctx.begin_path();
                                        let _ = ctx.arc(draw_x, draw_y, current_radius, 0.0, TAU);
                                        ctx.fill();
                                    } else {
                                        ctx.set_stroke_style(&palette.ring);
                                        ctx.set_line_width(1.0);
                                        ctx.begin_path();
                                        let _ = ctx.arc(draw_x, draw_y, current_radius, 0.0, TAU);
                                        ctx.stroke();
                                    }
                                }

                                y += profile.spacing;
                            }
                            x += profile.spacing;
                        }

                        drop(s);
                    }
                }

                if running_cb.get() {
                    if let Some(next) = frame_loop.borrow().as_ref() {
                        if let Ok(id) =
                            window().request_animation_frame(next.as_ref().unchecked_ref())
                        {
                            frame_id_cb.set(id);
                        }
                    }
                }
            }) as Box<dyn FnMut()>));

            let first = frame_closure.borrow();
            if let Some(first) = first.as_ref() {
                if let Ok(id) = win.request_animation_frame(first.as_ref().unchecked_ref()) {
                    frame_id.set(id);
                }
            }
        }

        on_cleanup({
            let resize_listener = Rc::clone(&resize_listener);
            let visibility_listener = Rc::clone(&visibility_listener);
            let frame_closure = Rc::clone(&frame_closure);
            let running = Rc::clone(&running);
            let frame_id = Rc::clone(&frame_id);
            move || {
                running.set(false);

                let id = frame_id.get();
                if id != 0 {
                    let _ = win.cancel_animation_frame(id);
                }

                if let Some(listener) = resize_listener.borrow().as_ref() {
                    let _ = win.remove_event_listener_with_callback(
                        "resize",
                        listener.as_ref().unchecked_ref(),
                    );
                }
                if let Some(listener) = visibility_listener.borrow().as_ref() {
                    let _ = document().remove_event_listener_with_callback(
                        "visibilitychange",
                        listener.as_ref().unchecked_ref(),
                    );
                }
                *resize_listener.borrow_mut() = None;
                *visibility_listener.borrow_mut() = None;
                *frame_closure.borrow_mut() = None;
            }
        });
    });

    view! {
        <canvas node_ref=canvas_ref class="animated-background" />
    }
}

const MOBILE_BREAKPOINT: f64 = 768.0;

#[derive(Clone, Copy)]
struct RenderProfile {
    spacing: f64,
    base_radius: f64,
    warp_scale: f64,
    time_scale: f64,
    dist_scale: f64,
    intensity_power: f64,
    radius_boost: f64,
    alpha_scale: f64,
    min_intensity: f64,
    max_dpr: f64,
    use_fill: bool,
    frame_skip: u32,
}

impl RenderProfile {
    fn for_width(width: f64) -> Self {
        if width < MOBILE_BREAKPOINT {
            Self {
                spacing: 72.0,
                base_radius: 2.75,
                warp_scale: 10.0,
                time_scale: 0.00045,
                dist_scale: 0.012,
                intensity_power: 2.75,
                radius_boost: 2.0,
                alpha_scale: 1.0,
                min_intensity: 0.0,
                max_dpr: 1.5,
                use_fill: false,
                frame_skip: 0,
            }
        } else {
            Self {
                spacing: 64.0,
                base_radius: 3.0,
                warp_scale: 15.0,
                time_scale: 0.0006,
                dist_scale: 0.01,
                intensity_power: 3.0,
                radius_boost: 2.0,
                alpha_scale: 1.0,
                min_intensity: 0.0,
                max_dpr: f64::MAX,
                use_fill: false,
                frame_skip: 0,
            }
        }
    }
}

struct AppState {
    width: f64,
    height: f64,
}

fn update_dimensions(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
    state: &mut AppState,
    max_dpr: f64,
) {
    let win = window();
    let ratio = win.device_pixel_ratio().min(max_dpr);

    let w = win.inner_width().unwrap().as_f64().unwrap();
    let h = win.inner_height().unwrap().as_f64().unwrap();

    canvas.set_width((w * ratio) as u32);
    canvas.set_height((h * ratio) as u32);

    ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
        .expect("Failed to reset transform");
    ctx.scale(ratio, ratio).expect("Failed to scale context");

    state.width = w;
    state.height = h;
}
