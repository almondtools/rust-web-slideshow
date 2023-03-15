use dioxus::prelude::*;
use front::http::client::FetchHttpClient;
use sdk::slide::Slides;
use std::rc::Rc;

pub fn use_slides(cx: &ScopeState) -> &UseFuture<Slides> {
    use_future!(cx, || {
        let http_client: Rc<FetchHttpClient> = cx.consume_context::<Rc<FetchHttpClient>>().unwrap();
        async move { http_client.get_slides().await }
    })
}