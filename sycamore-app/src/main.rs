use crate::app::launch;
use wasm_logger::Config;

mod app;

fn main() {
    wasm_logger::init(Config::default());
    launch();    
}