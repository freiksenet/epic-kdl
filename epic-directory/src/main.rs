use dioxus::prelude::*;
mod components;
use components::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn remove_spinner() {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("spinner")
        .unwrap()
        .remove();
}

fn main() {
    remove_spinner();
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| async move {
        let result = reqwasm::http::Request::get("/lists/codex-astartes.kdl")
            .send()
            .await
            .unwrap()
            .text()
            .await;
        result.map(epic_kdl::get_test_army_list)
    });
    cx.render(match future.value() {
        Some(Ok(list)) => rsx!(army_list { army_list: list }),
        Some(Err(_)) => rsx!(div { "Error, sorry."}),
        None => rsx!(loading_spinner {}),
    })
}

fn loading_spinner(cx: Scope) -> Element {
    cx.render(rsx!(
      div {
        id: "spinner",
        class: "vh-100 vw-100 d-flex justify-content-center align-items-center",
        div {
          class: "spinner-border",
          role: "status",
          span {
             class: "visually-hidden",
             "Loading..."
          }
        }
      }
    ))
}
