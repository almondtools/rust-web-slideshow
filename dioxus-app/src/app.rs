use crate::use_case::use_slides;
use dioxus::prelude::*;
use front::http::client::FetchHttpClient;
use slideshow::Slideshow;
use std::rc::Rc;

mod component;
mod slideshow;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    use_context_provider(cx, || {
        Rc::new(FetchHttpClient::new("http://127.0.0.1:8000"))
    });
    //TODO: this component creates a context then consumes it and the context is never used again - seems not to be the intended usage pattern
    let slides = use_slides(cx);

    cx.render(match slides.value() {
        Some(slides) => rsx! { Slideshow { slides: slides } },
        None => rsx! { div { "Loading slides..." } },
    })
}
