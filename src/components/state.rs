use lazy_static::lazy_static;
use std::sync::Mutex;
use serde_json::Value;
use crate::components::model::{Contract, WeightResponse, ID, Transactions};

struct GlobalState {
    language_json: serde_json::Value,
    contract: Contract,
    license_plate: String,
    weight_detail: WeightResponse,
    id: ID,
    transactions: Transactions
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState{
            language_json: Value::Null,
            contract: Contract::default(),
            license_plate: "".to_string(),
            weight_detail: WeightResponse::default(),
            id: ID::default(),
            transactions: Transactions::default(),
        }
    }
}

impl GlobalState {
    fn set_lang_json(&mut self, lang_json:serde_json::Value){
        self.language_json = lang_json;
    }
    fn set_contract(&mut self, contract:Contract){self.contract = contract;}
    fn set_license(&mut self, license_plate:String){self.license_plate = license_plate;}
    fn set_weight_detail(&mut self, weight_detail:WeightResponse){self.weight_detail = weight_detail;}
    fn set_id(&mut self, id:ID){self.id = id;}
    fn set_transactions(&mut self, transactions:Transactions){self.transactions = transactions;}
    fn reset_state(&mut self) {
        self.language_json = Value::Null;
        self.contract = Contract::default();
        self.license_plate = "".to_string();
        self.weight_detail = WeightResponse::default();
        self.id= ID::default();
        self.transactions= Transactions::default();

    }
}

lazy_static! {
    // static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
    static ref GLOBALSTATE: Mutex<GlobalState> = Mutex::new(GlobalState::default());
}

pub fn set_global_lang(lang_json:serde_json::Value) {
    GLOBALSTATE.lock().unwrap().set_lang_json(lang_json);
}

pub fn get_global_lang() -> serde_json::Value{
    let lang_json = GLOBALSTATE.lock().unwrap().language_json.clone();
    return lang_json;
}

pub fn get_contract() -> Contract {
    let contract = GLOBALSTATE.lock().unwrap().contract.clone();
    return contract
}

pub fn set_contract(contract: Contract){
    GLOBALSTATE.lock().unwrap().set_contract(contract);
}

pub fn set_id(id: ID){
    GLOBALSTATE.lock().unwrap().set_id(id);
}

pub fn get_id() -> ID {
    let id = GLOBALSTATE.lock().unwrap().id.clone();
    return id
}

pub fn set_licence_plate(license_plate: String) {
    GLOBALSTATE.lock().unwrap().set_license(license_plate);
}

pub fn get_license_plate() -> String {
    let license_plate = GLOBALSTATE.lock().unwrap().license_plate.clone();
    return license_plate
}

pub fn set_weight_detail(weight_detail: WeightResponse) {
    GLOBALSTATE.lock().unwrap().set_weight_detail(weight_detail);
}

pub fn get_weight_detail() -> WeightResponse {
    let weight_detail = GLOBALSTATE.lock().unwrap().weight_detail.clone();
    return weight_detail
}

pub fn set_transactions(transactions: Transactions) {
    GLOBALSTATE.lock().unwrap().set_transactions(transactions);
}

pub fn get_transactions() -> Transactions {
    let transactions = GLOBALSTATE.lock().unwrap().transactions.clone();
    return transactions
}

pub fn reset_state() {
    GLOBALSTATE.lock().unwrap().reset_state();
}