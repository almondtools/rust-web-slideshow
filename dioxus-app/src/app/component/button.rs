use crate::util::style::use_style;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn ButtonLikeLink<'a>(
    cx: Scope<'a>,
    on_click: EventHandler<'a, ()>,
    children: Element<'a>,
) -> Element<'a> {
    let style = use_style(cx, || {
        r#"
            cursor: pointer;
            padding: 1rem;
            color: white;
            font-weight: bold;
            font-size: 18px;
            transition: 0.6s ease;
            user-select: none;
            background-color: rgba(0,0,0,0.3);

            &:hover {
                background-color: rgba(0,0,0,0.8);
            }
        "#
    });

    cx.render(rsx! {
        a { class: "{style}", onclick: |_| on_click.call(()), children }
    })
}
