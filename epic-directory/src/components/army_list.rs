use crate::components::*;
use dioxus::prelude::*;
use epic_kdl;

#[derive(Props)]
pub struct ArmyListProps<'a> {
    army_list: &'a epic_kdl::ArmyList,
}

pub fn army_list<'a>(cx: Scope<'a, ArmyListProps<'a>>) -> Element {
    cx.render(rsx! (
        div {
            class: "container",

            army_list_title {}

            army_list_section {
              title: "Formations",
              army_list_formations {}
            }

            army_list_section {
              title: "Forces",
              for force in cx.props.army_list.forces.iter() {
                army_list_forces {
                  forces: force
                }
              }
            }
        }
    ))
}

fn army_list_title(cx: Scope) -> Element {
    cx.render(rsx! (
      b_row {
        class: "mb-4",
        div {
          class: "col",
          h1 {
            "Army Name TODO"
          }
          blockquote {
            "Fluff TODO"
          }
        }
      }
    ))
}

#[derive(Props)]
struct ArmyListSectionProps<'a> {
    #[props(into)]
    title: String,
    children: Element<'a>,
}

fn army_list_section<'a>(cx: Scope<'a, ArmyListSectionProps<'a>>) -> Element {
    cx.render(rsx! (
      b_row {
        class: "mb-4 gy-2",
        div {
          class: "col-12",
          h2 { "{cx.props.title}" }
        }
        &cx.props.children
      }
    ))
}

fn army_list_formations(cx: Scope) -> Element {
    cx.render(rsx!(div {}))
}

#[derive(Props)]
struct ArmyListForcesProps<'a> {
    forces: &'a epic_kdl::Forces,
}

fn army_list_forces<'a>(cx: Scope<'a, ArmyListForcesProps<'a>>) -> Element {
    cx.render(rsx!(
      Fragment {
      h3 {
        "{cx.props.forces.name}"
      }
      b_row {
        class: "gy-3",
        for unit in cx.props.forces.units.iter() {
          unit_card {
            unit: unit
          }
        }
      }
    }))
}
