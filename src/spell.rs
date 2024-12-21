use crate::SpellType;

pub mod spell_type;

pub struct Spell<'a> {
    pub name: &'a str,
    pub value: i32,
    pub spell_type: SpellType,
}

impl<'a> Spell<'_> {
    pub const fn new(name: &'a str) -> Spell<'a> {
        Spell {
            name,
            value: 0,
            spell_type: SpellType::None,
        }
    }
}

impl<'a> Copy for Spell<'_> {}

impl Clone for Spell<'_> {
    fn clone(&self) -> Self {
        Spell {
            name: self.name,
            value: self.value,
            spell_type: self.spell_type,
        }
    }
}

pub const SPELLS: [Spell; 6] = [    
    Spell { name: "Fireball", value: 20, spell_type: SpellType::Debuff },
    Spell { name: "Frostbolt", value: 15, spell_type: SpellType::Debuff },
    Spell { name: "Lightning", value: 25, spell_type: SpellType::Debuff },
    Spell { name: "Heal", value: 35, spell_type: SpellType::Buff },
    Spell { name: "Bless", value: 0, spell_type: SpellType::Buff },
    Spell { name: "Curse", value: 0, spell_type: SpellType::Debuff },
];