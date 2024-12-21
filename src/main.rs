use neargate::{Equippable, JobType, Spell, Unit, SPELLS};

fn main() {
    let mut warrior = Unit::new("Warrior");
    warrior.accuracy += 50.0;
    warrior.evasion += 50.0;
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

    while warrior.is_alive() && mage.is_alive() {
        if warrior.is_alive() {
            warrior.attack(&mut mage);
        }
        if mage.is_alive() {
            mage.cast(&mut warrior, &SPELLS[1]);
        }
    }
    println!("Warrior: {} HP", warrior.current_health);
    println!("Mage: {} HP", mage.current_health);

    if let Some(job) = mage.jobs.get_key_value(&JobType::Theurgist) {
        println!("Mage has {} points in the {} job", job.1.points, job.1.job_type);
    }
}