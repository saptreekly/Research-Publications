use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = initLabCodeEditor)]
    fn init_lab_code_editor(textarea: &web_sys::HtmlTextAreaElement);

    #[wasm_bindgen(js_namespace = window, js_name = refreshLabCodeEditor)]
    fn refresh_lab_code_editor(textarea: &web_sys::HtmlTextAreaElement);
}

pub fn init(textarea: &web_sys::HtmlTextAreaElement) {
    init_lab_code_editor(textarea);
}

pub fn refresh(textarea: &web_sys::HtmlTextAreaElement) {
    refresh_lab_code_editor(textarea);
}
