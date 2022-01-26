use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use log::log;
use serde_json::Value;

pub async fn start_websocket() -> Result<(), JsValue> {
    // Connect to an echo server
    let ws = WebSocket::new("ws://127.0.0.1:3200")?;
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    // create callback
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        // if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
        //     log::info!("message event, received blob: {:?}", blob);
        //     // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
        //     let fr = web_sys::FileReader::new().unwrap();
        //     let fr_c = fr.clone();
        //     // create onLoadEnd callback
        //     let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
        //         let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
        //         let len = array.byte_length() as usize;
        //         log::info!("Blob received {}bytes: {:?}", len, array.to_vec());
        //         // here you can for example use the received image/png data
        //     })
        //         as Box<dyn FnMut(web_sys::ProgressEvent)>);
        //     fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
        //     fr.read_as_array_buffer(&blob).expect("blob not readable");
        //     onloadend_cb.forget();
        // } else
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            let resp:String = txt.into();
            if resp != "Connected" {
                log::info!("message event, received JSON: {}", resp);

                // resp = resp.clone();
                let response:Value = serde_json::from_str(resp.as_str()).unwrap();
                log::info!("message event, received JSON: {}", response);
            }
        } else {
            log::info!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        log::info!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        log::info!("socket opened");
        match cloned_ws.send_with_str("GET PLATE0") {
            Ok(_) => log::info!("message successfully sent"),
            Err(err) => log::info!("error sending message: {:?}", err),
        }
        // send off binary message
        // match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
        //     Ok(_) => log::info!("binary message successfully sent"),
        //     Err(err) => log::info!("error sending message: {:?}", err),
        // }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}