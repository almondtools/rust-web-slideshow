#[derive(Clone, Debug, PartialEq)]
pub enum HttpStatus {
    Ok,
    Created,
    NotFound,
    Conflict,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub body: Option<Vec<u8>>,
}
