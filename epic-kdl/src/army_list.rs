use std::fmt;

#[derive(Debug, knuffel::Decode)]
pub struct ArmyList {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(property)]
    pub schema: String,
    #[knuffel(property)]
    pub source: Option<String>,
    #[knuffel(property)]
    pub version: Option<String>,

    #[knuffel(children(name = "text"))]
    pub texts: Option<Vec<Text>>,

    #[knuffel(children(name = "formation-group"), default)]
    pub formation_groups: Vec<FormationGroup>,

    #[knuffel(children(name = "upgrades"), default)]
    pub upgrades: Vec<Upgrades>,

    #[knuffel(children(name = "forces"), default)]
    pub forces: Vec<Forces>,
}

#[derive(Debug, knuffel::Decode)]
pub struct Text {
    #[knuffel(argument)]
    pub content: String,

    #[knuffel(property)]
    pub tag: String,
}

#[derive(Debug, knuffel::Decode)]
pub struct FormationGroup {
    #[knuffel(argument)]
    pub name: String,

    #[knuffel(child, unwrap(argument))]
    pub help_text: Option<String>,

    #[knuffel(child)]
    pub limit: Option<ArmyListLimit>,

    #[knuffel(children(name = "formation-group"), default)]
    pub nested_formation_groups: Vec<FormationGroup>,
    #[knuffel(children(name = "formation"), default)]
    pub formations: Vec<Formation>,
}

#[derive(Debug, knuffel::Decode)]
pub struct ArmyListLimit {
    #[knuffel(children)]
    pub limits: Vec<ArmyListLimitItem>,
}

#[derive(Debug, knuffel::Decode)]
pub enum ArmyListLimitItem {
    MaxPoints(#[knuffel(argument)] f32),
    MaxList(#[knuffel(argument)] u32),
}

#[derive(Debug, knuffel::Decode)]
pub struct Formation {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(child, unwrap(argument))]
    pub help_text: Option<String>,
    #[knuffel(child, unwrap(argument))]
    pub cost: u32,
    #[knuffel(child, unwrap(children(name = "unit")), default)]
    pub units: Vec<FormationUnit>,
    #[knuffel(child, unwrap(children(name = "upgrade")), default)]
    pub upgrades: Vec<FormationUpgrade>,
    #[knuffel(child, unwrap(children(name = "tag")), default)]
    pub tags: Vec<FormationTag>,
}

#[derive(Debug, knuffel::Decode)]
pub struct FormationUnit {
    #[knuffel(argument)]
    pub count: u32,
    #[knuffel(arguments)]
    pub units: Vec<String>,
}

impl fmt::Display for FormationUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = english_numbers::convert(
            self.count as i64,
            english_numbers::Formatting {
                title_case: false,
                spaces: true,
                conjunctions: true,
                ..Default::default()
            },
        );
        if self.units.len() > 1 {
            write!(
                f,
                "any {} of the following units: {}",
                count,
                self.units.join(", ")
            )
        } else {
            write!(f, "{} {} units", count, self.units.join(" "))
        }
    }
}

#[derive(Debug, knuffel::Decode)]
pub struct FormationUpgrade {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(property)]
    pub max: Option<u32>,
}

impl fmt::Display for FormationUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.max {
            Some(max) => write!(f, "0-{} {}", max, self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Debug, knuffel::Decode)]
pub struct FormationTag {
    #[knuffel(argument)]
    pub tag: String,
}

#[derive(Debug, knuffel::Decode)]
pub struct Upgrades {
    #[knuffel(argument)]
    pub name: String,

    #[knuffel(child, unwrap(argument))]
    pub help_text: Option<String>,

    #[knuffel(children(name = "upgrades"), default)]
    pub nested_upgrades: Vec<Upgrades>,
    #[knuffel(children(name = "upgrade"), default)]
    pub upgrades: Vec<Upgrade>,
}

#[derive(Debug, knuffel::Decode)]
pub struct Upgrade {
    #[knuffel(argument)]
    pub name: String,

    #[knuffel(child, unwrap(argument))]
    pub cost: u32,

    #[knuffel(child)]
    pub limit: Option<ArmyListLimit>,

    #[knuffel(children(name = "replace"), default)]
    pub replace: Vec<UpgradeReplace>,
    #[knuffel(children(name = "unit"), default)]
    pub unit: Vec<FormationUnit>,
}

#[derive(Debug, knuffel::Decode)]
pub struct UpgradeReplace {
    #[knuffel(argument)]
    pub count: u32,
    #[knuffel(argument)]
    pub from: String,
    #[knuffel(argument)]
    pub to: String,
}

#[derive(Debug, knuffel::Decode)]
pub struct Forces {
    #[knuffel(argument)]
    pub name: String,

    #[knuffel(child, unwrap(argument))]
    pub help_text: Option<String>,

    #[knuffel(child, unwrap(argument))]
    pub initiative_rating: Option<u32>,

    #[knuffel(children(name = "unit"), default)]
    pub units: Vec<Unit>,
}

#[derive(Debug, knuffel::Decode)]
pub struct Unit {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(child, unwrap(argument))]
    pub unit_type: String,

    #[knuffel(child, unwrap(argument))]
    pub speed: Option<u32>,
    #[knuffel(child, unwrap(argument))]
    pub armour: Option<u32>,
    #[knuffel(child, unwrap(argument))]
    pub cc: Option<u32>,
    #[knuffel(child, unwrap(argument))]
    pub ff: Option<u32>,

    #[knuffel(children(name = "weapon"), default)]
    pub weapons: Vec<UnitWeapon>,

    #[knuffel(child, unwrap(arguments))]
    pub notes: Option<Vec<String>>,
}

#[derive(Debug, knuffel::Decode)]
pub struct UnitWeapon {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(child, unwrap(argument))]
    pub range: String,

    #[knuffel(property)]
    pub x: Option<u32>,

    #[knuffel(child, unwrap(arguments))]
    pub firepower: Vec<String>,
}
