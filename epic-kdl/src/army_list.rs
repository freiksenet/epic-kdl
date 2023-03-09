use std::{fmt, str::FromStr, vec};

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

impl FormationGroup {
    pub fn formations_have_upgrades(&self) -> bool {
        self.formations
            .iter()
            .any(|formation| !formation.upgrades.is_empty())
    }
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
    #[knuffel(children(name = "unit"), default)]
    pub units: Vec<FormationUnit>,
    #[knuffel(children(name = "upgrade"), default)]
    pub upgrades: Vec<FormationUpgrade>,
    #[knuffel(children(name = "tag"), default)]
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
                "any {} of the following unit{}: {}",
                count,
                if self.units.len() > 1 { "s" } else { "" },
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
    pub ac_type: Option<String>,
    #[knuffel(child, unwrap(argument))]
    pub armour: Option<u32>,
    #[knuffel(child, unwrap(argument))]
    pub cc: Option<u32>,
    #[knuffel(child, unwrap(argument))]
    pub ff: Option<u32>,

    #[knuffel(children(name = "weapon"), default)]
    pub weapons: Vec<UnitWeapon>,

    #[knuffel(children(name = "loadout"), default)]
    pub loadouts: Vec<UnitLoadout>,

    #[knuffel(children, default)]
    pub notes: Vec<UnitNote>,
}

impl Unit {
    pub fn speed_or_ac_type(&self) -> String {
        match (&self.ac_type, &self.speed) {
            (None, None) => "n/a".to_string(),
            (None, Some(speed)) => format!("{}cm", speed),
            (Some(ac_type), _) => ac_type.to_string(),
        }
    }

    pub fn armour(&self) -> String {
        match self.cc {
            Some(armour) => format!("{}+", armour),
            None => "n/a".to_string(),
        }
    }

    pub fn cc(&self) -> String {
        match self.cc {
            Some(cc) => format!("{}+", cc),
            None => "n/a".to_string(),
        }
    }

    pub fn ff(&self) -> String {
        match self.ff {
            Some(ff) => format!("{}+", ff),
            None => "n/a".to_string(),
        }
    }

    pub fn damage_capacity(&self) -> Option<&UnitDamageCapacity> {
        self.notes.iter().find_map(|note| match note {
            UnitNote::DamageCapacity(dc) => Some(dc),
            _ => None,
        })
    }

    pub fn transport(&self) -> Option<&UnitTransport> {
        self.notes.iter().find_map(|note| match note {
            UnitNote::Transport(t) => Some(t),
            _ => None,
        })
    }

    pub fn has_notes_or_special_rules(&self) -> bool {
        self.notes
            .iter()
            .any(|note| matches!(note, UnitNote::SpecialRule(_) | UnitNote::Note(_)))
    }

    pub fn notes_and_special_rules(&self) -> impl Iterator<Item = &String> {
        self.special_rules().chain(self.notes())
    }

    pub fn special_rules(&self) -> impl Iterator<Item = &String> {
        self.notes.iter().filter_map(|note| match note {
            UnitNote::SpecialRule(sr) => Some(sr),
            _ => None,
        })
    }

    pub fn notes(&self) -> impl Iterator<Item = &String> {
        self.notes.iter().filter_map(|note| match note {
            UnitNote::Note(note) => Some(note),
            _ => None,
        })
    }

    pub fn has_special_rules(&self) -> bool {
        self.special_rules().next().is_some()
    }
}

#[derive(Debug)]
pub enum UnitNote {
    Note(String),
    SpecialRule(String),
    Transport(UnitTransport),
    DamageCapacity(UnitDamageCapacity),
}

impl<S> ::knuffel::Decode<S> for UnitNote
where
    S: ::knuffel::traits::ErrorSpan,
{
    fn decode_node(
        node: &::knuffel::ast::SpannedNode<S>,
        ctx: &mut ::knuffel::decode::Context<S>,
    ) -> Result<Self, ::knuffel::errors::DecodeError<S>> {
        match &**node.node_name {
            "note" => {
                let mut iter_args = node.arguments.iter();
                let val = iter_args.next().ok_or_else(|| {
                    ::knuffel::errors::DecodeError::missing(node, "additional argument is required")
                })?;
                let field0 = ::knuffel::traits::DecodeScalar::decode(val, ctx)?;
                if let Err(e) = crate::decode_utils::validate_single_arg(ctx, node, iter_args) {
                    Err(e)
                } else {
                    Ok(UnitNote::Note(field0))
                }
            }
            "sr" => {
                let mut iter_args = node.arguments.iter();
                let val = iter_args.next().ok_or_else(|| {
                    ::knuffel::errors::DecodeError::missing(node, "additional argument is required")
                })?;
                let field0 = ::knuffel::traits::DecodeScalar::decode(val, ctx)?;
                if let Err(e) = crate::decode_utils::validate_single_arg(ctx, node, iter_args) {
                    Err(e)
                } else {
                    Ok(UnitNote::SpecialRule(field0))
                }
            }
            "transport" => ::knuffel::Decode::decode_node(node, ctx).map(UnitNote::Transport),
            "DC" => ::knuffel::Decode::decode_node(node, ctx).map(UnitNote::DamageCapacity),
            name_str => Err(::knuffel::errors::DecodeError::conversion(
                &node.node_name,
                "expected `note`, `sr`, `transport` or `DC`",
            )),
        }
    }
}

