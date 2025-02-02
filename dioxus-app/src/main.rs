use crate::app::App;
use wasm_logger::Config;

mod app;
mod use_case;
mod style;

fn main() {
    wasm_logger::init(Config::default());
    dioxus_web::launch(App);
}
