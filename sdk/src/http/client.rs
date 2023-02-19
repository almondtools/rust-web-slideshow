use super::request::GetRequest;
use super::response::HttpResponse;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait HttpClient {
    async fn get(&self, request: &GetRequest) -> HttpResponse;
}