#[derive(Debug, knuffel::Decode)]
pub struct UnitDamageCapacity {
    #[knuffel(argument)]
    pub amount: String,

    #[knuffel(child, unwrap(argument))]
    pub critical_hit: String,
}

#[derive(Debug, knuffel::Decode)]
pub struct UnitTransport {
    #[knuffel(children(name = "capacity"), default)]
    pub capacitities: Vec<UnitTransportCapacity>,
    #[knuffel(child, unwrap(argument))]
    pub text: String,
}

#[derive(Debug, knuffel::Decode)]
pub struct UnitTransportCapacity {
    #[knuffel(argument)]
    pub amount: UnitTransportCapacityAmount,

    #[knuffel(children(name = "allow"), default)]
    pub allow: Vec<UnitSelectors>,
    #[knuffel(children(name = "disallow"), default)]
    pub disallow: Vec<UnitSelectors>,
    #[knuffel(children(name = "cost"), default)]
    pub cost: Vec<UnitTransportCost>,
}

impl UnitTransportCapacity {
    pub fn all_allowed(&self) -> impl Iterator<Item = &UnitSelector> {
        self.allow.iter().flat_map(|a| a.selectors.iter())
    }

    pub fn all_disallowed(&self) -> impl Iterator<Item = &UnitSelector> {
        self.disallow.iter().flat_map(|a| a.selectors.iter())
    }
}

#[derive(Debug)]
pub enum UnitTransportCapacityAmount {
    Fixed(u32),
    Formation,
    Text(String),
}

impl<S: knuffel::traits::ErrorSpan> knuffel::DecodeScalar<S> for UnitTransportCapacityAmount {
    fn raw_decode(
        val: &knuffel::ast::Value<S>,
        ctx: &mut knuffel::decode::Context<S>,
    ) -> Result<UnitTransportCapacityAmount, knuffel::errors::DecodeError<S>> {
        match &*val.literal {
            knuffel::ast::Literal::String(s) if s.clone().to_string() == "formation" => {
                Ok(UnitTransportCapacityAmount::Formation)
            }
            knuffel::ast::Literal::String(ref s) => {
                Ok(UnitTransportCapacityAmount::Text(s.clone().into()))
            }
            knuffel::ast::Literal::Int(ref value) => match value.try_into() {
                Ok(val) => Ok(UnitTransportCapacityAmount::Fixed(val)),
                Err(e) => {
                    ctx.emit_error(knuffel::errors::DecodeError::conversion(&val.literal, e));
                    Ok(UnitTransportCapacityAmount::Formation)
                }
            },
            _ => {
                ctx.emit_error(knuffel::errors::DecodeError::scalar_kind(
                    knuffel::decode::Kind::String,
                    &val.literal,
                ));
                Ok(UnitTransportCapacityAmount::Formation)
            }
        }
    }
    fn type_check(
        type_name: &Option<knuffel::span::Spanned<knuffel::ast::TypeName, S>>,
        ctx: &mut knuffel::decode::Context<S>,
    ) {
        if let Some(typ) = type_name {
            if typ.as_builtin() != Some(&knuffel::ast::BuiltinType::U32) {
                ctx.emit_error(knuffel::errors::DecodeError::TypeName {
                    span: typ.span().clone(),
                    found: Some(typ.value.clone()),
                    expected: knuffel::errors::ExpectedType::optional(
                        knuffel::ast::BuiltinType::U32,
                    ),
                    rust_type: "u32",
                });
            }
        }
    }
}

#[derive(Debug, knuffel::Decode)]
pub struct UnitSelectors {
    #[knuffel(arguments, default)]
    pub selectors: Vec<UnitSelector>,
}

#[derive(Debug)]
pub enum UnitSelector {
    UnitType(String),
    Unit(String),
    SpecialRule(String),
}

impl<S: knuffel::traits::ErrorSpan> knuffel::DecodeScalar<S> for UnitSelector {
    fn raw_decode(
        val: &knuffel::ast::Value<S>,
        ctx: &mut knuffel::decode::Context<S>,
    ) -> Result<UnitSelector, knuffel::errors::DecodeError<S>> {
        let str = match &*val.literal {
            knuffel::ast::Literal::String(ref s) => s.clone().into(),
            _ => {
                ctx.emit_error(knuffel::errors::DecodeError::scalar_kind(
                    knuffel::decode::Kind::String,
                    &val.literal,
                ));
                String::new()
            }
        };
        if let Some(typ) = &val.type_name {
            let tn = typ.as_str();
            match tn {
                "unit-type" => Ok(UnitSelector::UnitType(str)),
                "sr" => Ok(UnitSelector::SpecialRule(str)),
                "unit" => Ok(UnitSelector::Unit(str)),
                _ => Err(knuffel::errors::DecodeError::scalar_kind(
                    knuffel::decode::Kind::String,
                    &val.literal,
                )),
            }
        } else {
            Err(knuffel::errors::DecodeError::scalar_kind(
                knuffel::decode::Kind::String,
                &val.literal,
            ))
        }
    }

