use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Hash, Default)]
pub enum JobType {
    #[default]
    None,
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
    Mercenary,
    Vanguard,

    // direct damage spells
    // t2
    Arcanist,
    // buff/debuff spells
    // t2
    Theurgist,
    // arcanist theurgist combination
    // t3, t4
    Scholar,
    Conduit,

    // duelist arcanist combination
    // t3, t4
    Custodian,
    Warden,
    // duelist theurgist combination
    // t3, t4
    Templar,
    Crusader,

    // scout arcanist combination
    // t3, t4
    Tactician,
    Strategist,
    // scout theurgist combination
    //t3, t4
    Missionary,
    Orator,
}

impl Copy for JobType {}

impl Clone for JobType {
    fn clone(&self) -> Self {
        *self
    }
}

impl Display for JobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JobType::None => "None",
                JobType::Soldier => "Soldier",
                JobType::Student => "Student",
                JobType::Duelist => "Duelist",
                JobType::Scout => "Scout",
                JobType::Mercenary => "Mercenary",
                JobType::Vanguard => "Vanguard",
                JobType::Arcanist => "Arcanist",
                JobType::Theurgist => "Theurgist",
                JobType::Scholar => "Scholar",
                JobType::Conduit => "Conduit",
                JobType::Custodian => "Custodian",
                JobType::Warden => "Warden",
                JobType::Templar => "Templar",
                JobType::Crusader => "Crusader",
                JobType::Tactician => "Tactician",
                JobType::Strategist => "Strategist",
                JobType::Missionary => "Missionary",
                JobType::Orator => "Orator",
            }
        )
    }
}
