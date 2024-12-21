pub mod consumable;
pub mod equippable;

pub struct Item<'a> {
    pub name: &'a str,
}
