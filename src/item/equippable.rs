use super::Item;
use crate::{EquippableSlot, ItemQuality, ItemRarity, Unit};
pub mod equippable_slot;

pub struct Equippable<'a> {
    pub name: &'a str,
    pub equippable_slot: EquippableSlot,
    pub item_quality: ItemQuality,
    pub item_rarity: ItemRarity,
}

impl<'a> Equippable<'_> {
    pub fn new(name: &'a str) -> Equippable<'a> {
        Equippable {
            name,
            equippable_slot: EquippableSlot::Head,
            item_quality: ItemQuality::Standard,
            item_rarity: ItemRarity::Common,
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

impl<'a> Copy for Equippable<'_> {}

impl Clone for Equippable<'_> {
    fn clone(&self) -> Self {
        Equippable {
            name: self.name,
            equippable_slot: self.equippable_slot,
            item_quality: self.item_quality,
            item_rarity: self.item_rarity,
        }
    }
}
