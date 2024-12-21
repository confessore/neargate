use neargate::{Equippable, JobType, Spell, Unit, AURAS, SPELLS};

fn main() {
    let mut warrior = Unit::new("Warrior");
    //warrior.accuracy += 50.0;
    //warrior.evasion += 50.0;
    let mut mage = Unit::new("Mage");
    let weapon = Equippable::new("Weapon");
    mage.equip(&weapon);
    let armor = Equippable::new("Armor");
    mage.equip(&armor);
    mage.equip(&armor);
    let fireball = Spell::new("Fireball");
    let immunity = Spell::new("Immunity");
    mage.learn(&fireball);
    mage.learn(&fireball);
    mage.learn(&immunity);
    for spell in mage.spellbook.iter() {
        println!("Mage has learned: {}", spell.name);
    }
    mage.set_job(JobType::Theurgist);
    for job in mage.jobs.iter() {
        println!("Mage has the job: {:?}", job.1.job_type);
    }
    let savage_gladiator = AURAS["Savage Gladiator"];
    mage.auras.push(savage_gladiator);
    let cripple = AURAS["Cripple"];
    mage.auras.push(cripple);
    println!("Mage has the aura: {} | {}", mage.auras[0].name, mage.auras[0].description);
    println!("Mage has the aura: {} | {}", mage.auras[1].name, mage.auras[1].description);
    warrior.calculate_stats();
    warrior.current_health = warrior.max_health;
    mage.calculate_stats();
    mage.current_health = mage.max_health;

    while warrior.is_alive() && mage.is_alive() {
        warrior.attack(&mut mage);
        if warrior.is_alive() && mage.is_alive() {
            warrior.process_effects();
        }
        
        if warrior.is_alive() && mage.is_alive() {
            mage.cast(&mut warrior, &SPELLS["Fireball"]);
            if warrior.is_alive() && mage.is_alive() {
                mage.process_effects();
            }
        }
    }
    println!("Warrior: {} HP", warrior.current_health);
    println!("Mage: {} HP", mage.current_health);

    if let Some(job) = mage.jobs.get_key_value(&JobType::Theurgist) {
        println!("Mage has {} points in the {} job", job.1.points, job.1.job_type);
    }
}