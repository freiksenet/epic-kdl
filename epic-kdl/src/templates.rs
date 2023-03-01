use crate::army_list;
use askama::Template;

#[derive(Template)]
#[template(path = "../templates/list.jinja")]
pub struct ListTemplate<'a> {
    pub army_list: &'a army_list::ArmyList,
    pub fluff: &'a str,
}
