use crate::app::App;
use wasm_logger::Config;

mod app;
mod use_case;
mod util;

fn main() {
    wasm_logger::init(Config::default());
    dioxus_web::launch(App);
}
