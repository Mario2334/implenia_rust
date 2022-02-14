use crate::{
    components::{
        constants::API_URL,
        model::Vehicle,
        request::get_request,
        state::{get_global_lang, set_vehicle},
        utils::set_get::get_lang,
    },
    pages::retry::RetryModel,
    routes::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Default)]
pub struct SelectVehicle {
    vehicle: String,
    vehicle_list: Vec<Vehicle>,
    flitered_list: Vec<Vehicle>,
    loading: bool,
}
pub enum Msg {
    SelectVehicle(&'static str),
    GotHome,
    NextPage,
    PreviousPage,
    AddVehicle,
    SetList(Vec<Vehicle>),
    UpdateLoading,
}

impl SelectVehicle {
    async fn get_vehicle_list() -> Vec<Vehicle> {
        let url = format!("{}api/Vehicle-View", API_URL);
        let result = get_request(&url, None).await.unwrap();
        let vals: Vec<Vehicle> = serde_json::from_value(result.clone()).unwrap();
        return vals;
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

impl Component for SelectVehicle {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_future(async {
            let vals = Self::get_vehicle_list().await;
            Msg::SetList(vals)
        });
        Self {
            vehicle: "".to_string(),
            vehicle_list: Vec::new(),
            flitered_list: Vec::new(),
            loading: false,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextPage => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::SelectContract);
                true
            }
            Msg::PreviousPage => {
                let history = _ctx.link().history().unwrap();
                history.push(Route::SelectVehicle);
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
                self.flitered_list = Vec::new();

                let m = self.vehicle.len();

                for i in &self.vehicle_list {
                    if i.license_plate.len() >= m {
                        let temp = &i.license_plate[..m];
                        log::info!("{}", temp.clone());
                        if self.vehicle == temp {
                            self.flitered_list.push(i.clone());
                        }
                    }
                }
                true
            }
            Msg::AddVehicle => {
                if self.vehicle != "" {
                    log::info!("{}", self.vehicle.clone());
                    self.vehicle_list.push(Vehicle {
                        license_plate: self.vehicle.clone(),
                        id: 0,
                    });
                    self.vehicle = "".to_string();
                    self.flitered_list = self.vehicle_list.clone();
                    true
                } else {
                    false
                }
            }
            Msg::SetList(vals) => {
                self.vehicle_list = vals.clone();
                self.flitered_list = vals.clone();
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
        let add_cb = link.callback(move |_| Msg::AddVehicle);
        let back_cb = link.callback(move |_| Msg::PreviousPage);
        let history = link.history().unwrap();
        const OVERIDE: &str = "
        display: block;
        margin: 0 auto;
        border-color: red;
        ";

        let render_item = |x: Vehicle| -> Html {
            let next_cb = link.callback(move |_| Msg::UpdateLoading);
            html! {
                <>
                        <div class = "row p-2" style = "height:100%">
                            <div onclick = {next_cb}  class = "col text-center" style = "height:100%;padding-top:60px;border-radius:15px;border: 1px solid black;background:#000947;color:white;font-size:20px">{x.license_plate}</div>
                        </div>
                </>

            }
        };

        if self.loading {
            let vehicle = self.flitered_list.get(0).unwrap().clone();
            log::info!("{}", vehicle.license_plate);
            // check for vehicle in yard
            link.send_future(async move {
                let url_1 = format!(
                    "{}api/Vehicle-View/?license_plate={}",
                    API_URL, vehicle.license_plate
                );
                let result_1 = get_request(&url_1, None).await;
                if result_1.as_ref().unwrap().get(0) != None {
                    let result_vehicle: Vehicle =
                        serde_json::from_value(result_1.unwrap().get(0).unwrap().clone()).unwrap();
                    let url_2 = format!(
                        "{}api/Transactions/?vehicle={}&trans_flag=0",
                        API_URL, result_vehicle.id
                    );
                    let result_2 = get_request(&url_2, None).await;
                    if result_2.as_ref().unwrap().get(0) != None {
                        history.push(Route::RetryModel);
                    }
                }
                set_vehicle(vehicle);
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
                            value={self.vehicle.clone()} />
                        <button class="col-auto ml-2" style="border-radius:7px;background:#000947;color:white"
                            onclick={add_cb}>{"Add new"}</button>
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
                                Msg::SelectVehicle(string)
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
                                Msg::SelectVehicle(string)
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
                                Msg::SelectVehicle(string)
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
                                    Msg::SelectVehicle(" ")
                                }
                                else{
                                    Msg::SelectVehicle(string)
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
            </div>
            </>
            }
        }
    }
}
