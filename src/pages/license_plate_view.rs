use std::default;

use crate::components::constants::*;
use crate::components::model::{Transactions, Vehicle, WeightResponse, ID};
use crate::components::request::{get_request, post_request};
use crate::components::state::*;
use crate::components::utils::set_get::*;
use crate::routes::Route;
use log::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::{history, prelude::*};

pub enum Msg {
    SetLicensePlate(String),
    GotHome,
    NextPage,
    PreviousPage,
    UpdateLoading,
}

pub struct LicensePlateView {
    license_plate: String,
    loading: bool,
}

impl LicensePlateView {
    fn get_value(&self, value: &str) -> String {
        let lang_json_inst = get_global_lang().clone();
        let val = lang_json_inst.get(get_lang()).and_then(|m| m.get(value));
        log::info!("{}", value);
        if val.is_none() == false {
            let a = val.unwrap().clone();
            return a.to_string().replace('"', "");
        } else {
            panic!("Language Setting Not Present")
        }
    }
}

impl Component for LicensePlateView {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let license_plate = get_license_plate();
        LicensePlateView {
            license_plate: "RW58341".to_string(),
            loading: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetLicensePlate(license_plate) => {
                if license_plate == String::from("<-") {
                    self.license_plate.pop();
                } else {
                    self.license_plate += &*license_plate;
                }
                true
            }
            Msg::NextPage => {
                let x = get_weighing_type();
                let history = _ctx.link().history().unwrap();

                match x {
                    WeighingType::First => history.push(Route::SelectVehicle),
                    WeighingType::Second => history.push(Route::WeightViewModel),
                    _default => history.push(Route::WeightViewModel),
                }
                true
            }
            Msg::PreviousPage => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::BarcodeModel);
                true
            }
            Msg::GotHome => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::Root);
                true
            }
            Msg::UpdateLoading => {
                self.loading = true;
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
        border-color: red;
        ";

        let link = ctx.link();
        let history = link.history().unwrap();
        let home_cb = link.callback(move |_| Msg::GotHome);
        let back_cb = link.callback(move |_| Msg::PreviousPage);
        let next_cb = link.callback(move |_| Msg::UpdateLoading);
        let contract = get_contract();
        log::info!("{}", contract.contract_number);
        log::info!("{}", get_license_plate());
        let lang_json_file = get_global_lang().clone();

        if lang_json_file.is_null() {
            let history = ctx.link().history().unwrap();
            history.push(Route::LanguageModel);
            return html! {<div></div>};
        }

        if self.loading {
            let b = self.license_plate.clone();
            ctx.link().send_future(async move {
                let w_type = get_weighing_type();
                match w_type {
                    WeighingType::TaraSava => {
                        log::info!("{}", "inside tara save");
                        let url = format!("{}api/ID/?ident={}", API_URL, b);
                        let res = get_request(&url, None).await;
                        if res.as_ref().unwrap().get(0) == None {
                            history.push(Route::RetryModel);
                        } else {
                            let data = res.unwrap().get_mut(0).unwrap().clone();
                            let id: ID = serde_json::from_value(data).unwrap();
                            set_id(id.clone());
                            if id.vehicle.is_some() {
                                let websocket_url = &format!("{}?cmd=GET WEIGHTNM", DEVMAN_URL);
                                let weight_response = get_request(websocket_url, None).await;
                                let weight_data = weight_response.unwrap().clone();
                                let weight_response: WeightResponse =
                                    serde_json::from_value(weight_data).unwrap();
                                log::info!("{}", weight_response.weight);
                                set_weight_detail(weight_response.clone());
                                history.push(Route::WeightViewModel);
                            } else {
                                history.push(Route::SelectVehicle);
                            }
                        }
                    }
                    _default => {
                        let request_url = format!(
                            "{}api/Transactions/?combination_id={}&trans_flag=0",
                            API_URL, b
                        );
                        let response = get_request(&request_url, None).await;
                        if response.as_ref().unwrap().get(0) == None {
                            let url = format!("{}api/ID/?ident={}", API_URL, b);
                            let res = get_request(&url, None).await;
                            if res.as_ref().unwrap().get(0) == None {
                                log::info!("{}", "from here");
                                history.push(Route::RetryModel);
                            } else {
                                let data = res.unwrap().get_mut(0).unwrap().clone();
                                let id: ID = serde_json::from_value(data).unwrap();
                                set_id(id.clone());
                                if id.tara_with_mobile.is_some() && id.tara_with_mobile.unwrap() {
                                    set_weighing_type(WeighingType::Tara);
                                    let url = format!(
                                        "{}api/Vehicle-View/{}/",
                                        API_URL,
                                        id.vehicle.unwrap()
                                    );
                                    let vehicle: Vehicle = serde_json::from_value(
                                        get_request(&url, None).await.unwrap(),
                                    )
                                    .unwrap();
                                    if vehicle.vehicle_weight.is_none()
                                        || vehicle.vehicle_weight.unwrap() <= 0.0
                                    {
                                        log::info!("{}", "from here 2");
                                        history.push(Route::RetryModel);
                                    } else {
                                        set_vehicle(vehicle);
                                        history.push(Route::ProcessDirection);
                                    }
                                } else {
                                    //first weighing
                                    set_weighing_type(WeighingType::First);
                                }
                            }
                        } else {
                            set_weighing_type(WeighingType::Second);
                            let trans: Transactions =
                                serde_json::from_value(response.unwrap().get(0).unwrap().clone())
                                    .unwrap();
                            let websocket_url = &format!("{}?cmd=GET WEIGHTNM", DEVMAN_URL);
                            let weight_response = get_request(websocket_url, None).await;
                            let weight_data = weight_response.unwrap().clone();
                            let weight_response: WeightResponse =
                                serde_json::from_value(weight_data).unwrap();
                            log::info!("{}", weight_response.weight);
                            set_weight_detail(weight_response.clone());
                            set_transactions(trans);
                        }
                    }
                };

                Msg::NextPage
            });
        }

        html! {
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

                        {
                                    if self.loading {
                                        html!{
                                        <div style="margin-top: 30px">
                                            // <ClockLoader color={'#000947'} loading={true} css={override} size={50} id='loaderone' />
                                            <div style={OVERIDE} class="spinner-border text-primary" role="status">
                                              <span class="sr-only">{"Loading..."}</span>
                                            </div>
                                            <label style="font-size: 40px; font-weight: bold; color: #000947; margin-left: 400px; margin-top: 25px" >{self.get_value("please_wait") + "..."}</label>
                                        </div>
                                        }
                                    }
                                    else{
                                        html!{
                                            <>
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
                                                <div class="col-4" onclick={back_cb}>
                                                    <div style="margin-left: 30px">
                                                        <img width={80} height={80} src="/img/buttons/BackArrow.png" />
                                                    </div>
                                                </div>
                                                <div class="col-4">
                                                    // <div style="margin-left: 145px">
                                                    //     <img width={64} height={64} src="/img/phone-call.png"/>
                                                    // </div>
                                                </div>
                                                <div class="col-4" onclick={next_cb}>
                                                    <div style="margin-right: 30px;float:right">
                                                        <img width={80} height={80} src="/img/buttons/NextArrow.png"/>
                                                    </div>
                                                </div>
                                            </div>
                                            </>
                                        }
                                    }
                                }
                </div>
            </div>
        }
    }
}
