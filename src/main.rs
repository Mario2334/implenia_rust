use yew::prelude::*;

#[function_component(Main)]
fn main_comp() -> Html {
    html! {
        <p> {{"Hello world"}} </p>
    }
}

fn main() {
    yew::start_app::<Main>();
}
