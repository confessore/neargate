use lazy_static::lazy_static;

use std::collections::HashMap;

pub struct Aura<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub constitution: f32,
    pub strength: f32,
    pub agility: f32,
    pub intelligence: f32,
}

impl Copy for Aura<'_> { }

impl Clone for Aura<'_> {
    fn clone(&self) -> Self {
        Aura {
            name: self.name,
            description: self.description,
            constitution: self.constitution,
            strength: self.strength,
            agility: self.agility,
            intelligence: self.intelligence,
        }
    }
}

lazy_static! {
    pub static ref AURAS: HashMap<&'static str, Aura<'static>> = {
        let map = HashMap::from([
            ("Savage Gladiator", Aura { 
                name: "Savage Gladiator",
                description: "All stats are increased by 5%",
                constitution: 0.05,
                strength: 0.05,
                agility: 0.05,
                intelligence: 0.05,
            }),
            ("Cripple", Aura { 
                name: "Cripple",
                description: "Constitution is decreased by 20%",
                constitution: -0.20,
                strength: 0.00,
                agility: 0.00,
                intelligence: 0.00,
            }),
            ("Burning", Aura {
                name: "Burning",
                description: "This unit is burning",
                constitution: 0.00,
                strength: 0.00,
                agility: 0.00,
                intelligence: 0.00,
            })
        ]);
        map
    };
}