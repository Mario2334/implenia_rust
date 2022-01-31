use std::default;

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
