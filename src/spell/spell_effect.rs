use crate::SpellType;


pub struct SpellEffect<'a> {
    pub name: &'a str,
    pub value: i32,
    pub spell_type: SpellType,
    pub turns: u8,
}

impl Copy for SpellEffect<'_> { }

impl Clone for SpellEffect<'_> {
    fn clone(&self) -> Self {
        SpellEffect {
            name: self.name,
            value: self.value,
            spell_type: self.spell_type,
            turns: self.turns,
        }
    }
}

pub const SPELL_EFFECTS: [SpellEffect; 1] = [    
    SpellEffect { name: "Ignite", value: 5, spell_type: SpellType::Debuff, turns: 3 },
];