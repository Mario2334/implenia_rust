use crate::components::constants::API_URL;
use crate::components::model::{DriverSignRequest, TransactionPDFRequest};
use crate::components::request::{post_request, put_request};
use crate::components::state::{get_global_lang, get_transactions};
use crate::components::utils::set_get::*;
use crate::routes::Route;
use js_sys::Boolean;
use log::log;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    SetCanvas,
    SetLoading,
    PreviousPage,
    NextPage,
}

pub struct SignatureModel {
    is_loading: bool,
}

impl SignatureModel {
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

    fn start(&self) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        // let document = web_sys::window().unwrap().document().unwrap();
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        // document.body().unwrap().append_child(&canvas)?;
        canvas.set_width(640);
        canvas.set_height(480);
        canvas.style().set_property("border", "solid")?;
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let context = Rc::new(context);
        let pressed = Rc::new(Cell::new(false));
        {
            let context = context.clone();
            let pressed = pressed.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
                pressed.set(true);
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let context = context.clone();
            let pressed = pressed.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if pressed.get() {
                    context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                    context.stroke();
                    context.begin_path();
                    context.move_to(event.offset_x() as f64, event.offset_y() as f64);
                }
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let context = context.clone();
            let pressed = pressed.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                pressed.set(false);
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        Ok(())
    }

    fn get_canvas_image(&self) -> String {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let image = canvas.to_data_url().unwrap();
        return image;
    }
}

impl Component for SignatureModel {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { is_loading: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetCanvas => {
                self.start();
                false
            }
            Msg::SetLoading => {
                self.is_loading = true;
                true
            }
            Msg::PreviousPage => {
                let history = ctx.link().history().unwrap();
                history.push(Route::LanguageModel);
                false
            }
            Msg::NextPage => {
                let history = ctx.link().history().unwrap();
                history.push(Route::LanguageModel);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // self.start();
        let link = ctx.link();
        if self.is_loading == false {
            let canvas = link.send_future(async move { Msg::SetCanvas });
        }
        let back_cb = link.callback(move |_| Msg::PreviousPage);
        let get_contract_cb = ctx.link().callback(move |_| Msg::SetLoading);

        let lang_json_file = get_global_lang().clone();

        if lang_json_file.is_null() {
            let history = ctx.link().history().unwrap();
            history.push(Route::LanguageModel);
            return html! {<div></div>};
        }

        if self.is_loading {
            log::info!("loading is true");
            let image_data = self.get_canvas_image();
            log::info!("{}", image_data);
            ctx.link().send_future(async {
                let transactions = get_transactions();
                let _driver_sign_request = DriverSignRequest {
                    image: Some(image_data),
                    transaction_id: Some(transactions.id.unwrap()),
                };
                let driver_sign_update_url = &format!("{}/api/DriverSign/", API_URL);
                let body = serde_json::to_string(&_driver_sign_request).unwrap();
                let driver_sign_resp =
                    post_request(driver_sign_update_url, body.as_ref(), None).await;

                // Request PDF API
                let pdf_request_body = TransactionPDFRequest {
                    id: transactions.id.unwrap(),
                };
                let pdf_request_url = &format!("{}/api/pdf_backend", API_URL);
                let data = serde_json::to_string(&pdf_request_body);
                let response =
                    post_request(&pdf_request_url, &data.unwrap().to_string(), None).await;
                Msg::NextPage
            });
        }

        html! {
            <div style="overflow: 'hidden'">
                <div class="container">
                    if self.is_loading{
                        <div class="row">
                            <div class="col-md-12 text-center" style="margin-top: 250px;">
                                <label style="font-size:60px; font-weight: bold; color: #000947;">
                                    { self.get_value("please_wait") }
                                </label>
                            </div>
                        </div>
                    }
                    else{

                        <div class="row" style="margin-top: '40px'">
                        </div>
                        <div class="row" style="margin-top: 10px">
                            <div>
                                <img width={80} height={80} src="img/buttons/Home.png" />
                            </div>
                            <div style="width: 250px;margin-left: auto;margin-right: auto;text-align: center;">
                                //<img width=150 height=70 src="/img/evo.png"/>

                            </div>
                            <div>
                                <img width=150 height=70 src="/img/Logo.png"/>
                            </div>
                        </div>

                        <div class="row" style="margin-top: 10px;align: center;">
                            <canvas id="canvas">
                            </canvas>
                        </div>
                        <div class="row" style="margin-top:250px;">
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
                    }
                </div>
            </div>
        }
    }
}
