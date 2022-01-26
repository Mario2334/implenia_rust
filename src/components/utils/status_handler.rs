use wasm_bindgen::JsCast;
use web_sys;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen(module = "bootstrap")]
// extern "C" {
//     #[wasm_bindgen]
//     pub type Modal;
//
//     #[wasm_bindgen(constructor)]
//     pub fn new(e: web_sys::Element) -> Modal;
//
//     #[wasm_bindgen(method)]
//     pub fn toggle(this: &Modal);
//
//     #[wasm_bindgen(method)]
//     pub fn show(this: &Modal);
//
//     #[wasm_bindgen(method)]
//     pub fn hide(this: &Modal);
// }


pub fn error_handler(err: String){
    let document = web_sys::window().unwrap().document().unwrap();
    let error_modal = document.get_element_by_id("alert-modal").unwrap();
    // error_modal
    // let modal =  Modal::new(error_modal);
    // modal.show();
}