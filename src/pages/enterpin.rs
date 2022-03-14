use js_sys::parse_int;
use yew::prelude::*;
use yew_router::{
    history::{self, History},
    prelude::RouterScopeExt,
};

use crate::{
    components::{
        constants::TaraPin,
        state::{set_global_lang, set_weighing_type},
        utils::set_get::set_lang,
    },
    routes::Route,
};

pub struct EnterPinModel {
    pin: i32,
}

pub enum Msg {
    NextPage,
    Home,
    BackArrow,
    UpdatePin(String),
}
impl Component for EnterPinModel {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self { pin: 0 }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextPage => {
                let history = _ctx.link().history().unwrap();
                if self.pin != TaraPin {
                    history.push(Route::RetryModel);
                }
                set_lang("en");
                set_weighing_type(crate::components::state::WeighingType::TaraSava);
                history.push(Route::BarcodeModel);
                true
            }
            Msg::BackArrow => true,
            Msg::Home => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::LanguageModel);
                true
            }
            Msg::UpdatePin(s) => {
                if s == "<".to_string() {
                    self.pin /= 10;
                } else {
                    let dig = parse_int(&s, 10) as i32;
                    self.pin = self.pin * 10 + dig;
                }
                true
            }
        }
    }
    fn view(self: &EnterPinModel, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let home_cb = link.callback(move |_| Msg::Home);
        let next_cb: Callback<MouseEvent> = link.callback(move |_| Msg::NextPage);
        let back_cb: Callback<MouseEvent> = link.callback(move |_| Msg::BackArrow);
        const style: &str = "height:75px;border-radius:15%;border:1px solid black;text-align:center;background: #000947;color:white";
        html! {
            <>
             <div class = "container">
                <div class="row" style="margin-top: '10px'"></div>
                <div class="row" style="margin-top: 10px">
                    <div onclick={home_cb}>
                        <img width={80} height={80} src="img/buttons/Home.png" />
                    </div>
                    <div style="width: 250px;margin-left: auto;margin-right: auto;text-align: center;"></div>
                    <div><img width=150 height=70 src="/img/Logo.png" /></div>
                </div>

                <div class = "row mt-5 justify-content-center">
                    <div class = "col-4">
                     <input type = "number" value = {self.pin.clone().to_string()} class = "p-3"
                     style="border-radius:20px;width:360px;height:60px;border: 1px solid black;font-size:20px"
                     />
                    </div>
                </div>

                <div class = "row mt-5 justify-content-center">
                    <div class = "col-6">
                        <div class = "row ml-5">
                            <div class = "col-3 p-1">
                                <div class = "col-9 pt-4" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("1".to_string()))}>{1}</div>
                            </div>
                            <div class = "col-3 p-1">
                                <div class = "col-9 pt-4" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("2".to_string()))}>{2}</div>
                            </div>
                            <div class = "col-3 p-1">
                                <div class = "col-9 pt-4" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("3".to_string()))}>{3}</div>
                            </div>
                            <div class = "col-3 p-1">
                                <div class = "col-9 pt-4" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("<".to_string()))}>{"<"}</div>
                            </div>
                        </div>
                        <div class = "row mt-2">
                            <div class = "col-9">
                                <div class = "row ml-5">
                                    <div class = "col-4 p-1">
                                        <div class = "col-9 pt-4" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("4".to_string()))}>{4}</div>
                                    </div>
                                    <div class = "col-4 p-1">
                                        <div class = "col-9 pt-4 ml-2" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("5".to_string()))}>{5}</div>
                                    </div>
                                     <div class = "col-4 p-1">
                                        <div class = "col-9 pt-4 ml-2" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("6".to_string()))}>{6}</div>
                                    </div>
                                </div>
                                <div class = "row pl-5 mt-2 ml-2">
                                    <div class = "col-4 p-1">
                                        <div class = "col-9 pt-4" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("7".to_string()))}>{7}</div>
                                    </div>
                                    <div class = "col-4 p-1">
                                        <div class = "col-9 pt-4 ml-2" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("8".to_string()))}>{8}</div>
                                    </div>
                                    <div class = "col-4 p-1">
                                        <div class = "col-9 pt-4 ml-2" style = {style.clone()} onclick = {link.callback(|_| Msg::UpdatePin("9".to_string()))}>{9}</div>
                                    </div>
                                </div>
                            </div>
                            <div class = "col-3">
                                    <div class = "col-12" onclick = {link.callback(|_| Msg::UpdatePin("0".to_string()))}>
                                            <div class = "col-12 pt-4" style = "height:160px;text-align:center;border-radius:10%;border:1px solid black;background: #000947;color:white">{0}</div>
                                    </div>
                            </div>
                        </div>
                    </div>
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
            </div>
            </>
        }
    }
}
