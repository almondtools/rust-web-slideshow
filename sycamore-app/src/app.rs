use std::rc::Rc;

use front::http::client::FetchHttpClient;
use sdk::slide::Slide;
use sycamore::futures::create_resource;
use sycamore::prelude::*;

pub fn launch() {
    sycamore::render(|cx| {
        provide_context(cx, FetchHttpClient::new("http://127.0.0.1:8000"));
        view! { cx,
            App {}
        }
    });
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let client = use_context::<FetchHttpClient>(cx);
    let maybe_slides = create_resource(cx, client.get_slides());
    let slides = create_signal(cx, Vec::new());

    view! { cx,
        (if let Some(found_slides) = maybe_slides.get().as_ref() {
            slides.set(found_slides.clone());
            view!{cx, Slideshow(slides=slides)}
        } else {
            view!{cx, div { "Loading slides..." }}
        })
    }
}

#[derive(Prop)]
pub struct SlideshowProps<'a> {
    slides: &'a ReadSignal<Vec<Slide>>,
}

#[component]
pub fn Slideshow<'a, G: Html>(cx: Scope<'a>, props: SlideshowProps<'a>) -> View<G> {
    let slide_number = create_signal(cx, 0usize);
    let slide = create_selector(cx, || {
        let number = *slide_number.get();
        let slides = props.slides.get();
        slides[number].clone()
    });
    view! { cx,
        Slide(slide=slide)
        Navigation(number=slide_number)
    }
}

#[derive(Prop)]
pub struct SlideProps<'a> {
    slide: &'a ReadSignal<Slide>,
}

pub fn Slide<'a, G: Html>(cx: Scope<'a>, props: SlideProps<'a>) -> View<G> {
    view! { cx,
        img(src=(props.slide.get().src), style="width:100%")
    }
}

#[derive(Prop)]
pub struct NavigationProps<'a> {
    number: &'a Signal<usize>,
}

pub fn Navigation<'a, G: Html>(cx: Scope<'a>, props: NavigationProps<'a>) -> View<G> {
    view! { cx,
        a(on:click=|_| { props.number.set((*props.number.get()+1) % 3)}){"label"}
    }
}
