use crate::components::*;
use dioxus::prelude::*;
use epic_kdl;

#[derive(Props)]
pub struct UnitProps<'a> {
    unit: &'a epic_kdl::Unit,
}

pub fn unit_card<'a>(cx: Scope<'a, UnitProps<'a>>) -> Element {
    cx.render(rsx! (
      div {
        class: "px-2",
        b_row {
          class: "border border-secondary",
          div {
            class: "col-12 col-lg-4 border-right-0 border-lg-right border-bottom border-lg-bottom-0",
            b_row {
              div {
                class: "col-12 border-bottom bg-secondary-subtle",
                div {
                  class: "flex flex-column text-align-center",
                  div {
                    class: "fw-bold fs-5 text-center",
                    "{cx.props.unit.name}"
                  }
                }
              }
              div {
                class: "col-12",
                b_row {
                  unit_card_stat {
                    title: "Type",
                    value: cx.props.unit.unit_type.clone()
                  }
                  unit_card_stat {
                    title: "Speed",
                    value: cx.props.unit.speed_or_ac_type()
                  }
                  unit_card_stat {
                    title: "Armour",
                    value: cx.props.unit.armour()
                  }
                  unit_card_stat {
                    title: "CC",
                    value: cx.props.unit.cc()
                  }
                  unit_card_stat {
                    title: "FF",
                    value: cx.props.unit.ff()
                  }
                }
              } 
            }
          }
          unit_weapons_or_loadouts {
            unit: cx.props.unit
          }
          unit_notes {
            unit: cx.props.unit
          }
        }
      }
    ))
}

#[inline_props]
fn unit_card_stat<'a>(cx: Scope, title: &'a str, value: String) -> Element {
    cx.render(rsx! (
      div {
        class: "col text-center",
        div {
          class: "flex flex-column",
          unit_stat_header {
            title: title
          }
          div {
            "{value}"
          }
        }
      }
    ))
}


#[derive(Props)]
pub struct UnitWeaponsOrLoadoutsProps<'a> {
    unit: &'a epic_kdl::Unit,
}


fn unit_weapons_or_loadouts<'a>(cx: Scope<'a, UnitWeaponsOrLoadoutsProps<'a>>) -> Element {
  let body = if !cx.props.unit.loadouts.is_empty() {
    cx.render(rsx! (unit_loadouts {
      loadouts: cx.props.unit.loadouts.iter().collect(),
    }))
  } else {
    cx.render(rsx! (unit_weapons {
      weapons: cx.props.unit.weapons.iter().collect(),
      cc: None,
      ff: None,
    }))
  };
  cx.render(rsx! (
    div {
      class: "col-lg col-md-12 pb-1 pb-lg-2",
      body
    }
  ))
}

#[derive(Props)]
pub struct UnitLoadoutsProps<'a> {
    loadouts: Vec<&'a epic_kdl::UnitLoadout>,
}


fn unit_loadouts<'a>(cx: Scope<'a, UnitLoadoutsProps<'a>>) -> Element {
  cx.render(rsx! (
    cx.props.loadouts.iter().enumerate().map(|(i, loadout)| {
      cx.render(rsx! (
        unit_loadout {
          index: i,
          loadout: loadout
        }
      ))
    })
  ))
}



#[derive(Props)]
pub struct UnitLoadoutProps<'a> {
  index: usize,
  loadout: &'a epic_kdl::UnitLoadout,
}


fn unit_loadout<'a>(cx: Scope<'a, UnitLoadoutProps<'a>>) -> Element {
  cx.render(rsx! (
    b_row {
      div {
        class: "col-12",
        span {
          class: "fw-bold",
          "{cx.props.loadout.name(cx.props.index + 1)}"
        }
      }
    }
    unit_weapons {
      weapons: cx.props.loadout.weapons.iter().collect(),
      cc: cx.props.loadout.cc,
      ff: cx.props.loadout.ff,
    }
  ))
}

const WEAPON_COLUMN: &str = "col-6 col-md-5 col-lg-4";
const RANGE_COLUMN: &str = "col-2 col-lg-2 col-xxl-1 text-center";
const FIREPOWER_COLUMN: &str = "col";

#[derive(Props)]
pub struct UnitWeaponsProps<'a> {
    weapons: Vec<&'a epic_kdl::UnitWeapon>,
    #[props(!optional)]
    cc: Option<u32>,
    #[props(!optional)]
    ff: Option<u32>
}


fn unit_weapons<'a>(cx: Scope<'a, UnitWeaponsProps<'a>>) -> Element {
  cx.render(rsx! (
    b_row {
      cx.props.cc.map(|_cc| rsx!(
        div {
          class: "col-1 text-center",
          unit_stat_header {
            title: "CC"
          }
        }
      ))
      cx.props.ff.map(|_ff| rsx!(
        div {
          class: "col-1 text-center",
          unit_stat_header {
            title: "FF"
          }
        }
      ))
      div {
        class: WEAPON_COLUMN,
        unit_stat_header {
          title: "Weapons"
        }
      }
      div {
        class: RANGE_COLUMN,
        unit_stat_header {
          title: "Range"
        }
      }
      div {
        class: FIREPOWER_COLUMN,
        unit_stat_header {
          title: "Firepower"
        }
      }
    }
    for weapon in cx.props.weapons.iter() {
      unit_weapon { 
        weapon: weapon,
        cc: cx.props.cc,
        ff: cx.props.ff
      }
    }
  ))
}

