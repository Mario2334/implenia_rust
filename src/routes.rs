use std::collections::HashMap;
use std::default;

use crate::components::constants::API_URL;
use crate::components::model::{Token, User};
use crate::components::request::post_request;
use crate::components::state::{get_token, set_token};
use crate::pages::*;
use yew::{function_component, html, Html};
use yew_router::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/language")]
    LanguageModel,
    #[at("/barcodescan")]
    BarcodeModel,
    #[at("/barcodeinput")]
    BarcodeInputModel,
    #[at("/licenseplate")]
    LicensePlateModel,
    #[at("/showlicenseplate")]
    LicensePlateViewModel,
    #[at("/showweight")]
    WeightViewModel,
    #[at("/thankyou")]
    ThankYouModel,
    #[at("/signature")]
    SignatureModel,
    #[at("/retry")]
    RetryModel,
    #[at("/selectvehicle")]
    SelectVehicle,
    #[at("/selectcontract")]
    SelectContract,
    #[at("/selectmaterial")]
    SelectMaterial,
    #[at("/processdirection")]
    ProcessDirection,
    #[at("/enterpin")]
    EnterPinModel,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Root | Route::LanguageModel => html! { <language::LanguageModel/> },
        Route::BarcodeModel => html! { <barcode_scan::BarcodeModel/>},
        Route::BarcodeInputModel => html! {<barcode_input::BarcodeInputModel/>},
        Route::LicensePlateModel => html! {<license_plate::LicensePlateModel />},
        Route::LicensePlateViewModel => html! {<license_plate_view::LicensePlateView />},
        Route::WeightViewModel => html! {<weight_view::WeightViewModel />},
        Route::ThankYouModel => html! {<thankyou::ThankYouModel />},
        Route::SignatureModel => html! {<signature::SignatureModel />},
        Route::RetryModel => html! {<retry::RetryModel />},
        Route::SelectVehicle => html! {<vehicle::SelectVehicle/>},
        Route::SelectContract => html! {<contract::SelectContract/>},
        Route::SelectMaterial => html! {<material::SelectMaterial/>},
        Route::ProcessDirection => html! {<process_direction::ProcessModel/>},
        Route::EnterPinModel => html! {<enterpin::EnterPinModel/>},
    }
}

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}
