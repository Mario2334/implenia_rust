use crate::components::constants::{API_URL, MULTISCALE, ONLY_ID, SCALE};
use crate::components::model::{TransactionPDFRequest, Transactions, WeightResponse};
use crate::components::request::{get_request, post_request, put_request};
use crate::components::state::*;
use crate::components::utils::set_get::*;
use crate::routes::Route;
use log::log;
use std::collections::HashMap;
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
        let link = ctx.link();
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
            log::info!("loading is true");
            ctx.link().send_future(async {
                let weight_detail = get_weight_detail();
                let mut vehicle = get_vehicle();
                let mut id = get_id();
                if vehicle.license_plate != "" {
                    let vehicle_id = vehicle.id;
                    let url = format!("{}api/Vehicle-View/", API_URL);
                    let body = format!(
                        "{{\"license_plate\":\"{}\"}}",
                        vehicle.license_plate.clone()
                    );
                    log::info!("{}", body.clone());
                    if vehicle_id == 0 {
                        // post
                        let result = post_request(&url, &body, None).await.unwrap();
                        vehicle = serde_json::from_value(result).unwrap();
                    } else {
                        // put
                        let url = format!("{}api/Vehicle-View/{}/", API_URL, vehicle.id);
                        let _result = put_request(&url, &body).await.unwrap();
                    }

                    if id.vehicle.is_none() {
                        id.vehicle = Some(vehicle.id);
                        let url = format!("{}api/ID/{}/", API_URL, id.id.unwrap());
                        let body = format!(
                            "{{
                                \"ident\":\"{}\",
                                \"vehicle\":\"{}\"

                        }}",
                            id.ident.unwrap(),
                            id.vehicle.unwrap()
                        );
                        let result = put_request(&url, &body).await.unwrap();
                        id = serde_json::from_value(result).unwrap();
                    }
                }
                /*
                            tras_data = {
                            "first_weight": BaseLayout.weight,
                            'vehicle': BaseLayout.id_details["vehicle"],
                            'article': BaseLayout.firstweight_data["material_id"],
                            'customer': BaseLayout.id_details["customer"],
                            'supplier': BaseLayout.id_details["supplier"],
                            "net_weight": BaseLayout.weight,
                            "firstw_date_time": str(BaseLayout.datetime.date())+"T"+str(BaseLayout.datetime.time())+"Z",
                            "firstw_alibi_nr": BaseLayout.alibi_nr.replace(" ", ""),
                            "combination_id": BaseLayout.id_details["ident"],
                            "trans_flag": 0,
                            "yard": 1
                        }
                         sup_id = ""
                if len(BaseLayout.contract_details["supplier"]) > 0:
                    sup_id = BaseLayout.contract_details["supplier"][0]["id"]
                else:
                    sup_id = None
                tras_data["forwarders"] = BaseLayout.id_details["forwarders"]
                tras_data["customer"] = BaseLayout.contract_details["customer"]["id"]
                tras_data["supplier"] = sup_id
                tras_data["contract_number"] = BaseLayout.contract_details["contract_number"]
                 if BaseLayout.MULTISCALE:
                tras_data["scale_nr"] = BaseLayout.SCALE

                */
                let date = format!(
                    "20{}-{}-{}",
                    &weight_detail.date[6..8],
                    &weight_detail.date[3..5],
                    &weight_detail.date[0..2]
                );
                let datetime = format!("{}T{}:00", date, weight_detail.time);
                let mut trans = Transactions::default();
                trans.first_weight = Some(weight_detail.clone().weight);
                trans.vehicle = id.vehicle;
                trans.article = Some(get_material().id.unwrap() as i32);
                trans.customer = id.customer;
                trans.supplier = id.supplier;
                trans.net_weight = Some(weight_detail.weight);
                trans.combination_id = id.ident;
                trans.yard = Some(1);
                trans.trans_flag = Some(0);
                trans.firstw_alibi_nr = Some(weight_detail.alibi_nr.to_string());
                trans.firstw_date_time = Some(datetime);
                if ONLY_ID == false {
                    let contract = get_contract();
                    let mut sup_id: Option<i32> = None;
                    if contract.supplier.is_some() && contract.supplier.as_ref().unwrap().len() > 0
                    {
                        sup_id = contract.supplier.unwrap().get(0).unwrap().id;
                    }
                    trans.forwarders = id.forwarders;
                    trans.customer = Some(contract.customer.unwrap().id.unwrap() as i32);
                    trans.supplier = sup_id;
                    trans.contract_number = Some(contract.contract_number);
                }
                if MULTISCALE == true {
                    trans.scale_nr = Some(SCALE);
                }
                let url = format!("{}api/Transactions/", API_URL);
                let body = serde_json::to_string(&trans).unwrap();
                let result = post_request(&url, &body, None).await;

                Msg::NextPage(false)
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
