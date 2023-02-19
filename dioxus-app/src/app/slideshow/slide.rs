use dioxus::prelude::*;
use sdk::slide::Slide;

#[allow(non_snake_case)]
#[inline_props]
pub fn Slide(cx: Scope, slide: Slide) -> Element {
    cx.render(rsx! {
        div { img { src: "{slide.src}" } }
    })
}
