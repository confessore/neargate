use crate::Unit;

pub struct Spell<'a> {
    pub name: &'a str,
}

impl<'a> Spell<'_> {
    pub fn new(name: &'a str) -> Spell<'a> {
        Spell {
            name,
        }
    }
}

impl<'a> Copy for Spell<'_> {}

impl Clone for Spell<'_> {
    fn clone(&self) -> Self {
        Spell {
            name: self.name,
        }
    }
}