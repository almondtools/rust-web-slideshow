use dioxus::prelude::*;
use stylist::Style;

pub fn use_style(
    cx: &ScopeState,
    initial_state_fn: impl FnOnce() -> &'static str,
) -> &UseState<String> {
    use_state::<String>(cx, || {
        Style::new(initial_state_fn())
            .expect("Failed to mount style")
            .get_class_name()
            .to_string()
    })
}
