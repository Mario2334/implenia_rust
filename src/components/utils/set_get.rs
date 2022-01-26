use js_sys::Math::log;
use crate::components::model::Transactions;

pub fn set_lang (val:  &str) {
    // unsafe { LANG = val };
    let window = web_sys::window().unwrap();
    if let Ok(Some(local_storage)) = window.local_storage(){
        local_storage.set_item("set_lang",val).unwrap();
    }
}


pub fn get_lang() -> String {
    let window = web_sys::window().unwrap();
    if let Ok(Some(local_storage)) = window.local_storage(){
        if let Ok(Some(value)) = local_storage.get_item("set_lang"){
            return value;
        }
        else {
            let lang = "de";
            local_storage.set_item("set_lang",lang).unwrap();
            return lang.to_string().clone();
        }
    }
    else {
        return "de".to_string();
    }
}


pub fn set_barcode (val:  &str) {
    let window = web_sys::window().unwrap();
    if let Ok(Some(local_storage)) = window.local_storage(){
        local_storage.set_item("barcode_number",val).unwrap();
    }
}


pub fn get_barcode() -> String {
    let window = web_sys::window().unwrap();
    if let Ok(Some(local_storage)) = window.local_storage(){
        if let Ok(Some(value)) = local_storage.get_item("barcode_number"){
            return value;
        }
        else {
            return "12345".to_string();
        }
    }
    else {
        return "12345".to_string();
    }
}

pub fn get_net_weight(weight:String,transactions:Transactions) -> String{
    let mut trimmed_weight = weight.clone();
    trimmed_weight = trimmed_weight.trim().to_string();
    let sw_val:i64 = trimmed_weight.parse().unwrap();
    let first_weight = transactions.first_weight.unwrap();
    log::info!("{}",first_weight);
    let fw_val:i64 = first_weight.trim().parse().unwrap();
    let net_weight = sw_val - fw_val;
    let string_net= net_weight.to_string();
    return string_net
}

