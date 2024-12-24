use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum EquippableSlot {
    Head,
    Neck,
    Shoulders,
    Chest,
    Waist,
    Legs,
    Feet,
    Hands,
    Finger,
    Trinket,
    MainHand,
    OffHand,
    Ranged,
}

impl Copy for EquippableSlot {}

impl Clone for EquippableSlot {
    fn clone(&self) -> Self {
        *self
    }
}

impl Display for EquippableSlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
