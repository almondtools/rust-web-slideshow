use crate::http::headers::{HttpHeader, HttpHeaders};

#[derive(Clone, Debug, PartialEq)]
pub struct GetRequest {
    pub path: String,
    pub headers: HttpHeaders,
}

impl GetRequest {
    pub fn to(path: impl Into<String>) -> Self {
        GetRequest {
            path: path.into(),
            headers: HttpHeaders::empty(),
        }
    }

    pub fn with_header(mut self, header: impl Into<HttpHeader>) -> Self {
        self.headers = self.headers.add_header(header);
        self
    }
}
