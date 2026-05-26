use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::f64::consts::TAU;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[component]
pub fn AnimatedBackground() -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();

    let state = Rc::new(RefCell::new(AppState {
        width: 0.0,
        height: 0.0,
    }));

    create_effect({
        let state = Rc::clone(&state);
        move |_| {
            if let Some(canvas) = canvas_ref.get() {
                let win = window();
                let perf = win.performance().expect("performance required");
                
                update_dimensions(&canvas, &mut state.borrow_mut());

                let handle_resize = {
                    let state = Rc::clone(&state);
                    let canvas = canvas.clone();
                    Closure::wrap(Box::new(move |_e: web_sys::Event| {
                        update_dimensions(&canvas, &mut state.borrow_mut());
                    }) as Box<dyn FnMut(_)>)
                };
                win.add_event_listener_with_callback("resize", handle_resize.as_ref().unchecked_ref())
                    .unwrap();

                let ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                let f = Rc::new(RefCell::new(None));
                let g = f.clone();

                let state_inner = Rc::clone(&state);
                let f_inner = Rc::clone(&f);
                
                let bg_color = JsValue::from_str("#000000");
                let ring_color = JsValue::from_str("#a855f7");

                *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                    let s = state_inner.borrow();
                    let time = perf.now();

                    ctx.set_fill_style(&bg_color);
                    ctx.fill_rect(0.0, 0.0, s.width, s.height);

                    ctx.set_stroke_style(&ring_color);
                    ctx.set_line_width(1.0);

                    const SPACING: f64 = 40.0; // WIDER spacing
                    const BASE_RADIUS: f64 = 3.0; // BIGGER rings
                    
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

                            let wave = ((dist * 0.008) - (time * 0.0006)).sin();
                            let intensity = ((wave + 1.0) * 0.5).powf(3.0);
                            
                            let warp = (wave * 15.0) * intensity;
                            let unit_x = if dist > 0.0 { dx / dist } else { 0.0 };
                            let unit_y = if dist > 0.0 { dy / dist } else { 0.0 };
                            
                            let draw_x = x + (unit_x * warp);
                            let draw_y = y + (unit_y * warp);
                            
                            ctx.set_global_alpha(0.2 + (0.8 * intensity));
                            let current_radius = BASE_RADIUS * (1.0 + intensity * 2.0);
                            
                            ctx.begin_path();
                            let _ = ctx.arc(draw_x, draw_y, current_radius, 0.0, TAU);
                            ctx.stroke();

                            y += SPACING;
                        }
                        x += SPACING;
                    }

                    request_animation_frame(f_inner.borrow().as_ref().unwrap());
                }) as Box<dyn FnMut()>));

                request_animation_frame(g.borrow().as_ref().unwrap());

                let f_cleanup = Rc::clone(&f);
                on_cleanup(move || {
                    drop(handle_resize);
                    let _ = f_cleanup.borrow_mut().take();
                });
            }
        }
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

fn update_dimensions(canvas: &HtmlCanvasElement, state: &mut AppState) {
    let win = window();
    let w = win.inner_width().unwrap().as_f64().unwrap();
    let h = win.inner_height().unwrap().as_f64().unwrap();
    
    canvas.set_width(w as u32);
    canvas.set_height(h as u32);
    
    state.width = w;
    state.height = h;
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
