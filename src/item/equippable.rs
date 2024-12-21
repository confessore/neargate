use super::Item;
use crate::{EquippableSlot, Unit};
pub mod equippable_slot;

pub struct Equippable<'a> {
    pub name: &'a str,
    pub equippable_slot: EquippableSlot,
}

impl<'a> Equippable<'_> {
    pub fn new(name: &'a str) -> Equippable<'a> {
        Equippable {
            name,
            equippable_slot: EquippableSlot::Head,
        }
    }

    pub fn equip(&self, target: &mut Unit) {
        println!("Equipping {} with {}", target.name, self.name);
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
        }
    }
}
