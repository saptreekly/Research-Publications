use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[component]
pub fn AnimatedBackground() -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();

    let state = Rc::new(RefCell::new(AppState {
        mouse_x: 0.0,
        mouse_y: 0.0,
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

                let handle_mousemove = {
                    let state = Rc::clone(&state);
                    Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        let mut s = state.borrow_mut();
                        s.mouse_x = e.client_x() as f64;
                        s.mouse_y = e.client_y() as f64;
                    }) as Box<dyn FnMut(_)>)
                };
                win.add_event_listener_with_callback("mousemove", handle_mousemove.as_ref().unchecked_ref())
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
                
                // JET-BLACK Background
                let bg_color = JsValue::from_str("#000000");
                let dot_color = JsValue::from_str("#a855f7");

                *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                    let s = state_inner.borrow();
                    let time = perf.now();

                    ctx.set_fill_style(&bg_color);
                    ctx.fill_rect(0.0, 0.0, s.width, s.height);

                    ctx.set_fill_style(&dot_color);

                    // Refined Spacing for bigger impact
                    const SPACING: f64 = 32.0; 
                    const DOT_SIZE: f64 = 2.0; // BIGGER DOTS
                    
                    let mut x = 0.0;
                    while x < s.width {
                        let mut y = 0.0;
                        while y < s.height {
                            let dx = x - s.mouse_x;
                            let dy = y - s.mouse_y;
                            let dist_sq = dx * dx + dy * dy;
                            let dist = dist_sq.sqrt();

                            // Refined Wave Mechanics:
                            // dist * 0.01 -> BIGGER RIPPLE (Lower spatial frequency)
                            // time * 0.002 -> LESS FREQUENT (Slower temporal speed)
                            let wave = ((dist * 0.01) - (time * 0.002)).sin();
                            let intensity = ((wave + 1.0) * 0.5).powf(3.0);
                            
                            // BIGGER Warp for more physical deformation
                            let warp = (wave * 12.0) * intensity;
                            let unit_x = if dist > 0.0 { dx / dist } else { 0.0 };
                            let unit_y = if dist > 0.0 { dy / dist } else { 0.0 };
                            
                            let draw_x = x + (unit_x * warp);
                            let draw_y = y + (unit_y * warp);
                            
                            ctx.set_global_alpha(0.1 + (0.9 * intensity));
                            let current_size = DOT_SIZE * (1.0 + intensity * 2.0);
                            
                            ctx.fill_rect(
                                draw_x - (current_size * 0.5), 
                                draw_y - (current_size * 0.5), 
                                current_size, 
                                current_size
                            );

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
                    drop(handle_mousemove);
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
    mouse_x: f64,
    mouse_y: f64,
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
    
    if state.mouse_x == 0.0 && state.mouse_y == 0.0 {
        state.mouse_x = w / 2.0;
        state.mouse_y = h / 2.0;
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
