use crate::components::constants::*;
use crate::components::model::{Contract, LicensePlateResponse, ID};
use crate::components::request::get_request;
use crate::components::state::*;
use crate::components::utils::set_get::*;
use crate::components::utils::status_handler::error_handler;
use crate::routes::Route;
use serde_json::Value;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::prelude::RouterScopeExt;

pub enum Msg {
    SetLanguage(&'static str),
    GotHome,
    SetBarcodeValue(String),
    SetLoading(bool),
    NextPage,
    PreviousPage,
}

pub struct BarcodeInputModel {
    barcode_value: String,
    is_auftrag_data_loading: bool,
}

impl BarcodeInputModel {
    async fn get_language_file() -> serde_json::Value {
        let lang_json = get_request("/bin/language.json").await;
        return lang_json.unwrap();
    }
    fn get_barcode(&self) -> String {
        self.barcode_value.clone()
    }

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

impl Component for BarcodeInputModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("{}", "inside barcode input");
        Self {
            barcode_value: "".to_string(),
            is_auftrag_data_loading: false,
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
                true
            }
            Msg::SetBarcodeValue(barcode_val) => {
                if barcode_val == String::from("<-") {
                    self.barcode_value.pop();
                } else {
                    self.barcode_value += &*barcode_val;
                }
                true
            }
            Msg::SetLoading(is_loading) => {
                self.is_auftrag_data_loading = is_loading;
                true
            }
            Msg::NextPage => {
                self.is_auftrag_data_loading = false;
                let history = _ctx.link().history().unwrap();
                history.push(Route::LicensePlateViewModel);
                true
            }
            Msg::PreviousPage => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::LanguageModel);
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
        let home_cb = link.callback(move |_| Msg::GotHome);
        let back_cb = link.callback(move |_| Msg::PreviousPage);
        let get_contract_cb = ctx.link().callback(move |_| Msg::SetLoading(true));

        if self.is_auftrag_data_loading {
            let barcode = self.barcode_value.clone();
            ctx.link().send_future(async move {
                let url = &format!("{}/api/ID/?ident={}", API_URL, barcode);
                let response = get_request(url).await;
                let data = response.unwrap().get(0).unwrap().clone();
                let id: ID = serde_json::from_value(data).unwrap();
                set_id(id.clone());
                // start_websocket();
                let websocket_url = &format!("{}?cmd=GET PLATE", DEVMAN_URL);
                let weight_response = get_request(websocket_url).await;
                let weight_data = weight_response.unwrap().clone();
                let license_plate_response: LicensePlateResponse =
                    serde_json::from_value(weight_data).unwrap();
                set_licence_plate(license_plate_response.license_plate.unwrap());
                // log::info!("{}",id.contract_number);
                Msg::NextPage
            });
        }
        // error_handler(String::from("Error test"));
        let lang_json_file = get_global_lang().clone();

        if lang_json_file.is_null() {
            let history = ctx.link().history().unwrap();
            history.push(Route::LanguageModel);
            return html! {<div></div>};
        }

        // link.sen

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
                                    <div class="col-1"></div>
                                    <div class="col-8">
                                        <label style="font-size:25px; font-weight: bold; color: #000947; margin-left: 20px">
                                            {self.get_value("enter_barcode_manually")}
                                        </label>
                                        <div style="display: flex; flexDirection: row; marginTop:20px">
                                            <label style="font-size:20px; font-weight: bold; color: #000947; margin-left: 20px">
                                            // {self.get_value("enter_id")}
                                            </label>
                                            <input autofocus={true} id="barcodeinput" style="text-align: center;margin-left: 10px; width:85%; height: 60px; border: 1px solid #000947;fontSize:18px"
                                            value={self.barcode_value.clone()}/>
                                        </div>
                                    </div>
                                    <div class="col">
                                    </div>
                                    <div class="col">
                                        <img width={150} height={150} style="border: 5px solid black" src="/img/buttons/reader.jpeg"/>
                                    </div>
                                </div>
                                <div class="row">
                                    <div class="col-1">
                                    </div>
                                    <div class="col"></div>
                                </div>
                                {
                                    if self.is_auftrag_data_loading {
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
                                                            Msg::SetBarcodeValue(string.to_string())
                                                        });
                                                        html!{
                                                        <div class="col" style={spacing} onclick={val_click_cb}>
                                                            <div style="border: 1px solid black; border-radius:4px; \
                                                                        width: 80px; height: 80px;\
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
                                                            Msg::SetBarcodeValue(string.to_string())
                                                        });
                                                        html!{
                                                        <div class="col" style={spacing} onclick={val_click_cb}>
                                                            <div style="border: 1px solid black; border-radius:4px; \
                                                                        width: 80px; height: 80px;\
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
                                                            Msg::SetBarcodeValue(string.to_string())
                                                        });
                                                        html!{
                                                        <div class="col" style={spacing} onclick={val_click_cb}>
                                                            <div style="border: 1px solid black; border-radius:4px; \
                                                                        width: 80px; height: 80px;\
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
                                                                        width: 80px; height: 80px;\
                                                                        text-align: center; justify-content: center; min-width:{}; \
                                                                        display: flex; align-items: center; color: #000947",space_val);
                                                        let val_click_cb = link.callback(move |_| {
                                                            if string == "SPACE"{
                                                                Msg::SetBarcodeValue(" ".to_string())
                                                            }
                                                            else{
                                                                Msg::SetBarcodeValue(string.to_string())
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
                                                    <div style="margin-left: 145px">
                                                        <img width={64} height={64} src="/img/phone-call.png"/>
                                                    </div>
                                                </div>
                                                <div class="col-4" onclick={get_contract_cb}>
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
