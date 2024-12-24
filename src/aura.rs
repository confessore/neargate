use lazy_static::lazy_static;

use std::collections::HashMap;

pub struct Aura<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub constitution: f32,
    pub strength: f32,
    pub agility: f32,
    pub intelligence: f32,
    pub initiative: f32,
    pub movement: f32,
    pub jump: f32,
    pub accuracy: f32,
    pub evasion: f32,
    pub critical: f32,
    pub critical_resist: f32,
    pub experience_rate: f32,
    pub burrowed: bool,
    pub cloaked: bool,
    pub flying: bool,
    pub harvestable: bool,
    pub immune: bool,
    pub selectable: bool,
    pub targetable: bool,
}

impl Copy for Aura<'_> {}

impl Clone for Aura<'_> {
    fn clone(&self) -> Self {
        Aura {
            name: self.name,
            description: self.description,
            constitution: self.constitution,
            strength: self.strength,
            agility: self.agility,
            intelligence: self.intelligence,
            initiative: self.initiative,
            movement: self.movement,
            jump: self.jump,
            accuracy: self.accuracy,
            evasion: self.evasion,
            critical: self.critical,
            critical_resist: self.critical_resist,
            experience_rate: self.experience_rate,
            burrowed: self.burrowed,
            cloaked: self.cloaked,
            flying: self.flying,
            harvestable: self.harvestable,
            immune: self.immune,
            selectable: self.selectable,
            targetable: self.targetable,
        }
    }
}

lazy_static! {
    pub static ref AURAS: HashMap<&'static str, Aura<'static>> = {
        let map = HashMap::from([
            (
                "Savage Gladiator",
                Aura {
                    name: "Savage Gladiator",
                    description: "All stats are increased by 5%",
                    constitution: 0.05,
                    strength: 0.05,
                    agility: 0.05,
                    intelligence: 0.05,
                    initiative: 0.05,
                    movement: 0.05,
                    jump: 0.05,
                    accuracy: 0.05,
                    evasion: 0.05,
                    critical: 0.05,
                    critical_resist: 0.05,
                    experience_rate: 0.05,
                    burrowed: false,
                    cloaked: false,
                    flying: false,
                    harvestable: false,
                    immune: false,
                    selectable: false,
                    targetable: false,
                },
            ),
            (
                "Cripple",
                Aura {
                    name: "Cripple",
                    description: "Constitution is decreased by 20%",
                    constitution: -0.2,
                    strength: 0.0,
                    agility: 0.0,
                    intelligence: 0.0,
                    initiative: 0.0,
                    movement: 0.0,
                    jump: 0.0,
                    accuracy: 0.0,
                    evasion: 0.0,
                    critical: 0.0,
                    critical_resist: 0.0,
                    experience_rate: 0.0,
                    burrowed: false,
                    cloaked: false,
                    flying: false,
                    harvestable: false,
                    immune: false,
                    selectable: false,
                    targetable: false,
                },
            ),
            (
                "Burning",
                Aura {
                    name: "Burning",
                    description: "This unit is burning losing 20% critical and 20% critical resist",
                    constitution: 0.00,
                    strength: 0.00,
                    agility: 0.00,
                    intelligence: 0.00,
                    initiative: 0.0,
                    movement: 0.0,
                    jump: 0.0,
                    accuracy: 0.0,
                    evasion: 0.0,
                    critical: -0.2,
                    critical_resist: -0.2,
                    experience_rate: 0.0,
                    burrowed: false,
                    cloaked: false,
                    flying: false,
                    harvestable: false,
                    immune: false,
                    selectable: false,
                    targetable: false,
                },
            ),
            (
                "Chilled",
                Aura {
                    name: "Chilled",
                    description: "This unit is chilled moving 20% slower and jumping 20% lower",
                    constitution: 0.00,
                    strength: 0.00,
                    agility: 0.00,
                    intelligence: 0.00,
                    initiative: 0.0,
                    movement: -0.2,
                    jump: -0.2,
                    accuracy: 0.0,
                    evasion: 0.0,
                    critical: 0.0,
                    critical_resist: 0.0,
                    experience_rate: 0.0,
                    burrowed: false,
                    cloaked: false,
                    flying: false,
                    harvestable: false,
                    immune: false,
                    selectable: false,
                    targetable: false,
                },
            ),
        ]);
        map
    };
}
