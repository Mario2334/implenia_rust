use crate::{
    components::{
        constants::{API_URL, DEVMAN_URL},
        model::{Material, WeightResponse},
        request::get_request,
        state::{get_global_lang, set_material, set_weight_detail},
        utils::set_get::get_lang,
    },
    routes::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;

use super::retry;

#[derive(Default)]
pub struct SelectMaterial {
    material: String,
    material_list: Vec<Material>,
    flitered_list: Vec<Material>,
    loading: bool,
}
pub enum Msg {
    SelectMaterial(&'static str),
    GotHome,
    NextPage,
    PreviousPage,
    SelectMaterialList(Vec<Material>),
    UpdateLoading,
}

impl SelectMaterial {
    async fn get_material_list() -> Vec<Material> {
        let url = format!("{}api/article-View/", API_URL);
        let result = get_request(&url, None).await.unwrap();
        let materials: Vec<Material> = serde_json::from_value(result).unwrap();
        return materials;
    }
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
}

impl Component for SelectMaterial {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_future(async {
            let materials = Self::get_material_list().await;
            Msg::SelectMaterialList(materials)
        });
        Self {
            material: "".to_string(),
            material_list: vec![],
            flitered_list: vec![],
            loading: false,
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

                for i in &self.material_list {
                    if i.name.as_ref().unwrap().len() >= m {
                        let temp = &i.name.as_ref().unwrap()[..m];
                        if self.material == temp {
                            self.flitered_list.push(i.clone());
                        }
                    }
                }

                true
            }
            Msg::SelectMaterialList(x) => {
                self.material_list = x.clone();
                self.flitered_list = x;
                true
            }
            Msg::UpdateLoading => {
                self.loading = true;
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
        let next_cb = link.callback(move |_| Msg::UpdateLoading);

        let render_item = |x: Material| -> Html {
            let next_cb = link.callback(move |_| Msg::UpdateLoading);

            html! {
                <>
                        <div class = "row p-2" style = "height:100%">
                            <div  onclick = {next_cb}  class = "col text-center" style = "height:100%;padding-top:60px;border-radius:15px;border: 1px solid black;background:#000947;color:white;font-size:20px">{x.name.unwrap()}</div>
                        </div>
                </>

            }
        };
        const OVERIDE: &str = "
        display: block;
        margin: 0 auto;
        border-color: red;
        ";
        if self.loading {
            let material = self.flitered_list.get(0).unwrap().clone();
            set_material(material.clone());
            link.send_future(async move {
                let websocket_url = &format!("{}?cmd=GET WEIGHTNM", DEVMAN_URL);
                let weight_response = get_request(websocket_url, None).await;
                let weight_data = weight_response.unwrap().clone();
                let weight_response: WeightResponse = serde_json::from_value(weight_data).unwrap();
                log::info!("{}", weight_response.weight);
                set_weight_detail(weight_response.clone());

                Msg::NextPage
            });
            html! {
            <div style="margin-top: 30px">
                // <ClockLoader color={'#000947'} loading={true} css={override} size={50} id='loaderone' />
                <div style={OVERIDE} class="spinner-border text-primary" role="status">
                  <span class="sr-only">{"Loading..."}</span>
                </div>
                <label style="font-size: 40px; font-weight: bold; color: #000947; margin-left: 400px; margin-top: 25px" >{self.get_value("please_wait") + "..."}</label>
            </div>
            }
        } else {
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
}
