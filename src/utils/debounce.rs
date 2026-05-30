use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Debounces a numeric signal for chart recomputation while sliders drag.
pub fn debounced_i32(source: RwSignal<i32>, delay_ms: i32) -> ReadSignal<i32> {
    let (debounced, set_debounced) = create_signal(source.get_untracked());
    let timeout_id = Rc::new(RefCell::new(None::<i32>));

    create_effect(move |_| {
        let value = source.get();
        let window = web_sys::window().expect("window");

        if let Some(id) = timeout_id.borrow_mut().take() {
            window.clear_timeout_with_handle(id);
        }

        let timeout_id_in_closure = Rc::clone(&timeout_id);
        let closure = Closure::once(Box::new(move || {
            set_debounced.set(value);
            *timeout_id_in_closure.borrow_mut() = None;
        }) as Box<dyn FnOnce()>);

        let id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                delay_ms,
            )
            .expect("timeout should schedule");
        *timeout_id.borrow_mut() = Some(id);
        closure.forget();
    });

    debounced.into()
}

pub fn debounced_usize(source: RwSignal<usize>, delay_ms: i32) -> ReadSignal<usize> {
    let (debounced, set_debounced) = create_signal(source.get_untracked());
    let timeout_id = Rc::new(RefCell::new(None::<i32>));

    create_effect(move |_| {
        let value = source.get();
        let window = web_sys::window().expect("window");

        if let Some(id) = timeout_id.borrow_mut().take() {
            window.clear_timeout_with_handle(id);
        }

        let timeout_id_in_closure = Rc::clone(&timeout_id);
        let closure = Closure::once(Box::new(move || {
            set_debounced.set(value);
            *timeout_id_in_closure.borrow_mut() = None;
        }) as Box<dyn FnOnce()>);

        let id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                delay_ms,
            )
            .expect("timeout should schedule");
        *timeout_id.borrow_mut() = Some(id);
        closure.forget();
    });

    debounced.into()
}

/// Debounces a string signal (e.g. search input) before downstream filtering.
pub fn debounced_string(source: ReadSignal<String>, delay_ms: i32) -> ReadSignal<String> {
    let (debounced, set_debounced) = create_signal(source.get_untracked());
    let timeout_id = Rc::new(RefCell::new(None::<i32>));

    create_effect(move |_| {
        let value = source.get();
        let window = web_sys::window().expect("window");

        if let Some(id) = timeout_id.borrow_mut().take() {
            window.clear_timeout_with_handle(id);
        }

        let timeout_id_in_closure = Rc::clone(&timeout_id);
        let closure = Closure::once(Box::new(move || {
            set_debounced.set(value);
            *timeout_id_in_closure.borrow_mut() = None;
        }) as Box<dyn FnOnce()>);

        let id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                delay_ms,
            )
            .expect("timeout should schedule");
        *timeout_id.borrow_mut() = Some(id);
        closure.forget();
    });

    debounced.into()
}
