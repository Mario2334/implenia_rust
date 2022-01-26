use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    type SignaturePad;

    #[wasm_bindgen(constructor)]
    fn new() -> SignaturePad;
}
