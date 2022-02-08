use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Default)]
pub struct SelectMaterial {
    material: String,
    contract_list: Vec<String>,
    flitered_list: Vec<String>,
}
pub enum Msg {
    SelectMaterial(&'static str),
    GotHome,
    NextPage,
    PreviousPage,
}

impl Component for SelectMaterial {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            material: "".to_string(),
            contract_list: vec!["Material_1".to_string(), "Material_2".to_string()],
            flitered_list: vec!["Material_1".to_string(), "Material_2".to_string()],
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
                history.push(Route::SelectMaterial);
                true
            }
            Msg::GotHome => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::Root);
                true
            }
            Msg::SelectMaterial(x) => {
                if x == String::from("<-") {
                    self.material.pop();
                } else {
                    self.material += &*x;
                }
                self.flitered_list = Vec::new();

                let m = self.material.len();

                for i in &self.contract_list {
                    if i.len() >= m {
                        let temp = &i[..m];
                        log::info!("{}", temp.clone());
                        if self.material == temp {
                            self.flitered_list.push(i.clone());
                        }
                    }
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let key_one_line = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-"];
        let key_two_line = vec!["Q", "W", "E", "R", "T", "Z", "U", "I", "O", "P", "<-"];
        let key_three_line = vec!["A", "S", "D", "F", "G", "H", "J", "K", "L", "Ä", "Ö"];
        let key_four_line = vec!["Y", "X", "C", "V", "B", "N", "M", "Ü", ".", "SPACE"];
        //let all_line = vec![key_one_line, key_two_line, key_three_line, key_four_line];
        let home_cb = link.callback(move |_| Msg::GotHome);
        let back_cb = link.callback(move |_| Msg::PreviousPage);
        let next_cb = link.callback(move |_| Msg::NextPage);

        let render_item = |x: String| -> Html {
            html! {
                <>
                        <div class = "row p-2" style = "height:100%">
                            <div class = "col text-center" style = "height:100%;padding-top:60px;border-radius:15px;border: 1px solid black;background:#000947;color:white;font-size:20px">{x}</div>
                        </div>
                </>

            }
        };

        html! {
        <>
            <div class="container">
                <div class="row" style="margin-top: '10px'"></div>
                <div class="row" style="margin-top: 10px">
                    <div onclick={home_cb}>
                        <img width={80} height={80} src="img/buttons/Home.png" />
                    </div>
                    <div style="width: 250px;margin-left: auto;margin-right: auto;text-align: center;"></div>
                    <div><img width=150 height=70 src="/img/Logo.png" /></div>
                </div>

                <div class="row mt-4">
                    <input class="col-auto" style="border-radius:15px;width:1000px;height:50px;border: 1px solid black"
                        value={self.material.clone()} />
                </div>

                <div class="row mt-3">

                    <div class="col-auto" style="border-radius:15px;width:1000px;height:170px;border: 1px solid black;overflow:auto">

                        {
                            for self.flitered_list.iter().map(|st| render_item(st.clone()))
                        }

                    </div>

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
                            Msg::SelectMaterial(string)
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
                            Msg::SelectMaterial(string)
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
                            Msg::SelectMaterial(string)
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
                        let button_style= format!("border: 1px solid black; border-radius:4px; \
                        width: 70px; height: 70px;\
                        text-align: center; justify-content: center; min-width:{}; \
                        display: flex; align-items: center; color: #000947",space_val);
                        let val_click_cb = link.callback(move |_| {
                            if string == "SPACE"{
                                Msg::SelectMaterial(" ")
                            }
                            else{
                                Msg::SelectMaterial(string)
                            }
                        });
                        html!{
                        <div class="col" style={spacing} onclick={val_click_cb}>
                            <div style={button_style}>
                                {{char1}}
                            </div>
                        </div>
                        }
                        }).collect::<Html>()
                    }
                </div>

            </div>
            <div class="row" style="margin-top:10px">
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
        </>
        }
    }
}
