use dioxus::prelude::*;

#[derive(Props)]
pub struct RowProps<'a> {
    #[props(into, default)]
    class: String,
    children: Element<'a>,
}

pub fn b_row<'a>(cx: Scope<'a, RowProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "row {cx.props.class}",
            &cx.props.children
        }
    ))
}
