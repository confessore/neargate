use crate::Unit;

pub struct Equippable<'a> {
    pub name: &'a str,
}

impl<'a> Equippable<'_> {
    pub fn new(name: &'a str) -> Equippable<'a> {
        Equippable {
            name,
        }
    }

    pub fn equip(&self, target: &mut Unit) {
        println!("Equipping {} with {}", target.name, self.name);
    }
}

impl<'a> Copy for Equippable<'_> { }

impl Clone for Equippable<'_> {
    fn clone(&self) -> Self {
        Equippable {
            name: self.name,
        }
    }
}