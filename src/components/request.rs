use std::collections::HashMap;
use std::time::Duration;
use js_sys::Map;
use serde_json::{json, Value};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response, RequestMode};

pub async fn get_request(url: &str) -> Result<serde_json::Value, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let resp = send_request(url, &opts,"GET".to_string()).await?;
    Ok(JsFuture::from(resp.json()?).await?.into_serde().unwrap())
}

pub async fn put_request(url: &str, body: &str) -> Result<serde_json::Value, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("PUT");
    let body = JsValue::from_str(&body);
    opts.body(Some(&body));
    let resp = send_request(url, &opts,"PUT".to_string()).await?;
    Ok(JsFuture::from(resp.json()?).await?.into_serde().unwrap())
}

pub async fn post_request(url: &str, body: &str) -> Result<serde_json::Value, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST").mode(RequestMode::Cors);
    // let headers = JsValue::from_str("{'content-type':'Application/json'}");
    // opts.headers(&headers);
    let body = JsValue::from_str(&body);
    opts.body(Some(&body));
    let resp = send_request(url, &opts,"POST".to_string()).await?;
    Ok(JsFuture::from(resp.json()?).await?.into_serde().unwrap())
    
}

async fn send_request(url: &str, opts: &RequestInit, method:String) -> Result<Response, JsValue> {
    let request = Request::new_with_str_and_init(&url, opts)?;
    if method.to_string() == "POST" || method.to_string()== "PUT" {
        log::info!("Method is POST");
        request.headers()
            .set("Content-Type", "application/json").unwrap();
    }

    let window = web_sys::window().unwrap();
    let resp_val = JsFuture::from(window.fetch_with_request(&request)).await?;

    if resp_val.is_instance_of::<Response>() {
        resp_val.dyn_into()
    }
    else { Err(JsValue::from_serde(&json!({
		"err": "response not of type Response" // TODO
	})).unwrap()) }
}

