use leptos::*;
use leptos::html::Div;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

struct ObserverState {
    observer: Option<IntersectionObserver>,
    closure: Option<Closure<dyn FnMut(js_sys::Array)>>,
}

#[component]
pub fn LazySection(
    children: ChildrenFn,
    #[prop(default = "320px 0px 320px 0px")] root_margin: &'static str,
    #[prop(default = 280_u32)] min_height: u32,
    #[prop(default = false)] release_when_hidden: bool,
    #[prop(optional)] on_visible: Option<Callback<()>>,
) -> impl IntoView {
    let container_ref = create_node_ref::<Div>();
    let visible = create_rw_signal(false);
    let state = Rc::new(RefCell::new(ObserverState {
        observer: None,
        closure: None,
    }));

    create_effect({
        let state = Rc::clone(&state);
        move |_| {
            let Some(element) = container_ref.get() else {
                return;
            };

            if state.borrow().observer.is_some() {
                return;
            }

            let visible = visible;
            let on_visible = on_visible;
            let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                let entry_value = entries.get(0);
                let Some(entry) = entry_value.dyn_ref::<IntersectionObserverEntry>() else {
                    return;
                };

                if entry.is_intersecting() {
                    visible.set(true);
                    if let Some(handler) = on_visible {
                        handler.call(());
                    }
                } else if release_when_hidden {
                    visible.set(false);
                }
            }) as Box<dyn FnMut(js_sys::Array)>);

            let options = IntersectionObserverInit::new();
            options.set_root_margin(root_margin);
            options.set_threshold(&js_sys::Array::of1(&JsValue::from(0.01)));

            let observer = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &options,
            )
            .expect("intersection observer should initialize");

            observer.observe(&element);

            let mut guard = state.borrow_mut();
            guard.observer = Some(observer);
            guard.closure = Some(callback);
        }
    });

    on_cleanup({
        let state = Rc::clone(&state);
        move || {
            let mut guard = state.borrow_mut();
            if let Some(observer) = guard.observer.take() {
                observer.disconnect();
            }
            guard.closure = None;
        }
    });

    let placeholder_style = format!("min-height: {min_height}px");

    view! {
        <div class="lazy-section" node_ref=container_ref>
            {move || {
                if visible.get() {
                    children().into_view()
                } else {
                    view! {
                        <div class="lazy-section-placeholder" style=placeholder_style.clone() aria-hidden="true" />
                    }
                    .into_view()
                }
            }}
        </div>
    }
}
