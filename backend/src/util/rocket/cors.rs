use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::{fairing, Build, Request, Response, Rocket};

pub struct Cors {
    pub allowed_origins: Vec<&'static str>,
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Ignite | Kind::Response,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        Ok(rocket.mount("/", routes![preflight_route]))
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, response: &mut Response<'r>) {
        if let Some(origin) = req.headers().get_one("Origin") {
            if !self.allowed_origins.contains(&origin) {
                return;
            }

            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "*"));
            response.set_header(Header::new("Access-Control-Allow-Origin", origin));
        }
    }
}

#[options("/<_..>")]
fn preflight_route() -> Status {
    Status::NoContent
}
