use neargate::{item::equippable::Equippable, Spell, Unit};

fn main() {
    let mut warrior = Unit::new("Warrior");
    warrior.accuracy = 10.0;
    warrior.evasion = 10.0;
    let mut mage = Unit::new("Mage");
    let sword = Equippable::new("Sword");
    sword.equip(&mut mage);
    let fireball = Spell::new("Fireball");
    let immunity = Spell::new("Immunity");
    mage.learn(&fireball);
    mage.learn(&fireball);
    mage.learn(&immunity);
    for spell in mage.spellbook.iter() {
        println!("Mage has learned: {}", spell.name);
    }

    warrior.attack(&mut mage);
    mage.attack(&mut warrior);
    println!("Warrior: {} HP", warrior.current_health);
    println!("Mage: {} HP", mage.current_health);
    

}