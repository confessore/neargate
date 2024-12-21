pub mod damage;
pub mod item;
pub mod job;
pub mod spell;
pub mod unit;

pub use crate::{
    damage::{MagicalDamage, PhysicalDamage},
    item::{Item, consumable::Consumable, equippable::Equippable},
    job::{Job, job_type::JobType},
    spell::{Spell, SPELLS, spell_effect::{SpellEffect, SPELL_EFFECTS}, spell_type::SpellType},
    unit::Unit
};