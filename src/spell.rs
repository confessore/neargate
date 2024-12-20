use crate::Unit;

pub struct Spell<'a> {
    pub name: &'a str,
}

impl Clone for Spell<'_> {
    fn clone(&self) -> Self {
        Spell {
            name: self.name,
        }
    }
}

impl<'a> Spell<'_> {
    pub fn new(name: &'a str) -> Spell<'a> {
        Spell {
            name,
        }
    }
}

impl<'a> Copy for Spell<'_> {
}