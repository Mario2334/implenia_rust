use crate::components::constants::*;
use crate::components::model::{
    ErrorHandlerModel, LicensePlateResponse, Transactions, WeightResponse, ID,
};
use crate::components::request::get_request;
use crate::components::state::get_global_lang;
use crate::components::state::*;
use crate::components::utils::set_get::*;
use crate::routes::Route;
use gloo_timers::callback::Timeout;
use js_sys::Math::log;
use serde_json::Value;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::prelude::RouterScopeExt;

pub enum RouteType {
    Manual,
    LicensePlate,
    WeightInput,
}

pub struct BarcodeModel {
    barcode_number: String,
    my_input: NodeRef,
    is_auftrag_data_loading: bool,
    manual: bool,
}

pub enum Msg {
    SetLanguage(&'static str),
    GotHome,
    InputChanged,
    ManualBarcode,
    NextPage(RouteType),
    ErrorPage(String),
}

impl BarcodeModel {
    async fn get_language_file() -> serde_json::Value {
        let lang_json = get_request("/bin/language.json").await;
        return lang_json.unwrap();
    }

    fn get_value(&self, value: &str) -> String {
        log::info!("{}", "I am here");
        let lang_json_inst = get_global_lang().clone();
        let val = lang_json_inst.get(get_lang()).and_then(|m| m.get(value));
        if val.is_none() == false {
            log::info!("{}", "inside if");
            let mut a: String = val.unwrap().to_string();
            a = a.replace("\\n", " ").replace('"', "");
            return a.clone();
        } else {
            log::info!("{}", "inside else");
            panic!("Language Setting Not Present")
        }
    }
}

impl Component for BarcodeModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        /* _ctx.link().send_future(async {
            let lang_json = Self::get_language_file().await;
            Msg::GetLanguage(lang_json)
        });*/
        Self {
            barcode_number: "".to_string(),
            my_input: NodeRef::default(),
            is_auftrag_data_loading: false,
            manual: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetLanguage(str) => {
                set_lang(str.clone());
                true
            }
            Msg::GotHome => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::Root);
                false
            }
            Msg::ManualBarcode => {
                self.is_auftrag_data_loading = true;
                self.manual = true;
                _ctx.link().send_future(async {
                    // start_websocket();
                    /*let websocket_url = &format!("{}?cmd=GET PLATE", DEVMAN_URL);
                    let weight_response = get_request(websocket_url).await;
                    let weight_data = weight_response.unwrap().clone();
                    let license_plate_response: LicensePlateResponse =
                        serde_json::from_value(weight_data).unwrap();
                    set_licence_plate(license_plate_response.license_plate.unwrap());*/

                    Msg::NextPage(RouteType::Manual)
                });
                true
            }

            Msg::ErrorPage(error) => {
                let history = _ctx.link().history().unwrap();
                let error_query = ErrorHandlerModel {
                    message: self.get_value(error.as_str()).to_string(),
                };
                history.push_with_query(Route::RetryModel, error_query);
                false
            }

            Msg::InputChanged => {
                self.is_auftrag_data_loading = true;
                self.manual = false;
                if let Some(input) = self.my_input.cast::<HtmlInputElement>() {
                    let value = input.value();
                    self.barcode_number = value;
                    set_barcode(&self.barcode_number.clone());
                    let b = self.barcode_number.clone();
                    _ctx.link().send_future(async move {
                        let url = &format!("{}/api/ID/?ident={}", API_URL, &b);
                        // let url = "http://80.152.148.142:9000/api/Contract/";
                        let response = get_request(&url).await;
                        log::info!(
                            "Response {:?}",
                            response.as_ref().unwrap().as_array().unwrap().len()
                        );
                        if response.as_ref().unwrap().as_array().unwrap().len() != 0 {
                            let data = response.unwrap().get_mut(0).unwrap().clone();
                            let id: ID = serde_json::from_value(data).unwrap();
                            log::info!("{:?}", id.clone());
                            set_id(id.clone());
                            let transaction_request_url = &format!(
                                "{}/api/Transactions/?combination_id={}&trans_flag=0",
                                API_URL,
                                id.ident.as_ref().unwrap()
                            );
                            let response = get_request(&transaction_request_url).await;
                            if response.as_ref().unwrap().get(0) != None {
                                let data = response.unwrap().get_mut(0).unwrap().clone();
                                log::info!("{}",data);
                                let transaction: Transactions = serde_json::from_value(data).unwrap();
                                set_transactions(transaction);
                            } else {
                                if id.vehicle.is_none() {
                                    return Msg::NextPage(RouteType::LicensePlate);
                                }
                            }
                            let websocket_url = &format!("{}?cmd=GET WEIGHTNM", DEVMAN_URL);
                            let weight_response = get_request(websocket_url).await;
                            let weight_data = weight_response.unwrap().clone();
                            let weight_response: WeightResponse =
                                serde_json::from_value(weight_data).unwrap();
                            set_weight_detail(weight_response.clone());
                            Msg::NextPage(RouteType::WeightInput)

                        } else {
                            // let data_null = ID::default();
                            // set_id(data_null.clone());
                            Msg::ErrorPage("id_not_found".to_string())
                        }
                    });

                    return true;
                } else {
                    return false;
                }
            }

            Msg::NextPage(route_type) => {
                let history = _ctx.link().history().unwrap();
                match route_type {
                    RouteType::Manual => {
                        history.push(Route::LicensePlateViewModel);
                    }
                    RouteType::LicensePlate => {
                        history.push(Route::LicensePlateViewModel);
                    }
                    RouteType::WeightInput => {
                        history.push(Route::WeightViewModel);
                    }
                }
                return false;
                // if self.manual {
                //     history.push(Route::LicensePlateViewModel);
                //     return false;
                // } else {
                //     if get_id().ident == None && get_transactions().id == None {
                //         log::info!("Goint to retry");
                //         self.is_auftrag_data_loading = false;
                //         history.push(Route::RetryModel);
                //     } else {
                //         history.push(Route::WeightViewModel);
                //     }
                //     return false;
                // }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let home_cb = link.callback(move |_| Msg::GotHome);
        let onclick = link.callback(move |_| Msg::ManualBarcode);
        let onchange = link.callback(move |_| Msg::InputChanged);

        let lang_json_file = get_global_lang().clone();
        if lang_json_file.is_null() {
            let history = ctx.link().history().unwrap();
            history.push(Route::LanguageModel);
            return html! {<div></div>};
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
                                        // <img width=150 height=70 src="/img/evo.png"/>

                                    </div>
                                    <div>
                                        <img width=150 height=70 src="/img/Logo.png"/>
                                    </div>
                                </div>
                                if self.is_auftrag_data_loading{
                                    <div class="row">
                                        <div class="col-md-12 text-center" style="margin-top: 250px;">
                                            <label style="font-size:60px; font-weight: bold; color: #000947;">
                                                { self.get_value("please_wait") }
                                            </label>
                                        </div>
                                    </div>

                                } else{
                                    <div class="row">
                                        <div class="col-md-12 text-center">
                                            <label style="font-size:50px; font-weight: bold; color: #000947;">
                                                { self.get_value("barcode_scan") }
                                            </label>
                                        </div>
                                    </div>

                                    <div class="row" style="margin-top: 50px; ">
                                    <div class="col-md-6">
                                        <img width="100%" height="400px" style="border: 2px solid black" src="/img/buttons/reader.jpeg"/>
                                    </div>
                                    <div class="col-md-6 text-center" style="margin-top:100px;">
                                        <label style="white-space: pre-line; font-size:25px; font-weight: bold; color: #000947;">
                                            { self.get_value("hold_barcode_infront") }
                                        </label>
                                        <div style="display: flex; flexDirection: row; marginTop:20px">
                                            <input ref={self.my_input.clone()}
                                                {onchange}
                                                id="barcodescan"
                                                type="text"
                                                style="text-align: center;margin-left: 10px; width:100%; height: 60px; border: 1px solid #000947;fontSize:18px; border-radius:5px;"
                                                autofocus={true}
                                                />
                                        </div>
                                        <button
                                            {onclick}
                                            class="btn"
                                            style="
                                                background-color: #000947; 
                                                color: white;
                                                height: 60px;
                                                width: 200px;
                                                margin-top: 100px;">
                                                { self.get_value("enter_barcode_manually") }
                                        </button>
                                    </div>
                                </div>
                                }


                    </div>
            </div>
        }
    }
}