    fn type_check(
        type_name: &Option<knuffel::span::Spanned<knuffel::ast::TypeName, S>>,
        ctx: &mut knuffel::decode::Context<S>,
    ) {
        if let Some(typ) = type_name {
            let tn = typ.as_str();
            if tn != "unit-type" && tn != "unit" && tn != "sr" {
                ctx.emit_error(knuffel::errors::DecodeError::TypeName {
                    span: typ.span().clone(),
                    found: Some(typ.value.clone()),
                    expected: knuffel::errors::ExpectedType::required(
                        knuffel::ast::TypeName::from_string(
                            String::from("`unit-type` | `unit` | `sr`").into_boxed_str(),
                        ),
                    ),
                    rust_type: "String",
                });
            }
        }
    }
}

#[derive(Debug, knuffel::Decode)]
pub struct UnitTransportCost {
    #[knuffel(argument)]
    pub cost: f32,
    #[knuffel(arguments)]
    pub selector: Vec<UnitSelector>,
}

#[derive(Debug, knuffel::Decode)]
pub struct UnitLoadout {
    #[knuffel(child, unwrap(argument))]
    pub cc: Option<u32>,
    #[knuffel(child, unwrap(argument))]
    pub ff: Option<u32>,

    #[knuffel(children(name = "weapon"), default)]
    pub weapons: Vec<UnitWeapon>,
}

#[derive(Debug, knuffel::Decode)]
pub struct UnitWeapon {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(child, unwrap(argument))]
    pub range: Option<String>,

    #[knuffel(property)]
    pub x: Option<u32>,

    #[knuffel(property)]
    pub arc: Option<String>,

    #[knuffel(child, unwrap(arguments))]
    pub firepower: Vec<String>,

    #[knuffel(children, default)]
    pub special_rules: Vec<UnitWeaponSpecialRule>,
}

#[derive(Debug)]
pub enum UnitWeaponSpecialRule {
    Note(String),
    SpecialRule(String),
    EA(String),
}

impl<S> ::knuffel::Decode<S> for UnitWeaponSpecialRule
where
    S: ::knuffel::traits::ErrorSpan,
{
    fn decode_node(
        node: &::knuffel::ast::SpannedNode<S>,
        ctx: &mut ::knuffel::decode::Context<S>,
    ) -> Result<Self, ::knuffel::errors::DecodeError<S>> {
        match &**node.node_name {
            "note" => {
                let mut iter_args = node.arguments.iter();
                let val = iter_args.next().ok_or_else(|| {
                    ::knuffel::errors::DecodeError::missing(node, "additional argument is required")
                })?;
                let field0 = ::knuffel::traits::DecodeScalar::decode(val, ctx)?;
                crate::decode_utils::validate_single_arg(ctx, node, iter_args);

                Ok(UnitWeaponSpecialRule::Note(field0))
            }
            "sr" => {
                let mut iter_args = node.arguments.iter();
                let val = iter_args.next().ok_or_else(|| {
                    ::knuffel::errors::DecodeError::missing(node, "additional argument is required")
                })?;
                let field0 = ::knuffel::traits::DecodeScalar::decode(val, ctx)?;
                crate::decode_utils::validate_single_arg(ctx, node, iter_args);
                Ok(UnitWeaponSpecialRule::SpecialRule(field0))
            }
            "EA" => {
                let mut iter_args = node.arguments.iter();
                let val = iter_args.next().ok_or_else(|| {
                    ::knuffel::errors::DecodeError::missing(node, "additional argument is required")
                })?;
                let field0 = ::knuffel::traits::DecodeScalar::decode(val, ctx)?;
                crate::decode_utils::validate_single_arg(ctx, node, iter_args);
                Ok(UnitWeaponSpecialRule::EA(field0))
            }
            name_str => Err(::knuffel::errors::DecodeError::conversion(
                &node.node_name,
                "expected one of `note`, `sr`, `EA`",
            )),
        }
    }
}

impl fmt::Display for UnitWeaponSpecialRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitWeaponSpecialRule::Note(text) => write!(f, "{}", text),
            UnitWeaponSpecialRule::SpecialRule(text) => write!(f, "{}", text),
            UnitWeaponSpecialRule::EA(text) => write!(f, "EA{}", text),
        }
    }
}

impl UnitWeapon {
    pub fn range(&self) -> String {
        match &self.range {
            Some(range) => range.to_string(),
            None => "n/a".to_string(),
        }
    }
}
