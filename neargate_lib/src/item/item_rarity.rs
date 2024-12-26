use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Artifact,
    Heirloom,
}

impl Copy for ItemRarity {}

impl Clone for ItemRarity {
    fn clone(&self) -> Self {
        *self
    }
}

impl Display for ItemRarity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
