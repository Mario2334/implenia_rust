use crate::components::send_weight::*;
use crate::components::state::*;
use crate::components::utils::set_get::*;
use crate::routes::Route;
use log::log;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    GotHome,
    NextPage(bool),
    PreviousPage,
    UpdateLoading,
}

pub struct WeightViewModel {
    weight: String,
    loading: bool,
}

impl WeightViewModel {
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

    fn get_weight_title(&self) -> String {
        let transactions = get_transactions();
        if transactions.id.is_none() {
            return self.get_value("first_weight");
        } else {
            return self.get_value("second_weight");
        }
    }
}

impl Component for WeightViewModel {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let weight = get_weight_detail().weight;
        log::info!("{}", weight);
        WeightViewModel {
            weight,
            loading: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextPage(is_fw) => {
                let history = _ctx.link().history().unwrap();
                if is_fw {
                    history.push(Route::ThankYouModel);
                } else {
                    history.push(Route::SignatureModel);
                }
                false
            }
            Msg::PreviousPage => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::BarcodeModel);
                false
            }
            Msg::GotHome => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::Root);
                false
            }
            Msg::UpdateLoading => {
                self.loading = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let home_cb = link.callback(move |_| Msg::GotHome);
        let back_cb = link.callback(move |_| Msg::PreviousPage);
        let next_cb = link.callback(move |_| Msg::UpdateLoading);
        let contract = get_contract();
        log::info!("{}", contract.contract_number);
        log::info!("{}", get_license_plate());
        let lang_json_file = get_global_lang().clone();
        let history = link.history().unwrap();

        if lang_json_file.is_null() {
            let history = ctx.link().history().unwrap();
            history.push(Route::LanguageModel);
            return html! {<div></div>};
        }

        if self.loading {
            log::info!("loading is true");
            ctx.link().send_future(async move {
                let weight_type = get_weighing_type();

                match weight_type {
                    WeighingType::First => send_first_weight().await,
                    WeighingType::Second => {
                        history.push(Route::SignatureModel);
                    }
                    WeighingType::Tara => send_tara_weight().await,
                    WeighingType::TaraSava => send_tara_save_weight().await,
                }

                Msg::NextPage(true)
            });
        }

        html! {
            <>
            <div>
                <div class="container" style="height: 660px">
                    if self.loading{
                            <div class="row">
                                <div class="col-md-12 text-center" style="margin-top: 250px;">
                                    <label style="font-size:60px; font-weight: bold; color: #000947;">
                                        { self.get_value("please_wait") }
                                    </label>
                                </div>
                            </div>
                    }
                    else{
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
                    <div class="row" style="margin-top: 30px;">
                        <div class="col-md-12 text-center">
                            <div class="" style="margin-bottom:20px;">
                                <label style="font-size:45px; font-weight: bold; color: #000947;">
                                    { self.get_weight_title() }
                                </label>
                            </div>
                            <div class="" style="margin-top:100px;">
                                <label style="font-size:30px; font-weight: bold; color: #000947;">
                                    { self.weight.clone() + " kg" }
                                </label>
                            </div>
                        </div>
                    </div>

                    <div class="row" style="margin-top:250px">
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


                    }
                </div>
            </div>
            </>
        }
    }
}
