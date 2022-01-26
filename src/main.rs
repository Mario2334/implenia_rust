// #![feature(extern_types)]

mod routes;
mod pages;
mod components;

fn main(){
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<routes::Root>();
}