#[derive(Props)]
pub struct UnitWeaponProps<'a> {
    weapon: &'a epic_kdl::UnitWeapon,
    #[props(!optional)]
    cc: Option<u32>,
    #[props(!optional)]
    ff: Option<u32>
}

fn unit_weapon<'a>(cx: Scope<'a, UnitWeaponProps<'a>>) -> Element {
  let x = match cx.props.weapon.x {
    Some(x) => format!("{}x ", x),
    None => "".to_string(),
  };
  cx.render(rsx! (
    b_row {
      cx.props.cc.map(|cc| rsx!(
        div {
          class: "col-1 text-center",
          "{cc}+"
        }
      ))
      cx.props.ff.map(|ff| rsx!(
        div {
        class: "col-1 text-center",
        "{ff}+"
      }))
      div {
        class: WEAPON_COLUMN,
        "{x}{cx.props.weapon.name}"
      }
      div {
        class: RANGE_COLUMN,
        "{cx.props.weapon.range()}"
      }
      div {
        class: FIREPOWER_COLUMN,
        "{cx.props.weapon.firepower_as_string()}"
      }
    }
  ))
}

#[inline_props]
fn unit_stat_header<'a>(cx: Scope, title: &'a str) -> Element {
  cx.render(rsx! (
    div {
      class: "text-uppercase text-secondary-emphasis",
      small {
        class: "fw-light",
        "{title}"
      }
    }
  ))
}

#[derive(Props)]
struct UnitNotesProps<'a> {
  unit: &'a epic_kdl::Unit
}

fn unit_notes<'a>(cx: Scope<'a, UnitNotesProps<'a>>) -> Element {
  if cx.props.unit.notes.is_empty() {
     None
  } else {
     cx.render(rsx!(
       div {
        class: "col-12 border-top",
        b_row {
          class: "gx-0 m-0"
          div {
            class: "col-12",
            unit_stat_header {
              title: "Notes"
            }
          }
          b_row {
            class: "pb-1 gx-0 row-cols-1"
            unit_notes_special_rules { 
              rules: cx.props.unit.special_rules().collect()
            }
            unit_loadout_note {
              loadouts: cx.props.unit.loadouts.iter().collect()
            }
            unit_notes_dc {
              dc: cx.props.unit.damage_capacity()
            }
            unit_notes_transport {
              transport: cx.props.unit.transport()
            }
          }
        }
       }

     ))
  }
}

#[derive(Props)]
struct UnitNotesSpecialRulesProps<'a> {
  rules: Vec<&'a String>
}


fn unit_notes_special_rules<'a>(cx: Scope<'a, UnitNotesSpecialRulesProps<'a>>) -> Element {
  cx.render(rsx! (
    div {
      class: "col",
      for (i, rule) in cx.props.rules.iter().enumerate() {
        if i != 0 {
          ", "
        }
        span {
          class: "fst-italic epic-kdl-keyword",
          "{rule}"
        }
      }
    }
  ))
}

#[derive(Props)]
struct UnitNotesDCProps<'a> {
  #[props(!optional)]
  dc: Option<&'a epic_kdl::UnitDamageCapacity>
}

fn unit_notes_dc<'a>(cx: Scope<'a, UnitNotesDCProps<'a>>) -> Element {
  match cx.props.dc {
    Some(dc) => cx.render(rsx! (
      div {
        class: "col",
        span {
          class: "fst-italic epic-kdl-keyword",
          "Damage Capacity { dc.amount } "
        }
        "Critical Hit Effect: { dc.critical_hit }"
      }
    )),
    None => None
  }
}


#[derive(Props)]
struct UnitLoadoutNoteProps<'a> {
  #[props(!optional)]
  loadouts: Vec<&'a epic_kdl::UnitLoadout>
}

fn unit_loadout_note<'a>(cx: Scope<'a, UnitLoadoutNoteProps<'a>>) -> Element {
  if !cx.props.loadouts.is_empty() {
    let note = cx.props.loadouts
      .iter().map(|l| l.description())
      .collect::<Vec<String>>()
      .join("; or ");
    let cc_notes = cx.props.loadouts.iter().map(|l| l.cc_or_ff_postfix()).flatten().collect::<Vec<String>>();
    let cc_note = if !cc_notes.is_empty() {
      let cc_notes_str = cc_notes.join(". ");
      Some(rsx! (
        span {
          " {cc_notes_str}."
        }
      ))
    } else {
      None
    };
    cx.render(rsx! (
      div {
        class: "col",
        span {
          class: "fst-italic epic-kdl-keyword",
          "Weapon options "
        }
        "This unit can be armed with one of the options listed: "
        "{note}."
        cc_note
      }
    ))
  } else {
    None
  }
}


#[derive(Props)]
struct UnitNotesTransportProps<'a> {
  #[props(!optional)]
  transport: Option<&'a epic_kdl::UnitTransport>
}



fn unit_notes_transport<'a>(cx: Scope<'a, UnitNotesTransportProps<'a>>) -> Element {
  match cx.props.transport {
    Some(transport) => cx.render(rsx! (
      div {
        class: "col",
        span {
          class: "fst-italic epic-kdl-keyword",
          "Transport "
        } 
        "{ transport.description() }"
        "{ transport.text }"
      }
    )),
    None => None
  }
}