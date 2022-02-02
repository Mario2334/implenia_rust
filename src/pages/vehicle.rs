use crate::routes::Route;
use serde_derive::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Default)]
pub struct SelectVehicle {
    vehicle: String,
    vehicle_list: Vec<String>,
}
pub enum Msg {
    SelectVehicle(String),
    GotHome,
    NextPage,
    PreviousPage,
    AddVehicle,
}

impl Component for SelectVehicle {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            vehicle: "".to_string(),
            vehicle_list: Vec::new(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextPage => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::WeightViewModel);
                true
            }
            Msg::PreviousPage => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::BarcodeModel);
                true
            }
            Msg::GotHome => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::Root);
                true
            }
            Msg::SelectVehicle(x) => {
                if x == String::from("<-") {
                    self.vehicle.pop();
                } else {
                    self.vehicle += &*x;
                }
                true
            }
            Msg::AddVehicle => {
                if self.vehicle != "" {
                    self.vehicle_list.push(self.vehicle.clone());
                    self.vehicle = "".to_string();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let key_one_line = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-"];
        let key_two_line = vec!["Q", "W", "E", "R", "T", "Z", "U", "I", "O", "P", "<-"];
        let key_three_line = vec!["A", "S", "D", "F", "G", "H", "J", "K", "L", "Ä", "Ö"];
        let key_four_line = vec!["Y", "X", "C", "V", "B", "N", "M", "Ü", ".", "SPACE"];
        //let all_line = vec![key_one_line, key_two_line, key_three_line, key_four_line];
        let home_cb = link.callback(move |_| Msg::GotHome);
        let add_cb = link.callback(move |_| Msg::AddVehicle);
        html! {
            <>
                <div class = "container">
                    <div class="row" style="margin-top: '10px'"></div>
                    <div class="row" style="margin-top: 10px">
                            <div onclick={home_cb}>
                                <img width={80} height={80} src="img/buttons/Home.png" />
                            </div>
                            <div style="width: 250px;margin-left: auto;margin-right: auto;text-align: center;"></div>
                            <div><img width=150 height=70 src="/img/Logo.png"/></div>
                    </div>

                    <div class="row mt-4">
                        <input class="col" style = "border-radius:15px;width:700px;height:50px;border: 1px solid black" value = {self.vehicle.clone()}/>
                        <button class = "col-auto ml-2" style = "border-radius:7px;background:#000947;color:white"   onclick={add_cb}>{"Add new"}</button>
                    </div>

                    <div class="row mt-3">

                        <div class="col" style = "border-radius:15px;width:800px;height:220px;border: 1px solid black"></div>

                    </div>
                    <div class="row" style="margin-top:20px">
                    {
                        key_one_line
                        .iter()
                        .enumerate()
                        .map(|(i, char1)|{
                            let string = char1.clone();
                            let mut spacing = "";
                            if i > 0 {
                                spacing = "margin-left: -20px";
                            }
                            let val_click_cb = link.callback(move |_| {
                                Msg::SelectVehicle(string.to_string())
                            });
                            html!{
                            <div class="col" style={spacing} onclick={val_click_cb}>
                                <div style="border: 1px solid black; border-radius:4px; \
                                            width: 70px; height: 70px;\
                                            text-align: center; justify-content: center; \
                                            display: flex; align-items: center; color: #000947">
                                {{string}}
                                </div>
                            </div>
                            }
                    }).collect::<Html>()
                    }
                </div>
                <div class="row" style="margin-top:10px">
                    {
                        key_two_line
                        .iter()
                        .enumerate()
                        .map(|(i, char1)|{
                            let string = char1.clone();
                            let mut spacing = "";
                            if i > 0 {
                                spacing = "margin-left: -20px";
                            }
                            let val_click_cb = link.callback(move |_| {
                                Msg::SelectVehicle(string.to_string())
                            });
                            html!{
                            <div class="col" style={spacing} onclick={val_click_cb}>
                                <div style="border: 1px solid black; border-radius:4px; \
                                            width: 70px; height: 70px;\
                                            text-align: center; justify-content: center; \
                                            display: flex; align-items: center; color: #000947">
                                {{string}}
                                </div>
                            </div>
                            }
                    }).collect::<Html>()
                    }
                </div>
                <div class="row" style="margin-top:10px">
                    {
                        key_three_line
                        .iter()
                        .enumerate()
                        .map(|(i, char1)|{
                            let string = char1.clone();
                            let mut spacing = "";
                            if i > 0 {
                                spacing = "margin-left: -20px";
                            }
                            let val_click_cb = link.callback(move |_| {
                                Msg::SelectVehicle(string.to_string())
                            });
                            html!{
                            <div class="col" style={spacing} onclick={val_click_cb}>
                                <div style="border: 1px solid black; border-radius:4px; \
                                            width: 70px; height: 70px;\
                                            text-align: center; justify-content: center; \
                                            display: flex; align-items: center; color: #000947">
                                {{string}}
                                </div>
                            </div>
                            }
                    }).collect::<Html>()
                    }
                </div>
                <div class="row" style="margin-top:10px">
                    {
                        key_four_line
                        .iter()
                        .enumerate()
                        .map(|(i, char1)|{
                            let string = char1.clone();
                            let mut spacing = "";
                            if i > 0 {
                                spacing = "margin-left: -20px";
                            }
                            let mut space_val = "0";
                            if char1 == &"SPACE"{
                                space_val = "160px";
                            }
                            let buttonStyle= format!("border: 1px solid black; border-radius:4px; \
                                            width: 70px; height: 70px;\
                                            text-align: center; justify-content: center; min-width:{}; \
                                            display: flex; align-items: center; color: #000947",space_val);
                            let val_click_cb = link.callback(move |_| {
                                if string == "SPACE"{
                                    Msg::SelectVehicle(" ".to_string())
                                }
                                else{
                                    Msg::SelectVehicle(string.to_string())
                                }
                            });
                            html!{
                            <div class="col" style={spacing} onclick={val_click_cb}>
                                <div style={buttonStyle}>
                                {{char1}}
                                </div>
                            </div>
                            }
                    }).collect::<Html>()
                    }
                </div>


                </div>
            </>
        }
    }
}
