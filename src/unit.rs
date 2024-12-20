use std::collections::HashMap;

use rand::Rng;

use crate::{item::equippable::Equippable, Job, Spell};

pub struct Unit<'a> {
    pub name: &'a str,
    pub current_health: f32,
    pub max_health: f32,
    pub current_magic: f32,
    pub max_magic: f32,
    pub physical_armor: f32,
    pub magical_armor: f32,

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

    pub experience: f32,
    pub max_experience: f32,
    pub level: i32,
    pub max_level: i32,

    pub burrowed: bool,
    pub cloaked: bool,
    pub flying: bool,
    pub harvestable: bool,
    pub immune: bool,
    pub selectable: bool,
    pub targetable: bool,

    pub job: Job,
    pub jobs: HashMap<Job, (i32, i32)>,
    pub spellbook: Vec<Spell<'a>>,
    pub equipment: Vec<Equippable<'a>>,

    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl<'a> Unit<'_> {
    pub fn new(name: &'a str) -> Unit<'a> {
        Unit {
            name,
            current_health: 100.0,
            max_health: 100.0,
            current_magic: 100.0,
            max_magic: 100.0,
            physical_armor: 0.0,
            magical_armor: 0.0,
            constitution: 10.0,
            strength: 10.0,
            agility: 10.0,
            intelligence: 10.0,
            initiative: 10.0,
            movement: 10.0,
            jump: 10.0,
            accuracy: 10.0,
            evasion: 10.0,
            critical: 10.0,
            critical_resist: 10.0,
            experience: 0.0,
            max_experience: 100.0,
            level: 1,
            max_level: 100,
            burrowed: false,
            cloaked: false,
            flying: false,
            harvestable: false,
            immune: false,
            selectable: true,
            targetable: true,
            job: Job::Soldier,
            jobs: HashMap::new(),
            spellbook: vec![],
            equipment: vec![],
            x: 0,
            y: 0,
            z: 0
        }
    }

    pub fn attack(&mut self, target: &mut Unit<'a>) {
        let mut rng = rand::thread_rng();
        let hit_chance = rng.gen_range(0.0..100.0);
        if hit_chance < self.accuracy {
            let damage = rng.gen_range(0.0..self.strength);
            target.current_health -= damage;
        }
    }

    pub fn learn(&mut self, spell: &Spell<'a>)
        where 'a: 'static {
            if self.spellbook.iter().any(|spellbook_spell| spellbook_spell.name == spell.name) {
                println!("You have already learned the spell: {}", spell.name);
            }
            else {
                self.spellbook.push(*spell);
                println!("You have learned the spell: {}", spell.name);
            }
    }

    pub fn equip(&mut self, item: &Equippable<'a>)
        where 'a: 'static {
            if self.equipment.iter().any(|equipment_item| equipment_item.name == item.name) {
                println!("You have already equipped the item: {}", item.name);
            }
            else {
                self.equipment.push(*item);
                println!("You have equipped the item: {}", item.name);
            }
    }
}