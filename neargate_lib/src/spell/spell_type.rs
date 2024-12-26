pub enum SpellType {
    None,
    Buff,
    Debuff,
}

impl Copy for SpellType {}

impl Clone for SpellType {
    fn clone(&self) -> Self {
        *self
    }
}
