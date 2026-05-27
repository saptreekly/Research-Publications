use leptos::*;
use std::cell::{Cell, RefCell};
use std::f64::consts::TAU;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[component]
pub fn AnimatedBackground() -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();
    let initialized = store_value(false);

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

        update_dimensions(&canvas, &ctx, &mut state.borrow_mut());

        let running = Rc::new(Cell::new(true));
        let frame_id = Rc::new(Cell::new(0i32));

        let resize_listener = Rc::new(RefCell::new(None::<Closure<dyn FnMut(web_sys::Event)>>));
        {
            let state = Rc::clone(&state);
            let canvas = canvas.clone();
            let ctx = ctx.clone();
            let resize_listener = Rc::clone(&resize_listener);
            *resize_listener.borrow_mut() = Some(Closure::wrap(Box::new(
                move |_e: web_sys::Event| {
                    update_dimensions(&canvas, &ctx, &mut state.borrow_mut());
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

        let frame_closure = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
        {
            let state_inner = Rc::clone(&state);
            let frame_loop = Rc::clone(&frame_closure);
            let running_cb = Rc::clone(&running);
            let frame_id_cb = Rc::clone(&frame_id);
            let ctx = ctx.clone();
            let bg_color = JsValue::from_str("#000000");
            let ring_color = JsValue::from_str("#a855f7");

            *frame_closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                if !running_cb.get() {
                    return;
                }

                let s = state_inner.borrow();
                let time = perf.now();

                ctx.set_global_alpha(1.0);
                ctx.set_fill_style(&bg_color);
                ctx.fill_rect(0.0, 0.0, s.width, s.height);

                ctx.set_stroke_style(&ring_color);
                ctx.set_line_width(1.0);

                const SPACING: f64 = 50.0;
                const BASE_RADIUS: f64 = 3.0;

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

                        let wave = ((dist * 0.01) - (time * 0.0006)).sin();
                        let intensity = ((wave + 1.0) * 0.5).powf(3.0);

                        let warp = (wave * 15.0) * intensity;
                        let unit_x = if dist > 0.0 { dx / dist } else { 0.0 };
                        let unit_y = if dist > 0.0 { dy / dist } else { 0.0 };

                        let draw_x = x + (unit_x * warp);
                        let draw_y = y + (unit_y * warp);

                        ctx.set_global_alpha(intensity);
                        let current_radius = BASE_RADIUS * (1.0 + intensity * 2.0);

                        ctx.begin_path();
                        let _ = ctx.arc(draw_x, draw_y, current_radius, 0.0, TAU);
                        ctx.stroke();

                        y += SPACING;
                    }
                    x += SPACING;
                }

                drop(s);

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
                *resize_listener.borrow_mut() = None;
                *frame_closure.borrow_mut() = None;
            }
        });
    });

    view! {
        <canvas
            node_ref=canvas_ref
            style="position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: -1; pointer-events: none;"
        />
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
) {
    let win = window();
    let ratio = win.device_pixel_ratio();

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
