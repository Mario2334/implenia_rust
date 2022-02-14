use crate::components::model::{LicensePlateResponse, WeightResponse, ID};
use crate::components::request::get_request;
use crate::components::state::get_global_lang;
use crate::components::state::*;
use crate::components::utils::set_get::*;
use crate::routes::Route;
use gloo_timers::callback::Timeout;
use serde_json::Value;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::prelude::RouterScopeExt;
pub struct ThankYouModel {}

pub enum Msg {
    GotHome,
}

impl ThankYouModel {
    async fn get_language_file() -> serde_json::Value {
        let lang_json = get_request("/bin/language.json", None).await;
        return lang_json.unwrap();
    }

    fn get_value(&self, value: &str) -> String {
        let lang_json_inst = get_global_lang().clone();
        let val = lang_json_inst.get(get_lang()).and_then(|m| m.get(value));
        if val.is_none() == false {
            let mut a: String = val.unwrap().to_string();
            a = a.replace("\\n", " ").replace('"', "");
            return a.clone();
        } else {
            panic!("Language Setting Not Present")
        }
    }
}

impl Component for ThankYouModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotHome => {
                let history = _ctx.link().history().unwrap();
                let timeout = Timeout::new(5_000, move || {
                    history.push(Route::Root);
                })
                .forget();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let gotohome = link.callback(move |_| Msg::GotHome);
        let lang_json_file = get_global_lang().clone();
        if lang_json_file.is_null() {
            let history = ctx.link().history().unwrap();
            history.push(Route::LanguageModel);
            return html! {<div></div>};
        }

        html! {
            <div onload={gotohome}>
                <div class="container" style="height: 660px">
                    <div class="row" style="margin-top: '10px'">
                    </div>
                    <div class="row" style="margin-top: 10px">
                        <div>
                            // <img width={80} height={80} src="img/buttons/Home.png" />
                        </div>
                        <div style="width: 250px;margin-left: auto;margin-right: auto;text-align: center;">
                            // <img width=150 height=70 src="/img/evo.png"/>

                        </div>
                        <div>
                            <img width=150 height=70 src="/img/Logo.png"/>
                        </div>
                    </div>

                    <div class="row">
                        <div class="col-md-12 text-center" style="margin-top: 250px;">
                            <label style="font-size:60px; font-weight: bold; color: #000947;">
                                { self.get_value("thank_you") }
                            </label>
                        </div>
                    </div>


                </div>
            </div>
        }
    }
}
