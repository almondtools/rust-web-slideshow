use crate::http::header::Accept;
use crate::http::GetRequest;
use crate::http::HttpClient;
use crate::http::HttpStatus;
use serde::{Deserialize, Serialize};

pub async fn get_slides(http_client: &dyn HttpClient) -> Slides {
    let request = get_slides_request();
    let response = http_client.get(&request).await;

    match response.status {
        HttpStatus::Ok => {
            let body: Vec<u8> = response.body.expect("Expect get slides to return body");
            serde_json::from_slice(body.as_slice()).unwrap()
        }
        status => panic!("Get Slides returned unhandled status code='{:?}'", status),
    }
}

fn get_slides_request() -> GetRequest {
    GetRequest::to("/slides").with_header(Accept::JSON)
}

pub type Slides = Vec<Slide>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Slide {
    pub number: u32,
    pub src: String,
    pub caption: String,
}
