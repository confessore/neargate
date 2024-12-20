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