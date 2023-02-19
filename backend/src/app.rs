use crate::util::rocket::cors::Cors;
use rocket::{Build, Rocket};
use slides::get_slides;

mod slides;

pub fn init() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![get_slides])
        .attach(Cors {
            allowed_origins: vec!["http://localhost:8080"],
        })
}
