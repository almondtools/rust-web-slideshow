use crate::app::component::button::ButtonLikeLink;
use crate::util::style::use_style;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn Navigation<'a>(cx: Scope<'a>, on_next: EventHandler<'a, ()>) -> Element<'a> {
    let button_style = use_style(cx, || {
        r#"
            position: absolute;
            top: 0;
            bottom: 0;
            display: flex;
            align-items: center;
        "#
    });

    cx.render(rsx! {
        div { class: "{button_style}", style: "right: 0;",
            ButtonLikeLink { on_click: |_| on_next.call(()), "❯" }
        }
    })
}
