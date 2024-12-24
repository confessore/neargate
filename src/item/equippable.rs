use super::Item;
use crate::{EquippableSlot, ItemQuality, ItemRarity, Unit};
pub mod equippable_slot;

pub struct Equippable<'a> {
    pub name: &'a str,
    pub equippable_slot: EquippableSlot,
    pub item_quality: ItemQuality,
    pub item_rarity: ItemRarity,
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
    pub auras: Vec<&'a str>,
}

impl<'a> Equippable<'_> {
    pub fn new(name: &'a str) -> Equippable<'a> {
        Equippable {
            name,
            equippable_slot: EquippableSlot::Head,
            item_quality: ItemQuality::Standard,
            item_rarity: ItemRarity::Common,
            constitution: 0.0,
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
            auras: Vec::new(),
        }
    }
}

impl<'a> Item<'a> for Equippable<'a> {
    fn use_item(&self, target: &mut Unit)
    where
        'a: 'static,
    {
        target.equip(self);
    }
}

impl Clone for Equippable<'_> {
    fn clone(&self) -> Self {
        Equippable {
            name: self.name,
            equippable_slot: self.equippable_slot,
            item_quality: self.item_quality,
            item_rarity: self.item_rarity,
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
            auras: self.auras.clone(),
        }
    }
}
