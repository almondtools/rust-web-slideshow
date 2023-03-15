use crate::style::use_style;
use dioxus::prelude::*;
use navigation::Navigation;
use sdk::slide::Slide;
use slide::Slide;
use slide_number::SlideNumber;

mod navigation;
mod slide;
mod slide_number;

#[allow(non_snake_case)]
#[inline_props]
pub fn Slideshow<'a>(cx: Scope, slides: &'a Vec<Slide>) -> Element {
    let container_style = use_style(cx, || {
        r#"
             max-width: 1000px;
             position: relative;
             margin: auto;
        "#
    });

    let slide_number = use_state::<SlideNumber>(cx, || 1.into());

    cx.render(rsx! {
        div { class: "{container_style}",
            slide(slides, *slide_number.get()).map(|slide| rsx! {
                Slide { key: "{slide.number}", slide: slide.clone() }
            }),
            Navigation { on_next: |_| slide_number.set(slide_number.next(slides.len() as u32)) }
        }
    })
}

fn slide(slides: &[Slide], slide_number: SlideNumber) -> Option<&Slide> {
    slides
        .iter()
        .find(|slide| SlideNumber::from(slide.number) == slide_number)
}
