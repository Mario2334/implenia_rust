use yew::{Component, Context, Html, html};
use yew_router::prelude::*;
use crate::components::state::get_global_lang;
use crate::routes::Route;

pub enum Msg{
    NextPage(Route)
}

pub struct ProcessModel{

}

impl Component for ProcessModel {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ProcessModel{

        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let history = ctx.link().history().unwrap();
        match msg {
            Msg::NextPage(route) => {
                history.push(route);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let input_cb = link.callback( move |_|{
            Msg::NextPage(Route::SelectMaterial)
        });
        let output_cb = link.callback( move |_|{
           Msg::NextPage(Route::SelectContract)
        });

        let lang_json_file = get_global_lang().clone();
        if lang_json_file.is_null() {
            let history = ctx.link().history().unwrap();
            history.push(Route::LanguageModel);
            return html! {<div></div>};
        }

        html!{
            <div style="overflow: 'hidden'">
                <div class="container">
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
                    <div class="row" style="margin-top: 250px;margin-left: 100px;align: center;">
                        <div class="col center-text">
                            <button class="btn btn-primary btn-lg active" onclick={input_cb} style="width: 150px">{"Input"}</button>
                        </div>
                        <div class="col center-text">
                            <button class="btn btn-primary btn-lg active" onclick={output_cb} style="width: 150px">{"Output"}</button>
                        </div>
                    </div>
                </div>
            </div>

        }
    }
}