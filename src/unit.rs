use std::collections::{BTreeMap, HashMap};

use rand::Rng;

use crate::{
    item::equippable, Aura, Consumable, Effect, Equippable, EquippableSlot, Item, Job, JobType,
    Spell, SpellType, AURAS, EFFECTS,
};

#[derive(Default)]
pub struct Unit<'a> {
    pub name: &'a str,

    pub current_health: f32,
    pub max_health: f32,

    pub current_magic: f32,
    pub max_magic: f32,

    pub constitution: f32,
    pub base_constitution: f32,

    pub strength: f32,
    pub base_strength: f32,

    pub agility: f32,
    pub base_agility: f32,

    pub intelligence: f32,
    pub base_intelligence: f32,

    pub physical_armor: f32,
    pub magical_armor: f32,

    pub initiative: f32,
    pub movement: f32,
    pub jump: f32,
    pub accuracy: f32,
    pub evasion: f32,
    pub critical: f32,
    pub critical_resist: f32,
    pub experience_rate: f32,
    pub experience: f32,
    pub max_experience: f32,
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
    // Name of the consumable and quantity
    pub consumables: HashMap<&'a str, u32>,
    pub equippables: Vec<Equippable<'a>>,
    pub equipment: BTreeMap<EquippableSlot, Equippable<'a>>,
    pub spellbook: Vec<&'a str>,
    // Name of the effect and how many turns are remaining
    pub effects: BTreeMap<&'a str, u8>,
    pub auras: Vec<&'a str>,

    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl<'a> Unit<'_> {
    pub fn new(name: &'a str) -> Unit<'a> {
        Unit {
            name,
            current_health: 0.0,
            max_health: 0.0,
            current_magic: 0.0,
            max_magic: 0.0,
            constitution: 0.0,
            base_constitution: 10.0,
            strength: 0.0,
            base_strength: 10.0,
            agility: 0.0,
            base_agility: 10.0,
            intelligence: 0.0,
            base_intelligence: 10.0,
            physical_armor: 0.0,
            magical_armor: 0.0,
            initiative: 0.0,
            movement: 0.0,
            jump: 0.0,
            accuracy: 0.0,
            evasion: 0.0,
            critical: 0.0,
            critical_resist: 0.0,
            experience_rate: 0.0,
            experience: 0.0,
            max_experience: 100.0,
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
            jobs: HashMap::from([(JobType::None, Job::new(JobType::None))]),
            consumables: HashMap::new(),
            equippables: vec![],
            equipment: BTreeMap::new(),
            spellbook: vec![],
            effects: BTreeMap::new(),
            auras: vec![],
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn set_base_stats(&mut self) {
        self.constitution = self.base_constitution;
        self.strength = self.base_strength;
        self.agility = self.base_agility;
        self.intelligence = self.base_intelligence;
        self.experience_rate = 1.0;
    }

    fn apply_equipment_to_base_stats(&mut self) {
        for equippable in self.equipment.values() {
            self.constitution += equippable.constitution;
            self.strength += equippable.strength;
            self.agility += equippable.agility;
            self.intelligence += equippable.intelligence;
        }
    }

    fn apply_equipment_to_stats(&mut self) {
        for equippable in self.equipment.values() {
            self.physical_armor += equippable.physical_armor;
            self.magical_armor += equippable.magical_armor;
            self.initiative += equippable.initiative;
            self.movement += equippable.movement;
            self.jump += equippable.jump;
            self.accuracy += equippable.accuracy;
            self.evasion += equippable.evasion;
            self.critical += equippable.critical;
            self.critical_resist += equippable.critical_resist;
            self.experience_rate += equippable.experience_rate;
        }
    }

    fn apply_equipment_auras_to_base_stats(&mut self) {
        for equippable in self.equipment.values() {
            for aura in &equippable.auras {
                self.constitution += self.constitution * &AURAS[aura].constitution;
                self.strength += self.strength * &AURAS[aura].strength;
                self.agility += self.agility * &AURAS[aura].agility;
                self.intelligence += self.intelligence * &AURAS[aura].intelligence;
            }
        }
    }

    fn apply_equipment_auras_to_stats(&mut self) {
        for equippable in self.equipment.values() {
            for aura in &equippable.auras {
                self.physical_armor += self.physical_armor * &AURAS[aura].physical_armor;
                self.magical_armor += self.magical_armor * &AURAS[aura].magical_armor;
                self.initiative += self.initiative * &AURAS[aura].initiative;
                self.movement += self.movement * &AURAS[aura].movement;
                self.jump += self.jump * &AURAS[aura].jump;
                self.accuracy += self.accuracy * &AURAS[aura].accuracy;
                self.evasion += self.evasion * &AURAS[aura].evasion;
                self.critical += self.critical * &AURAS[aura].critical;
                self.critical_resist += self.critical_resist * &AURAS[aura].critical_resist;
                self.experience_rate += self.experience_rate * &AURAS[aura].experience_rate;
            }
        }
    }

    fn derive_stats(&mut self) {
        self.physical_armor = self.strength * 0.1;
        self.magical_armor = self.intelligence * 0.1;
        self.initiative = self.agility * 0.1;
        self.movement = self.agility * 0.1;
        self.jump = self.agility * 0.1;
        self.accuracy = self.agility * 0.1;
        self.evasion = self.agility * 0.1;
        self.critical = self.agility * 0.1;
        self.critical_resist = self.agility * 0.1;
    }

    fn apply_base_auras_to_base_stats(&mut self) {
        for aura in &self.auras {
            self.constitution += self.constitution * &AURAS[aura].constitution;
            self.strength += self.strength * &AURAS[aura].strength;
            self.agility += self.agility * &AURAS[aura].agility;
            self.intelligence += self.intelligence * &AURAS[aura].intelligence;
        }
    }

    fn apply_base_auras_to_stats(&mut self) {
        for aura in &self.auras {
            self.physical_armor += self.physical_armor * &AURAS[aura].physical_armor;
            self.magical_armor += self.magical_armor * &AURAS[aura].magical_armor;
            self.initiative += self.initiative * &AURAS[aura].initiative;
            self.movement += self.movement * &AURAS[aura].movement;
            self.jump += self.jump * &AURAS[aura].jump;
            self.accuracy += self.accuracy * &AURAS[aura].accuracy;
            self.evasion += self.evasion * &AURAS[aura].evasion;
            self.critical += self.critical * &AURAS[aura].critical;
            self.critical_resist += self.critical_resist * &AURAS[aura].critical_resist;
            self.experience_rate += self.experience_rate * &AURAS[aura].experience_rate;
        }
    }

    fn apply_effect_auras_to_base_stats(&mut self) {
        for effect in self.effects.iter() {
            let fetched_effect = &EFFECTS[effect.0];
            for aura in &fetched_effect.auras {
                self.constitution += self.constitution * &AURAS[aura].constitution;
                self.strength += self.strength * &AURAS[aura].strength;
                self.agility += self.agility * &AURAS[aura].agility;
                self.intelligence += self.intelligence * &AURAS[aura].intelligence;
            }
        }
    }

    fn apply_effect_auras_to_stats(&mut self) {
        for effect in self.effects.iter() {
            let fetched_effect = &EFFECTS[effect.0];
            for aura in &fetched_effect.auras {
                self.physical_armor += self.physical_armor * &AURAS[aura].physical_armor;
                self.magical_armor += self.magical_armor * &AURAS[aura].magical_armor;
                self.initiative += self.initiative * &AURAS[aura].initiative;
                self.movement += self.movement * &AURAS[aura].movement;
                self.jump += self.jump * &AURAS[aura].jump;
                self.accuracy += self.accuracy * &AURAS[aura].accuracy;
                self.evasion += self.evasion * &AURAS[aura].evasion;
                self.critical += self.critical * &AURAS[aura].critical;
                self.critical_resist += self.critical_resist * &AURAS[aura].critical_resist;
                self.experience_rate += self.experience_rate * &AURAS[aura].experience_rate;
            }
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
                println!(
                    "{} critically hit {} for {} damage",
                    self.name,
                    target.name,
                    self.strength * 1.5
                );
            } else {
                target.current_health -= self.strength;
                self.jobs.get_mut(&self.job).unwrap().points += 6;
                println!(
                    "{} hit {} for {} damage",
                    self.name, target.name, self.strength
                );
            }
        }
    }

    pub fn learn(&mut self, spell: &Spell<'a>)
    where
        'a: 'static,
    {
        if self
            .spellbook
            .iter()
            .any(|spell_name| spell_name == &spell.name)
        {
            println!("You have already learned the spell: {}", spell.name);
        } else {
            self.spellbook.push(spell.name);
            println!("You have learned the spell: {}", spell.name);
        }
    }

    pub fn equip(&mut self, equippable: &Equippable<'a>)
    where
        'a: 'static,
    {
        if self.equipment.contains_key(&equippable.equippable_slot) {
            println!("You have already equipped the item: {}", equippable.name);
        } else {
            self.equipment
                .insert(equippable.equippable_slot, equippable.clone());
            println!("You have equipped the item: {}", equippable.name);
        }
    }

    pub fn consume(&mut self, consumable: &Consumable<'a>)
    where
        'a: 'static,
    {
        println!("{} consumed the item: {}", self.name, consumable.name);
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
    where
        'a: 'static,
    {
        match spell.spell_type {
            SpellType::Buff => {
                // heal the target
                target.current_health += spell.value as f32;
            }
            SpellType::Debuff => {
                // damage the target
                target.current_health -= spell.value as f32;
                println!(
                    "{} cast {} on {} for {} damage",
                    self.name, spell.name, target.name, spell.value
                );
                // apply effects to the target
                if spell.effects.len() > 0 {
                    for effect in spell.effects.iter() {
                        let effect = &EFFECTS[effect.0];
                        target.effects.insert(effect.name, effect.turns);
                        if target.effects.contains_key(effect.name) {
                            println!(
                                "Duration of {} effect on {} reset",
                                effect.name, target.name
                            );
                        } else {
                            println!(
                                "{} is now affected by the {} effect",
                                target.name, effect.name
                            );
                        }
                    }
                }
            }
            SpellType::None => {}
        }
    }

    pub fn process_effects(&mut self) {
        let mut effects_to_remove = Vec::new();
        for effect in self.effects.iter_mut() {
            let fetched_effect = &EFFECTS[effect.0];
            if *effect.1 > 0 {
                *effect.1 -= 1;
                match fetched_effect.spell_type {
                    SpellType::Buff => {
                        self.current_health += fetched_effect.value as f32;
                        println!(
                            "{} healed {} from the {} effect",
                            self.name, fetched_effect.value, fetched_effect.name
                        );
                    }
                    SpellType::Debuff => {
                        self.current_health -= fetched_effect.value as f32;
                        if fetched_effect.value > 0 {
                            println!(
                                "{} took {} damage from the {} effect",
                                self.name, fetched_effect.value, fetched_effect.name
                            );
                        }
                    }
                    SpellType::None => {}
                }
            } else {
                effects_to_remove.push(fetched_effect.name);
            }
        }
        self.effects
            .retain(|effect_name, turns| !effects_to_remove.contains(&effect_name));
    }

    fn derive_max_health(&mut self) {
        self.max_health = self.constitution * 10.0;
        if self.current_health > self.max_health {
            self.current_health = self.max_health;
        }
    }

    fn derive_max_magic(&mut self) {
        self.max_magic = self.intelligence * 10.0;
        if self.current_magic > self.max_magic {
            self.current_magic = self.max_magic;
        }
    }

    pub fn prepare(&mut self) {
        self.calculate_stats();
        self.current_health = self.max_health;
        self.current_magic = self.max_magic;
    }

    pub fn calculate_stats(&mut self) {
        self.set_base_stats();
        self.apply_equipment_to_base_stats();
        self.apply_base_auras_to_base_stats();
        self.apply_equipment_auras_to_base_stats();
        self.apply_effect_auras_to_base_stats();
        self.derive_stats();
        self.apply_equipment_to_stats();
        self.apply_base_auras_to_stats();
        self.apply_equipment_auras_to_stats();
        self.apply_effect_auras_to_stats();
        self.derive_max_health();
        self.derive_max_magic();
    }
}
