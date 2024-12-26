use std::collections::HashMap;

use super::Item;
use crate::Unit;
use lazy_static::lazy_static;

pub struct Consumable<'a> {
    pub name: &'a str,
}

impl Consumable<'_> {
    pub fn new(name: &str) -> Consumable {
        Consumable { name }
    }
}

impl<'a> Item<'a> for Consumable<'a> {
    fn use_item(&self, target: &mut Unit)
    where
        'a: 'static,
    {
        target.consume(self);
    }
}

impl<'a> Copy for Consumable<'_> {}

impl Clone for Consumable<'_> {
    fn clone(&self) -> Self {
        Consumable { name: self.name }
    }
}

lazy_static! {
    pub static ref CONSUMABLES: HashMap<&'static str, Consumable<'static>> = {
        let map = HashMap::from([("Potion", Consumable { name: "Potion" })]);
        map
    };
}
