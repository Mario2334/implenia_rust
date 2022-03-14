use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    pub name: Option<String>,
    pub id: Option<i8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Supplier {
    pub supplier_name: Option<String>,
    pub id: Option<i32>,
}
impl Default for Supplier {
    fn default() -> Self {
        Self {
            supplier_name: None,
            id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContractMaterial {
    material: Option<Material>,
    agreed_value: Option<i64>,
    remaining: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Material {
    pub id: Option<i8>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contract {
    pub name: String,
    pub contract_number: String,
    pub customer: Option<Customer>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub reserved_date: Option<String>,
    pub required_materials: Vec<ContractMaterial>,
    pub supplier: Option<Vec<Supplier>>,
}

impl Default for Contract {
    fn default() -> Self {
        Contract {
            name: "".to_string(),
            contract_number: "".to_string(),
            customer: None,
            start_date: None,
            end_date: None,
            reserved_date: None,
            required_materials: vec![],
            supplier: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LicensePlateResponse {
    pub msg_type: String,
    pub state: String,
    pub license_plate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WeightResponse {
    pub msg_type: String,
    pub state: String,
    pub alibi_nr: String,
    pub weight: String,
    pub date: String,
    pub time: String,
}

impl Default for WeightResponse {
    fn default() -> Self {
        WeightResponse {
            msg_type: "".to_string(),
            state: "".to_string(),
            alibi_nr: "".to_string(),
            weight: "".to_string(),
            date: "".to_string(),
            time: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TransactionPDFRequest {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Transactions {
    pub id: Option<i32>,
    pub combination_id: Option<String>,
    pub first_weight: Option<String>,
    pub second_weight: Option<String>,
    pub net_weight: Option<String>,
    pub total_price: Option<String>,
    pub firstw_date_time: Option<String>,
    pub secondw_date_time: Option<String>,
    pub firstw_alibi_nr: Option<String>,
    pub secondw_alibi_nr: Option<String>,
    pub vehicle_weight_flag: Option<i32>,
    pub vehicle_second_weight_flag: Option<i32>,
    pub trans_flag: Option<i32>,
    pub price_per_item: Option<String>,
    pub status: Option<String>,
    pub vehicle: Option<i32>,
    pub article: Option<i32>,
    pub customer: Option<i32>,
    pub forwarders: Option<i32>,
    pub supplier: Option<i32>,
    pub container: Option<i32>,
    pub yard: Option<i32>,
    pub contract_number: Option<String>,
    pub created_date_time: Option<String>,
    pub updated_date_time: Option<String>,
    pub scale_nr: Option<i32>,
}

impl Default for Transactions {
    fn default() -> Self {
        Transactions {
            id: None,
            combination_id: None,
            first_weight: None,
            second_weight: None,
            net_weight: None,
            total_price: None,
            firstw_date_time: None,
            secondw_date_time: None,
            firstw_alibi_nr: None,
            secondw_alibi_nr: None,
            vehicle_weight_flag: None,
            vehicle_second_weight_flag: None,
            trans_flag: None,
            price_per_item: None,
            status: None,
            vehicle: None,
            article: None,
            customer: None,
            supplier: None,
            container: None,
            yard: None,
            contract_number: None,
            created_date_time: None,
            updated_date_time: None,
            forwarders: None,
            scale_nr: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ID {
    pub id: Option<i32>,
    pub ident: Option<String>,
    pub short_name: Option<String>,
    pub status: Option<String>,
    pub tara_with_mobile: Option<bool>,
    pub created_date_time: Option<String>,
    pub updated_date_time: Option<String>,
    pub customer: Option<i32>,
    pub vehicle: Option<i32>,
    pub building_site: Option<i32>,
    pub supplier: Option<i32>,
    pub forwarders: Option<i32>,
    pub article: Option<i32>,
    pub yard: Option<i32>,
    pub container: Option<i32>,
    pub transaction_id: Option<String>,
}

impl Default for ID {
    fn default() -> Self {
        ID {
            id: None,
            ident: None,
            short_name: None,
            status: None,
            tara_with_mobile: None,
            created_date_time: None,
            updated_date_time: None,
            customer: None,
            vehicle: None,
            building_site: None,
            supplier: None,
            forwarders: None,
            article: None,
            yard: None,
            container: None,
            transaction_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DriverSignRequest {
    pub image: Option<String>,
    pub transaction_id: Option<i32>,
}

impl Default for DriverSignRequest {
    fn default() -> Self {
        DriverSignRequest {
            image: None,
            transaction_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorHandlerModel {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    pub token: String,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            token: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Settings {
    pub tara_save: bool,
    pub tara_pin: String,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Vehicle {
    pub license_plate: String,
    pub id: i32,
    pub vehicle_weight: Option<f64>,
    pub vehicle_weight_id: Option<String>,
    pub vehicle_weight_date: Option<String>,
    pub vehicle_weight_time: Option<String>,
    pub self_tara: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct IDRead {
    pub vehicle: Option<String>,
}
