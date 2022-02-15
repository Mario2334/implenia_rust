use js_sys::parse_int;

use super::constants::*;
use super::model::Transactions;
use super::request::*;
use super::state::{
    get_contract, get_id, get_material, get_transactions, get_vehicle, get_weight_detail,
};
pub async fn send_first_weight() {
    let weight_detail = get_weight_detail();
    let mut vehicle = get_vehicle();
    let mut id = get_id();
    if vehicle.license_plate != "" {
        let vehicle_id = vehicle.id;
        let url = format!("{}api/Vehicle-View/", API_URL);
        let body = format!(
            "{{\"license_plate\":\"{}\"}}",
            vehicle.license_plate.clone()
        );
        if vehicle_id == 0 {
            // post
            let result = post_request(&url, &body, None).await.unwrap();
            vehicle = serde_json::from_value(result).unwrap();
        } else {
            // put
            let url = format!("{}api/Vehicle-View/{}/", API_URL, vehicle.id);
            let _result = put_request(&url, &body).await.unwrap();
        }

        if id.vehicle.is_none() {
            id.vehicle = Some(vehicle.id);
            let url = format!("{}api/ID/{}/", API_URL, id.id.unwrap());
            let body = format!(
                "{{
                            \"ident\":\"{}\",
                            \"vehicle\":\"{}\"

                    }}",
                id.ident.unwrap(),
                id.vehicle.unwrap()
            );
            let result = put_request(&url, &body).await.unwrap();
            id = serde_json::from_value(result).unwrap();
        }
    }
    let date = format!(
        "20{}-{}-{}",
        &weight_detail.date[6..8],
        &weight_detail.date[3..5],
        &weight_detail.date[0..2]
    );
    let datetime = format!("{}T{}:00", date, weight_detail.time);
    let mut trans = Transactions::default();
    trans.first_weight = Some(weight_detail.clone().weight);
    trans.vehicle = id.vehicle;
    trans.article = Some(get_material().id.unwrap() as i32);
    trans.customer = id.customer;
    trans.supplier = id.supplier;
    trans.net_weight = Some(weight_detail.weight);
    trans.combination_id = id.ident;
    trans.yard = Some(1);
    trans.trans_flag = Some(0);
    trans.firstw_alibi_nr = Some(weight_detail.alibi_nr.to_string());
    trans.firstw_date_time = Some(datetime);
    if ONLY_ID == false {
        let contract = get_contract();
        let mut sup_id: Option<i32> = None;
        if contract.supplier.is_some() && contract.supplier.as_ref().unwrap().len() > 0 {
            sup_id = contract.supplier.unwrap().get(0).unwrap().id;
        }
        trans.forwarders = id.forwarders;
        trans.customer = Some(contract.customer.unwrap().id.unwrap() as i32);
        trans.supplier = sup_id;
        trans.contract_number = Some(contract.contract_number);
    }
    if MULTISCALE == true {
        trans.scale_nr = Some(SCALE);
    }
    let url = format!("{}api/Transactions/", API_URL);
    let body = serde_json::to_string(&trans).unwrap();
    let _result = post_request(&url, &body, None).await;
}

pub async fn send_second_weight() {
    let weight_detail = get_weight_detail();

    let date = format!(
        "20{}-{}-{}",
        &weight_detail.date[6..8],
        &weight_detail.date[3..5],
        &weight_detail.date[0..2]
    );
    let datetime = format!("{}T{}:00", date, weight_detail.time);
    let old_trans = get_transactions();
    let mut trans = old_trans.clone();
    trans.second_weight = Some(weight_detail.weight.clone());
    let mut net_weight =
        parse_int(&weight_detail.weight, 10) - parse_int(&old_trans.first_weight.unwrap(), 10);
    if net_weight < 0 as f64 {
        net_weight *= -1 as f64;
    }
    trans.net_weight = Some(net_weight.to_string());
    trans.trans_flag = Some(1);
    trans.yard = Some(1);
    trans.secondw_alibi_nr = Some(weight_detail.alibi_nr.to_string());
    trans.secondw_date_time = Some(datetime);
    let url = format!("{}api/Transactions/{}/", API_URL, old_trans.id.unwrap());
    let body = serde_json::to_string(&trans).unwrap();
    put_request(&url, &body).await.unwrap();
}

pub async fn send_tara_weight() {}

pub async fn send_tara_save_weight() {}
