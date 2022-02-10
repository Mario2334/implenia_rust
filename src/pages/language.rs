use crate::components::images;
use crate::components::popup_messages::error_popup;
use crate::components::request::{get_request, post_request};
use crate::components::state::{get_global_lang, reset_state, set_global_lang, set_settings, set_token};
use crate::components::utils::set_get::*;
use crate::components::utils::*;
use crate::components::websocket::start_websocket;
use serde_json::Value;
use std::collections::HashMap;
use log::log;
// use js_sys::Intl::format;
use web_sys::console;
use web_sys::console::log_1;
use yew::prelude::*;
use crate::components::constants::API_URL;
use crate::components::model::{Settings, Token, User};

pub enum Msg {
    GetLanguage(serde_json::Value),
    SetLanguage(&'static str),
}

pub struct LanguageModel {
    json_lang: serde_json::Value,
    is_loading: bool,
}

impl LanguageModel {
    async fn get_language_file() -> serde_json::Value {
        let lang_json = get_request("/bin/language.json").await;
        log::info!("{}", lang_json.is_ok());
        return lang_json.unwrap();
    }

    async fn set_settings() {
        let response = get_request("/bin/settings.json").await;
        let sett_json: Settings = serde_json::from_value(response.unwrap()).unwrap();
        set_settings(sett_json);
    }

    fn get_value(&self, value: &str) -> String {
        let lang_json_inst = get_global_lang().clone();
        let val = lang_json_inst.get(get_lang()).and_then(|m| m.get(value));
        if val.is_none() == false {
            let a = val.unwrap().clone();
            return a.to_string();
        } else {
            panic!("Language Setting Not Present")
        }
    }

    async fn authenticate() {
        let url = format!("{}/api-token-auth",API_URL);
        let user = User {
            username: "admin@admin.com".to_string(),
            password: "admin".to_string()
        };
        let body = serde_json::to_string(&user).unwrap();
        let response = post_request(url.as_str(),body.as_str()).await;
        let token:Token = serde_json::from_value(response.unwrap()).unwrap();
        set_token(token.token);
    }
}

impl Component for LanguageModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        reset_state();
        // error_popup();
        _ctx.link().send_future(async {
            let lang_json = Self::get_language_file().await;
            Self::authenticate().await;
            Self::set_settings().await;
            Msg::GetLanguage(lang_json)
        });
        Self {
            json_lang: Value::Null,
            is_loading: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetLanguage(lang_json) => {
                set_global_lang(lang_json);
                self.is_loading = false;
                true
            }
            Msg::SetLanguage(str) => {
                set_lang(str.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        // start_websocket();
        html! {
                <div class="container">
                    <div class="row" style="margin-top: 10px">
                            // <p>{"Test"}</p>
                    </div>
                {
                if self.is_loading == false {
                        html!{
                            <>
                            <div class="row" style="margin-top: 10px">
                                <div style=" display: flex; flex-direction: row;width:100% ">
                                    <div>
                                        // <img width=64 height=64 src="img/phone-call.png"/>
                                        <div width=64 height=64/>
                                    </div>
                                    <div style="width: 250px; margin-left:355px;\
                                                margin-right:auto; text-align: center">
                                        // <img width=150 height=70 src="img/evo.png" />

                                    </div>
                                    <div>
                                        <img width=150 height=70 src="/img/Logo.png"/>
                                    </div>
                                </div>
                            </div>
                            <div class="row" style="margin-top: 20px;align-items: center;margin-left: 60px">
                                {
                                    images::get_images1().into_iter().map(|image_stores1|{
                                    html!{
                                        <images::ImageStore
                                        src={image_stores1.src}
                                        width=1
                                        height=1
                                        language={image_stores1.language}
                                        >
                                        </images::ImageStore>
                                    }
                                }).collect::<Html>()
                                }
                            </div>
                            <div class="row" style="margin-top: 0px;align-items: center;margin-left: 60px">
                                {
                                    images::get_images2().into_iter().map(|image_stores2|{
                                        html!{
                                            <images::ImageStore
                                            src={image_stores2.src}
                                            width=1
                                            height=1
                                            language={image_stores2.language}
                                            >
                                            </images::ImageStore>
                                        }
                                        }).collect::<Html>()
                                    }
                            </div>
                            <div class="row" style="margin-top: 0px;align-items: center;margin-left: 220px">
                                {
                                    images::get_images3().into_iter().map(|image_stores3|{
                                        html!{
                                            <images::ImageStore
                                            src={image_stores3.src}
                                            width=1
                                            height=1
                                            language={image_stores3.language}
                                            >
                                            </images::ImageStore>
                                        }
                                        }).collect::<Html>()
                                    }
                            </div>
                        </>
                }
                        }
            else {
                html!{}
            }
                    }
           </div>
        }
    }
}
