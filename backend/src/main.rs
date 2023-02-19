mod app;
mod logger;
mod util;

#[macro_use]
extern crate rocket;

fn main() {
    logger::init();
    let app = app::init();
    let _ = rocket::execute(app.launch()).unwrap();
}
