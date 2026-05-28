use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn ensureKaTeX() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = window)]
    fn ensureLabEditor() -> js_sys::Promise;
}

pub async fn ensure_katex() {
    let _ = JsFuture::from(ensureKaTeX()).await;
}

pub async fn ensure_lab_editor() {
    let _ = JsFuture::from(ensureLabEditor()).await;
}
