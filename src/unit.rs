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
        let constitution = 10.0;
        Unit {
            name,
            current_health: constitution * 10.0,
            max_health: constitution * 10.0,
            current_magic: 100.0,
            max_magic: 100.0,
            physical_armor: 0.0,
            magical_armor: 0.0,
            constitution: 10.0,
            strength: 10.0,
            agility: 10.0,
            intelligence: 0.0,
            initiative: 0.0,
            movement: 0.0,
            jump: 0.0,
            accuracy: 0.0,
            evasion: 0.0,
            critical: 10.0,
            critical_resist: 10.0,
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
        let mut rng = rand::thread_rng();
        let hit_chance = rng.gen_range(0.0..100.0);
        if hit_chance < self.accuracy {
            let damage = rng.gen_range(0.0..self.strength);
            target.current_health -= damage;
        }
        self.jobs.get_mut(&self.job).unwrap().points += 6;
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