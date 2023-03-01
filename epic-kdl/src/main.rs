use askama::Template;
use std::fs;

mod army_list;
mod templates;

use army_list::*;

fn main() -> miette::Result<()> {
    let doc_str = fs::read_to_string("./lists/codex-astartes.kdl").expect("Oh my");
    let doc = knuffel::parse::<Vec<ArmyList>>("codex-astartes.kdl", &doc_str);
    match doc {
        Ok(a) => {
            let template = templates::ListTemplate {
                army_list: a.get(0).unwrap(),
                fluff: "TODO",
            };
            println!("{}", template.render().unwrap());
            Ok(())
        }
        Err(e) => miette::Result::Err(miette::Report::new(e)),
    }
}
