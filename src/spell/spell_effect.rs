use std::collections::HashMap;

use crate::{Aura, SpellType, AURAS};
use lazy_static::lazy_static;

pub struct SpellEffect<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub spell_type: SpellType,
    pub value: i32,
    pub turns: u8,
    pub auras: Vec<Aura<'a>>,
}

impl Clone for SpellEffect<'_> {
    fn clone(&self) -> Self {
        SpellEffect {
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
    pub static ref SPELL_EFFECTS: HashMap<&'static str, SpellEffect<'static>> = {
        let map = HashMap::from([
            ("Ignite", SpellEffect {
                name: "Ignite",
                description: "Burns the target with flames dealing 5 damage per turn",
                value: 5,
                spell_type: SpellType::Debuff,
                turns: 3,
                auras: vec![],
            },),
            ("Cripple", SpellEffect {
                name: "Frost",
                description: "Chills the target with ice slowing their movement by 20%",
                value: 0,
                spell_type: SpellType::Debuff,
                turns: 3,
                auras: vec![
                    AURAS["Cripple"]
                ],
            }),
        ]);
        map
    };
}