use std::collections::HashMap;

use rand::Rng;

use crate::{aura::Aura, item::equippable::Equippable, job::job_type::JobType, Job, Spell, SpellEffect, SpellType};

pub struct Unit<'a> {
    pub name: &'a str,

    pub current_health: f32,
    pub max_health: f32,

    pub current_magic: f32,
    pub max_magic: f32,

    pub physical_armor: f32,
    pub magical_armor: f32,

    pub constitution: f32,
    pub base_constitution: f32,

    pub strength: f32,
    pub base_strength: f32,

    pub agility: f32,
    pub base_agility: f32,

    pub intelligence: f32,
    pub base_intelligence: f32,
    
    pub initiative: f32,
    pub movement: f32,
    pub jump: f32,
    pub accuracy: f32,
    pub evasion: f32,
    pub critical: f32,
    pub critical_resist: f32,

    pub experience: u32,
    pub max_experience: u32,
    pub level: u8,
    pub max_level: u8,

    pub burrowed: bool,
    pub cloaked: bool,
    pub flying: bool,
    pub harvestable: bool,
    pub immune: bool,
    pub selectable: bool,
    pub targetable: bool,

    pub job: JobType,
    pub jobs: HashMap<JobType, Job>,
    pub spellbook: Vec<Spell<'a>>,
    pub equipment: Vec<Equippable<'a>>,
    pub spell_effects: Vec<SpellEffect<'a>>,
    pub auras: Vec<Aura<'a>>,

    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl<'a> Unit<'_> {
    pub fn new(name: &'a str) -> Unit<'a> {
        Unit {
            name,
            current_health: 0.0,
            max_health: 0.0,
            current_magic: 0.0,
            max_magic: 0.0,
            physical_armor: 0.0,
            magical_armor: 0.0,
            constitution: 0.0,
            base_constitution: 10.0,
            strength: 0.0,
            base_strength: 10.0,
            agility: 0.0,
            base_agility: 10.0,
            intelligence: 0.0,
            base_intelligence: 10.0,
            initiative: 0.0,
            movement: 0.0,
            jump: 0.0,
            accuracy: 0.0,
            evasion: 0.0,
            critical: 0.0,
            critical_resist: 0.0,
            experience: 0,
            max_experience: 100,
            level: 1,
            max_level: 99,
            burrowed: false,
            cloaked: false,
            flying: false,
            harvestable: false,
            immune: false,
            selectable: true,
            targetable: true,
            job: JobType::None,
            jobs: HashMap::from([
                (JobType::None, Job::new(JobType::None)),
            ]),
            spellbook: vec![],
            equipment: vec![],
            spell_effects: vec![],
            auras: vec![],
            x: 0,
            y: 0,
            z: 0
        }
    }

    pub fn calculate_stats(&mut self) {
        self.constitution = self.base_constitution;
        self.constitution += self.auras.iter()
            .map(|aura| self.constitution * aura.constitution)
            .sum::<f32>();
        self.max_health = self.constitution * 10.0;
        if self.current_health > self.max_health {
            self.current_health = self.max_health;
        }

        self.strength = self.base_strength;
        self.strength += self.auras.iter()
            .map(|aura| self.strength * aura.strength)
            .sum::<f32>();

        self.max_health = self.constitution * 10.0;
        self.max_magic = self.intelligence * 10.0;
        self.physical_armor = self.constitution * 0.1;
        self.magical_armor = self.intelligence * 0.1;
        self.initiative = self.agility * 0.1;
        self.movement = self.agility * 0.1;
        self.jump = self.agility * 0.1;
        self.accuracy = self.agility * 0.1;
        self.evasion = self.agility * 0.1;
        self.critical = self.agility * 0.1;
        self.critical_resist = self.agility * 0.1;
    }

    pub fn attack(&mut self, target: &mut Unit<'a>) {
        let mut hit_chance = 95.0;
        hit_chance -= target.evasion;
        hit_chance += self.accuracy;
        let mut rng = rand::thread_rng();
        let hit_roll = rng.gen_range(0.0..=100.0);
        if hit_roll > hit_chance {
            println!("{} missed {}", self.name, target.name);
        } else {
            let mut critical_chance = 95.0;
            critical_chance -= target.critical_resist;
            critical_chance += self.critical;
            let critical_roll = rng.gen_range(0.0..=100.0);
            if critical_roll > critical_chance {
                target.current_health -= self.strength * 1.5;
                self.jobs.get_mut(&self.job).unwrap().points += 9;
                println!("{} critically hit {} for {} damage", self.name, target.name, self.strength * 1.5);
            } else {
                target.current_health -= self.strength;
                self.jobs.get_mut(&self.job).unwrap().points += 6;
                println!("{} hit {} for {} damage", self.name, target.name, self.strength);
            }
        }
    }

    pub fn learn(&mut self, spell: &Spell<'a>)
        where 'a: 'static {
            if self.spellbook.iter().any(|x| x.name == spell.name) {
                println!("You have already learned the spell: {}", spell.name);
            }
            else {
                self.spellbook.push(spell.clone());
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

    pub fn set_job(&mut self, job_type: JobType) {
        self.job = job_type;
        if !self.jobs.contains_key(&job_type) {
            self.jobs.insert(job_type, Job::new(job_type));
        }
        println!("{} set their job to: {:?}", self.name, self.job);
    }

    pub fn is_alive(&self) -> bool {
        self.current_health > 0.0
    }

    pub fn cast(&mut self, target: &mut Unit<'a>, spell: &Spell<'a>)
        where 'a: 'static {
        match spell.spell_type {
            SpellType::Buff => {
                target.current_health += spell.value as f32;
            },
            SpellType::Debuff => {
                target.current_health -= spell.value as f32;
                println!("{} cast {} on {} for {} damage", self.name, spell.name, target.name, spell.value);
                if spell.effects.len() > 0 {
                    for effect in spell.effects.iter() {
                        if !target.spell_effects.iter().any(|x| x.name == effect.name) {
                            target.spell_effects.push(effect.clone());
                            println!("{} is now affected by the {} effect", target.name, effect.name);
                        } else {
                            if let Some(spell_effect) = target.spell_effects.iter_mut().find(|x| x.name == effect.name) {
                                spell_effect.turns = effect.turns;
                                println!("The duration of the {} effect has been reset on {}", effect.name, target.name);
                            }
                        }
                    }
                }
            },
            SpellType::None => {

            }
        }
    }

    pub fn process_effects(&mut self) {
        let mut effects_to_remove = Vec::new();
        for effect in self.spell_effects.iter_mut() {
            if effect.turns > 0 {
                effect.turns -= 1;
                match effect.spell_type {
                    SpellType::Buff => {
                        self.current_health += effect.value as f32;
                        println!("{} healed {} from the {} effect", self.name, effect.value, effect.name);
                    },
                    SpellType::Debuff => {
                        self.current_health -= effect.value as f32;
                        println!("{} took {} damage from the {} effect", self.name, effect.value, effect.name);
                    },
                    SpellType::None => {

                    }
                }
            } else {
                effects_to_remove.push(effect.name);
            }
        }
        self.spell_effects.retain(|spell_effect| !effects_to_remove.contains(&spell_effect.name));
    }
}