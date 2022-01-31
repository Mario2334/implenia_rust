use log::log;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::state::*;
use crate::components::utils::set_get::*;
use crate::routes::Route;
use crate::components::model::{WeightResponse, Transactions, TransactionPDFRequest};
use crate::components::request::{get_request, post_request, put_request};
use std::collections::HashMap;
use crate::components::constants::API_URL;


pub enum Msg{
    GotHome,
    NextPage(bool),
    PreviousPage,
    UpdateLoading,
}

pub struct WeightViewModel{
    weight: String,
    loading: bool
}

impl WeightViewModel {
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

    fn get_weight_title(&self) -> String {
        let transactions = get_transactions();
        if transactions.id.is_none(){
            return self.get_value("first_weight")
        }
        else {
            return self.get_value("second_weight")
        }
    }
}

impl Component for WeightViewModel {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let weight = get_weight_detail().weight;
        log::info!("{}",weight);
        WeightViewModel{
            weight,
            loading: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            
            Msg::NextPage(is_fw) => {
                let history = _ctx.link().history().unwrap();
                if is_fw{
                    history.push(Route::ThankYouModel);
                }
                else {
                    history.push(Route::SignatureModel);
                }
                false
            }
            Msg::PreviousPage => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::BarcodeModel);
                false
            }
            Msg::GotHome =>{
                let history = _ctx.link().history().unwrap();
                history.push(Route::Root);
                false
            }
            Msg::UpdateLoading =>{
                self.loading = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let home_cb = link.callback(move |_| Msg::GotHome );
        let back_cb = link.callback(move |_| Msg::PreviousPage);
        let next_cb = link.callback(move |_| Msg::UpdateLoading);
        let contract = get_contract();
        log::info!("{}",contract.contract_number);
        log::info!("{}",get_license_plate());
        let lang_json_file = get_global_lang().clone();

        if lang_json_file.is_null() {
            let history = ctx.link().history().unwrap();
            history.push(Route::LanguageModel);
            return html!{<div></div>}
        }

        if self.loading {
            log::info!("loading is true");
            ctx.link().send_future(async {
                let weight_detail = get_weight_detail();
                let id_detail = get_id();
                log::info!("{}",id_detail.ident.as_ref().is_none());
                let date = format!("20{}-{}-{}",&weight_detail.date[6..8],&weight_detail.date[3..5],&weight_detail.date[0..2]);
                let datetime = format!("{}T{}:00",date, weight_detail.time);
                let mut transactions = Transactions::default();
                if id_detail.ident != None {
                    transactions.first_weight = Some(weight_detail.weight.to_string());
                    transactions.firstw_date_time = Some(datetime.to_string());
                    transactions.net_weight = Some(weight_detail.weight.to_string());
                    transactions.firstw_alibi_nr = Some(weight_detail.alibi_nr.to_string());
                    transactions.vehicle = id_detail.vehicle;
                    transactions.article = id_detail.article;
                    transactions.customer = id_detail.customer;
                    transactions.supplier = id_detail.supplier;
                    transactions.yard = Some(1);
                    transactions.trans_flag = Some(0);
                    transactions.combination_id = id_detail.ident;
                    let url = &format!("{}/api/Transactions/",API_URL);
                    let data = serde_json::to_string(&transactions);
                    let response = post_request(&url, &data.unwrap().to_string()).await;
                    let response_transaction:Transactions = serde_json::from_value(response.unwrap()).unwrap();

                    // Request PDF API
                    let pdf_request_body =TransactionPDFRequest {
                        id: response_transaction.id.unwrap()
                    };
                    let pdf_request_url = &format!("{}/api/pdf_backend",API_URL);
                    let data = serde_json::to_string(&pdf_request_body);
                    let response = post_request(&pdf_request_url, &data.unwrap().to_string()).await;
                    log::info!("Response {:?}",response_transaction);
                    return Msg::NextPage(true);
                }
                else {
                    transactions = get_transactions();
                    transactions.second_weight = Some(weight_detail.weight.to_string());
                    transactions.secondw_alibi_nr = Some(weight_detail.alibi_nr.to_string());
                    transactions.secondw_date_time = Some(datetime.to_string());
                    transactions.trans_flag = Some(1);
                    transactions.net_weight = Some(get_net_weight(weight_detail.weight.to_string(),transactions.clone()));

                    let url = &format!("{}/api/Transactions/{}/",API_URL,transactions.id.unwrap());
                    let data = serde_json::to_string(&transactions);
                    let response = put_request(&url, &data.unwrap().to_string()).await;
                    let response_transaction:Transactions = serde_json::from_value(response.unwrap()).unwrap();
                    log::info!("{}",response_transaction.id.unwrap());
                    set_transactions(response_transaction);
                    return Msg::NextPage(false);
                }
            });
        }

        html!{
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