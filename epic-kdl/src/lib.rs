mod army_list;
mod decode_utils;
mod templates;

pub use army_list::*;

use std::fs;

pub fn get_test_army_list(doc_str: String) -> ArmyList {
    let doc = knuffel::parse::<Vec<ArmyList>>("codex-astartes.kdl", &doc_str);
    doc.unwrap().pop().unwrap()
}
