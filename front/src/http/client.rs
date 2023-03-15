use super::response::http_response_from;
use async_trait::async_trait;
use sdk::http::header::Accept;
use sdk::http::{GetRequest, HttpClient, HttpHeaders, HttpResponse, HttpStatus};
use sdk::slide::Slides;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Clone, PartialEq)]
pub struct FetchHttpClient {
    base_url: String,
}

impl FetchHttpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        FetchHttpClient {
            base_url: base_url.into(),
        }
    }

    fn path_to_api_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn fetch(
        &self,
        path: &str,
        headers: &HttpHeaders,
        opts: &mut RequestInit,
    ) -> HttpResponse {
        let url = self.path_to_api_url(path);
        let mut request = Request::new_with_str_and_init(url.as_str(), &opts).unwrap();
        add_headers(&mut request, headers);

        fetch(&request).await
    }

    pub async fn get_slides(&self) -> Slides {
        let request = GetRequest::to("/slides").with_header(Accept::JSON);
        let response = self.get(&request).await;
    
        match response.status {
            HttpStatus::Ok => {
                let body: Vec<u8> = response.body.expect("Expect get slides to return body");
                serde_json::from_slice(body.as_slice()).unwrap()
            }
            status => panic!("Get Slides returned unhandled status code='{:?}'", status),
        }
    }
    
}

#[async_trait(?Send)]
impl HttpClient for FetchHttpClient {
    async fn get(&self, request: &GetRequest) -> HttpResponse {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        self.fetch(&request.path, &request.headers, &mut opts).await
    }
}

fn add_headers(request: &mut Request, headers: &HttpHeaders) {
    headers.inner().into_iter().for_each(|header| {
        request.headers().set(header.0, header.1).unwrap();
    });
}

async fn fetch(request: &Request) -> HttpResponse {
    let promise = web_sys::window().unwrap().fetch_with_request(request);
    let response: Response = JsFuture::from(promise).await.unwrap().dyn_into().unwrap();
    http_response_from(response).await
}