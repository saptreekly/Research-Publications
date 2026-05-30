use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = initLabCodeEditor)]
    fn init_lab_code_editor(textarea: &web_sys::HtmlTextAreaElement);

    #[wasm_bindgen(js_namespace = window, js_name = refreshLabCodeEditor)]
    fn refresh_lab_code_editor(textarea: &web_sys::HtmlTextAreaElement);
}

pub fn init(textarea: &web_sys::HtmlTextAreaElement) {
    let element = textarea.clone();
    leptos::spawn_local(async move {
        crate::utils::script_loader::ensure_lab_editor().await;
        init_lab_code_editor(&element);
        refresh_lab_code_editor(&element);
    });
}

pub fn refresh(textarea: &web_sys::HtmlTextAreaElement) {
    if js_sys::Reflect::has(&js_sys::global(), &JsValue::from_str("refreshLabCodeEditor"))
        .unwrap_or(false)
    {
        refresh_lab_code_editor(textarea);
    }
}
