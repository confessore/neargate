use std::collections::HashMap;

use crate::{Effect, SpellType, EFFECTS};
use lazy_static::lazy_static;

pub mod spell_type;

pub struct Spell<'a> {
    pub name: &'a str,
    pub value: i32,
    pub spell_type: SpellType,
    pub effects: Vec<Effect<'a>>,
}

impl<'a> Spell<'_> {
    pub const fn new(name: &'a str) -> Spell<'a> {
        Spell {
            name,
            value: 0,
            spell_type: SpellType::None,
            effects: vec![],
        }
    }
}

impl Clone for Spell<'_> {
    fn clone(&self) -> Self {
        Spell {
            name: self.name,
            value: self.value,
            spell_type: self.spell_type,
            effects: self.effects.clone(),
        }
    }
}

lazy_static! {
    pub static ref SPELLS: HashMap<&'static str, Spell<'static>> = {
        let map = HashMap::from([
            (
                "Frostfire Bolt",
                Spell {
                    name: "Frostfire Bolt",
                    value: 20,
                    spell_type: SpellType::Debuff,
                    effects: vec![EFFECTS["Ignite"].clone(), EFFECTS["Frost"].clone()],
                },
            ),
            (
                "Frostbolt",
                Spell {
                    name: "Frostbolt",
                    value: 15,
                    spell_type: SpellType::Debuff,
                    effects: vec![EFFECTS["Frost"].clone()],
                },
            ),
            (
                "Lightning",
                Spell {
                    name: "Lightning",
                    value: 25,
                    spell_type: SpellType::Debuff,
                    effects: vec![],
                },
            ),
            (
                "Heal",
                Spell {
                    name: "Heal",
                    value: 35,
                    spell_type: SpellType::Buff,
                    effects: vec![],
                },
            ),
            (
                "Bless",
                Spell {
                    name: "Bless",
                    value: 0,
                    spell_type: SpellType::Buff,
                    effects: vec![],
                },
            ),
            (
                "Curse",
                Spell {
                    name: "Curse",
                    value: 0,
                    spell_type: SpellType::Debuff,
                    effects: vec![],
                },
            ),
        ]);
        map
    };
}
