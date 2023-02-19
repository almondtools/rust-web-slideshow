use sdk::http::{HttpResponse, HttpStatus};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

pub async fn http_response_from(response: Response) -> HttpResponse {
    let status = status_from(&response);
    let js_body = JsFuture::from(response.array_buffer().unwrap())
        .await
        .unwrap();
    let body = js_sys::Uint8Array::new(&js_body).to_vec();
    HttpResponse {
        status,
        body: Some(body),
    }
}

fn status_from(response: &Response) -> HttpStatus {
    use HttpStatus::*;
    match response.status() {
        200 => Ok,
        201 => Created,
        404 => NotFound,
        409 => Conflict,
        code => panic!("Mapping for status code='{}' not implemented", code),
    }
}
