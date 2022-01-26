use js_sys::Error;
use wasm_bindgen::prelude::*;
use web_sys;
use wasm_bindgen::JsCast;


// #[wasm_bindgen]
// extern "C" {
//     type Set;
//
//     # [wasm_bindgen(method)]
//     fn has(this: &Set, element: &JsValue) -> bool;
// }
//
pub fn error_popup() -> Result<(), Error> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().expect("document should have a body");

    let dialog_val = document.create_element("dialog").unwrap().dyn_into::<web_sys::HtmlDialogElement>().unwrap();
    dialog_val.set_title("Error");
    dialog_val.set_inner_text("TAKE A LOOK AT");

    document.body().unwrap().append_child(&dialog_val)?;
    dialog_val.show_modal().unwrap();
    // dialog_val.set_class_name("modal ");
    Ok(())
}