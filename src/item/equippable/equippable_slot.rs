#[derive(Debug, Eq, PartialEq, Hash)]
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
