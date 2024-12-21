use crate::Unit;

pub mod consumable;
pub mod equippable;

pub trait Item<'a> {
    fn use_item(&self, target: &mut Unit) where 'a: 'static;
}