// jobs are what makes units unique and different from each other
pub enum Job {
    // most weapon masters began as soldiers
    // t1
    Soldier,
    // many spell casters began as students
    // t1
    Student,

    // strength based soldier
    // t2
    Duelist,
    // agility based physical damage dealer
    // t2
    Scout,
    // duelist scout combination
    // t3, t4
    Mercenary, Vanguard,

    // direct damage spells
    // t2
    Arcanist,
    // buff/debuff spells
    // t2
    Theurgist,
    // arcanist theurgist combination
    // t3, t4
    Scholar, Conduit,

    // duelist arcanist combination
    // t3, t4
    Custodian, Warden,
    // duelist theurgist combination
    // t3, t4
    Templar, Crusader,

    // scout arcanist combination
    // t3, t4
    Tactician, Strategist,
    // scout theurgist combination
    //t3, t4
    Missionary, Orator,
}