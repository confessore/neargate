use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ItemQuality {
    Standard,
    Superior,
    Magic,
    Rare,
    Set,
    Unique,
    Crafted,
}

impl Copy for ItemQuality {}

impl Clone for ItemQuality {
    fn clone(&self) -> Self {
        *self
    }
}

impl Display for ItemQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
