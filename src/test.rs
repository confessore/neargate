#[cfg(test)]
mod tests {
    use crate::{Unit, SPELLS};

    #[test]
    fn test_unit() {
        let unit = Unit::new("Unit");
        assert_eq!(unit.name, "Unit");
    }

    #[test]
    fn test_unit_learn() {
        let mut unit = Unit::new("Unit");

        // Learn a spell
        let frostfire_bolt = &SPELLS["Frostfire Bolt"];
        unit.learn(frostfire_bolt);
        assert_eq!(unit.spellbook, vec!["Frostfire Bolt"]);

        // Learn the same spell again
        unit.learn(frostfire_bolt);
        assert_eq!(unit.spellbook, vec!["Frostfire Bolt"]);
    }

    #[test]
    fn test_unit_cast() {
        let mut unit = Unit::new("Unit");
        unit.prepare();

        let mut target = Unit::new("Target");
        target.prepare();

        // Cast a spell Unit doesn't know
        unit.cast(&mut target, &SPELLS["Frostfire Bolt"]);
        assert!(target.current_health != target.max_health);
    }
}
