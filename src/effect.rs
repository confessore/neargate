use std::collections::HashMap;

use crate::{Aura, SpellType, AURAS};
use lazy_static::lazy_static;

pub struct Effect<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub spell_type: SpellType,
    pub value: i32,
    pub turns: u8,
    pub auras: Vec<Aura<'a>>,
}

impl Clone for Effect<'_> {
    fn clone(&self) -> Self {
        Effect {
            name: self.name,
            description: self.description,
            value: self.value,
            spell_type: self.spell_type,
            turns: self.turns,
            auras: self.auras.clone(),
        }
    }
}

lazy_static! {
    pub static ref EFFECTS: HashMap<&'static str, Effect<'static>> = {
        let map = HashMap::from([
            (
                "Ignite",
                Effect {
                    name: "Ignite",
                    description: "Burns the target with flames dealing 5 damage per turn",
                    value: 5,
                    spell_type: SpellType::Debuff,
                    turns: 3,
                    auras: vec![],
                },
            ),
            (
                "Frost",
                Effect {
                    name: "Frost",
                    description: "Chills the target with ice slowing their movement by 20%",
                    value: 0,
                    spell_type: SpellType::Debuff,
                    turns: 3,
                    auras: vec![AURAS["Slowed"]],
                },
            ),
        ]);
        map
    };
}
