use crate::util::http::client::FetchHttpClient;
use dioxus::prelude::*;
use sdk::slide::{get_slides, Slides};
use std::rc::Rc;

pub fn use_slides(cx: &ScopeState) -> &UseFuture<Slides> {
    use_future!(cx, || {
        let http_client: Rc<FetchHttpClient> = cx.consume_context::<Rc<FetchHttpClient>>().unwrap();
        async move { get_slides(http_client.as_ref()).await }
    })
}
