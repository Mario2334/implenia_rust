use yew_router::*;
use yew::{Html, html, function_component};
use crate::pages::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/language")]
    LanguageModel,
    #[at("/barcodescan")]
    BarcodeModel,
    #[at("/retry")]
    RetryModel
}


fn switch(route: &Route) -> Html {
    match route {
        Route::Root |
        Route::LanguageModel => html!{ <language::LanguageModel/> },
        Route::BarcodeModel => html!{ <barcode_scan::BarcodeModel/>},
        Route::RetryModel => html!{<retry::RetryModel />}
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