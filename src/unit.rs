use std::collections::HashMap;

use rand::Rng;

use crate::{item::equippable::Equippable, job::job_type::JobType, Job, Spell};

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
            initiative: 1.0,
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
            x: 0,
            y: 0,
            z: 0
        }
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

    pub fn set_job(&mut self, job_type: JobType) {
        self.job = job_type;
        if !self.jobs.contains_key(&job_type) {
            self.jobs.insert(job_type, Job::new(job_type));
        }
        println!("{} set their job to: {:?}", self.name, self.job);
    }
}