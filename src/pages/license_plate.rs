#![feature(async_await)]
use serde_json::Value;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::prelude::RouterScopeExt;
use crate::components::request::get_request;
use crate::routes::Route;
use crate::components::state::*;
use crate::components::utils::set_get::*;
use wasm_bindgen::{JsCast, JsValue};
use crate::components::constants::*;


pub enum Msg {
    SetLanguage(&'static str),
    GotHome,
    SetLicensePlate(String),
    GetContractDetail(serde_json::Value),
    GoBack,
}

pub struct LicensePlateModel {
    license_plate:String,
    loading: bool,
    error: bool,
    contract_data: serde_json::Value
}

impl LicensePlateModel {
    async fn get_language_file() -> serde_json::Value{
        let lang_json = get_request("/bin/language.json").await;
        return lang_json.unwrap()
    }

    fn get_value(&self, value:&str) -> String{
        let lang_json_inst = get_global_lang().clone();
        let val = lang_json_inst.get(get_lang()).and_then(|m| m.get(value));
        log::info!("{}",value);
        if val.is_none() == false{
            let a =  val.unwrap().clone();
            return a.to_string().replace('"',"")
        }
        else {
            panic!("Language Setting Not Present")
        }
    }
    async fn get_contract_detail(contract_number: &str) -> serde_json::Value{
        let url: &str = &format!("{}/api/Contract/{}/",API_URL,contract_number);
        let res = get_request(&url).await;
        return res.unwrap()
    }
}

impl Component for LicensePlateModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_future(async {
            let res = Self::get_contract_detail(&get_barcode()).await;
            Msg::GetContractDetail(res)
        });
        Self {
            license_plate: "".to_string(),
            loading: true,
            contract_data: Value::Null,
            error: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetLanguage(str) => {
                set_lang(str.clone());
                true
            },
            Msg::GotHome =>{
                let history = _ctx.link().history().unwrap();
                history.push(Route::Root);
                true
            }
            Msg::SetLicensePlate(license_plate_val) =>{
                if license_plate_val == String::from("<-") {
                    self.license_plate.pop();
                }
                else {
                    self.license_plate += &*license_plate_val;
                }
                true
            }
            Msg::GetContractDetail(val)=>{
                if val["contract_number"] != Value::Null{
                    self.loading = false;
                    self.contract_data = val;
                } else{
                    self.loading = true;
                    self.error = true;
                }
                true
            }

            Msg::GoBack =>{
                
                for n in 1..10000000{
                    println!("Sleep {}",n);
                }
                let history = _ctx.link().history().unwrap();
                history.push(Route::BarcodeModel);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let key_one_line = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-"];
        let key_two_line = vec!["Q", "W", "E", "R", "T", "Z", "U", "I", "O", "P", "<-"];
        let key_three_line = vec!["A", "S", "D", "F", "G", "H", "J", "K", "L", "Ä", "Ö"];
        let key_four_line = vec!["Y", "X", "C", "V", "B", "N", "M", "Ü", ".", "SPACE"];

        const OVERIDE: &str = "
        display: block;
        margin: 0 auto;
        marginTop: 100;
        border-color: red;
        ";

        let link = ctx.link();
        let home_cb = link.callback(move |_| Msg::GotHome );

        let lang_json_file = get_global_lang().clone();
        log::info!("{}",lang_json_file);

        if lang_json_file.is_null() {
            let history = ctx.link().history().unwrap();
            history.push(Route::LanguageModel);
            return html!{<div></div>}
        }

        html!{
            <div>
                <div class="container" style="height: 660px">
                    <div class="row" style="margin-top: '10px'">
                    </div>
                    <div class="row" style="margin-top: 10px">
                        <div onclick={home_cb}>
                            <img width={80} height={80} src="img/buttons/Home.png" />
                        </div>
                        <div style="width: 250px;margin-left: auto;margin-right: auto;text-align: center;">
                            //<img width=150 height=70 src="/img/evo.png"/>
                            
                        </div>
                        <div>
                            <img width=150 height=70 src="/img/Logo.png"/>
                        </div>
                    </div>
                    if self.loading == true{
                        <div class="row">
                            <div class="col-md-12 text-center">
                                
                                if self.error == false{
                                    <label 
                                    style="font-size:50px; 
                                    margin-top:200px; 
                                    font-weight: bold; 
                                    color: #000947;">
                                        { self.get_value("please_wait") }
                                    </label>
                                } else{
                                    <label
                                    style="font-size:50px; 
                                    margin-top:200px; 
                                    font-weight: bold; 
                                    color: red;">
                                        { self.get_value("retry_butt") }
                                    </label>
                                }
                                
                            </div>
                        </div>
                    } else{
                        <div class="row" style="margin-top: 20px">
                            <div class="col-2"></div>
                            <div class="col-8 text-center">
                                <label style="font-size:25px; font-weight: bold; color: #000947; margin-left: 20px">
                                    {self.get_value("check_extracted_license_plate")}
                                </label>
                                <div style="display: flex; flexDirection: row; marginTop:20px">
                                    <label style="font-size:20px; font-weight: bold; color: #000947; margin-left: 20px">
                                    // {self.get_value("enter_id")}
                                    </label>
                                    <input autofocus={true} id="barcodeinput" style="text-align: center;margin-left: 10px; width:85%; height: 60px; border: 1px solid #000947;fontSize:18px"
                                    value={self.license_plate.clone()}/>
                                </div>
                            </div>
                            <div class="col-2"></div>
                        </div>
                        <div class="row">
                            <div class="col-1">
                            </div>
                            <div class="col"></div>
                        </div>
                        <div class="row" style="margin-top:20px">
                            {
                                key_one_line
                                .iter()
                                .enumerate()
                                .map(|(i, char1)|{
                                    let string = char1.clone();
                                    let mut spacing = "";
                                    if i > 0 {
                                        spacing = "margin-left: -20px";
                                    }
                                    let val_click_cb = link.callback(move |_| {
                                        Msg::SetLicensePlate(string.to_string())
                                    });
                                    html!{
                                    <div class="col" style={spacing} onclick={val_click_cb}>
                                        <div style="border: 1px solid black; border-radius:4px; \
                                                    width: 70px; height: 70px;\
                                                    text-align: center; justify-content: center; \
                                                    display: flex; align-items: center; color: #000947">
                                        {{string}}
                                        </div>
                                    </div>
                                    }
                            }).collect::<Html>()
                            }
                        </div>
                        <div class="row" style="margin-top:10px">
                            {
                                key_two_line
                                .iter()
                                .enumerate()
                                .map(|(i, char1)|{
                                    let string = char1.clone();
                                    let mut spacing = "";
                                    if i > 0 {
                                        spacing = "margin-left: -20px";
                                    }
                                    let val_click_cb = link.callback(move |_| {
                                        Msg::SetLicensePlate(string.to_string())
                                    });
                                    html!{
                                    <div class="col" style={spacing} onclick={val_click_cb}>
                                        <div style="border: 1px solid black; border-radius:4px; \
                                                    width: 70px; height: 70px;\
                                                    text-align: center; justify-content: center; \
                                                    display: flex; align-items: center; color: #000947">
                                        {{string}}
                                        </div>
                                    </div>
                                    }
                            }).collect::<Html>()
                            }
                        </div>
                        <div class="row" style="margin-top:10px">
                            {
                                key_three_line
                                .iter()
                                .enumerate()
                                .map(|(i, char1)|{
                                    let string = char1.clone();
                                    let mut spacing = "";
                                    if i > 0 {
                                        spacing = "margin-left: -20px";
                                    }
                                    let val_click_cb = link.callback(move |_| {
                                        Msg::SetLicensePlate(string.to_string())
                                    });
                                    html!{
                                    <div class="col" style={spacing} onclick={val_click_cb}>
                                        <div style="border: 1px solid black; border-radius:4px; \
                                                    width: 70px; height: 70px;\
                                                    text-align: center; justify-content: center; \
                                                    display: flex; align-items: center; color: #000947">
                                        {{string}}
                                        </div>
                                    </div>
                                    }
                            }).collect::<Html>()
                            }
                        </div>
                        <div class="row" style="margin-top:10px">
                            {
                                key_four_line
                                .iter()
                                .enumerate()
                                .map(|(i, char1)|{
                                    let string = char1.clone();
                                    let mut spacing = "";
                                    if i > 0 {
                                        spacing = "margin-left: -20px";
                                    }
                                    let mut space_val = "0";
                                    if char1 == &"SPACE"{
                                        space_val = "160px";
                                    }
                                    let buttonStyle= format!("border: 1px solid black; border-radius:4px; \
                                                    width: 70px; height: 70px;\
                                                    text-align: center; justify-content: center; min-width:{}; \
                                                    display: flex; align-items: center; color: #000947",space_val);
                                    let val_click_cb = link.callback(move |_| {
                                        if string == "SPACE"{
                                            Msg::SetLicensePlate(" ".to_string())
                                        }
                                        else{
                                            Msg::SetLicensePlate(string.to_string())
                                        }
                                    });
                                    html!{
                                    <div class="col" style={spacing} onclick={val_click_cb}>
                                        <div style={buttonStyle}>
                                        {{char1}}
                                        </div>
                                    </div>
                                    }
                            }).collect::<Html>()
                            }
                        </div>
                        <div class="row" style="margin-top:10px">
                            <div class="col-4">
                                <div style="margin-left: 30px">
                                    <img width={80} height={80} src="/img/buttons/BackArrow.png" />
                                </div>
                            </div>
                            <div class="col-4">
                                // <div style="margin-left: 145px">
                                //     <img width={64} height={64} src="/img/phone-call.png"/>
                                // </div>
                            </div>
                            <div class="col-4">
                                <div style="margin-right: 30px;float:right">
                                    <img width={80} height={80} src="/img/buttons/NextArrow.png"/>
                                </div>
                            </div>
                        </div>
                    }
                </div>
            </div>
        }
    }
}