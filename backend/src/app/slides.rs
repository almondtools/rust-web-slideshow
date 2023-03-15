use rocket::serde::json::Json;
use sdk::slide::{Slide, Slides};

#[get("/slides", format = "json")]
pub async fn get_slides() -> Json<Slides> {
    Json(vec![
        Slide {
            number: 1,
            src: "assets/wald.png".into(),
            caption: "Wald".into(),
        },
        Slide {
            number: 2,
            src: "assets/schnee.png".into(),
            caption: "Schnee".into(),
        },
        Slide {
            number: 3,
            src: "assets/berge.png".into(),
            caption: "Berge".into(),
        },
    ])
}
