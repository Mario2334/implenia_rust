use web_sys::console::log_1;
use yew::prelude::*;
use yew_router::prelude::{History, RouterScopeExt, use_history};
use yew_router::prelude::*;
use crate::routes::Route;
use crate::components::utils::set_get::set_lang;

#[derive(PartialEq, Properties)]
pub struct Props{
    pub src: String,
    pub width:i8,
    pub height: i8,
    pub language: String,
}

pub enum ImageStoreMessage {

    SetLanguage
}

pub struct ImageStore{
    pub src: String,
    pub width:i8,
    pub height: i8,
    pub language: String
}

impl Component for ImageStore {
    type Message = ImageStoreMessage;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            src: ctx.props().src.clone(),
            width: ctx.props().width.clone(),
            height: ctx.props().height.clone(),
            language: ctx.props().language.clone()
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ImageStoreMessage::SetLanguage => {
                let history = _ctx.link().history().unwrap();
                let lang_cl = self.language.as_str();
                set_lang(lang_cl);
                history.push(Route::BarcodeModel);
                true
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let _link = _ctx.link();
        let sel_lang_cb = _link.callback(|_|{
            ImageStoreMessage::SetLanguage
        });
        html!{
            <>
            <div style="
                        width: 150px;
                        height: 150px;
                        display: flex;
                        flex-direction: row;
                        flex-wrap: wrap;
                        margin-left: 10px;
                        margin-top: 10px"
            >
            <img style="
                        width: 150px; height: 150px; border: 1px solid black;
                        border-radius: 4px" src={self.src.clone()} onclick={sel_lang_cb}/>
            </div>
            </>
        }
    }
}

pub fn get_images1() -> Vec<ImageStore> {
    vec![
        ImageStore{
            src: "img/lang_images/DE.png".to_string(),
            width: 1,
            height: 1,
            language: "de".to_string()
        },
        ImageStore{
            src: "img/lang_images/GB.png".to_string(),
            width: 1,
            height: 1,
            language: "en".to_string()
        },
        ImageStore{
            src: "img/lang_images/FR.png".to_string(),
            width: 1,
            height: 1,
            language: "fr".to_string()
        },
        ImageStore{
            src: "img/lang_images/Netherlands.png".to_string(),
            width: 1,
            height: 1,
            language: "nf".to_string()
        },
        ImageStore{
            src: "img/lang_images/RO.png".to_string(),
            width: 1,
            height: 1,
            language: "ro".to_string()
        }
    ]
}

pub fn get_images2() -> Vec<ImageStore> {
    vec![
        ImageStore{
            src: "img/lang_images/TR.png".to_string(),
            width: 1,
            height: 1,
            language: "tr".to_string()
        },
        ImageStore{
            src: "img/lang_images/Bulgaria.png".to_string(),
            width: 1,
            height: 1,
            language: "bg".to_string()
        },
        ImageStore{
            src: "img/lang_images/Russia.png".to_string(),
            width: 1,
            height: 1,
            language: "rs".to_string()
        },
        ImageStore{
            src: "img/lang_images/Serbia.png".to_string(),
            width: 1,
            height: 1,
            language: "ser".to_string()
        },
        ImageStore{
            src: "img/lang_images/Poland.png".to_string(),
            width: 1,
            height: 1,
            language: "pl".to_string()
        }
    ]
}

pub fn get_images3() -> Vec<ImageStore> {
    vec![
        ImageStore{
            src: "img/lang_images/Czech_Republic.png".to_string(),
            width: 1,
            height: 1,
            language: "cz".to_string()
        },
        ImageStore{
            src: "img/lang_images/Hungary.png".to_string(),
            width: 1,
            height: 1,
            language: "hu".to_string()
        },
        ImageStore{
            src: "img/lang_images/Croatia.png".to_string(),
            width: 1,
            height: 1,
            language: "cr".to_string()
        }
    ]
}