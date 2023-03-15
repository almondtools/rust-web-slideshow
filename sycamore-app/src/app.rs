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
    let slides = create_resource(cx, client.get_slides());
    view! { cx,
        (if let Some(slides) = slides.get().as_ref() {
            let slides = Rc::new(slides.clone());
            view!{cx, Slideshow(slides=slides)}
        } else {
            view!{cx, div { "Loading slides..." }}
        })
    }

}


#[derive(Prop)]
pub struct SlideshowProps {
    slides: Rc<Vec<Slide>>,
}


#[component]
pub fn Slideshow<G: Html>(cx: Scope, props: SlideshowProps) -> View<G> {
    let slide_number = create_signal(cx, 0usize);
    view! { cx,
        Slide(slide=Rc::new(props.slides[*slide_number.get()].clone()))
    }
}

#[derive(Prop)]
pub struct SlideProps {
    slide: Rc<Slide>,
}

pub fn Slide<G: Html>(cx: Scope, props: SlideProps) -> View<G> {
    view! { cx,
        img(src=(props.slide.src), style="width:100%")
    }
}
