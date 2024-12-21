use crate::JobType;

pub mod job_type;

pub struct Job {
    pub job_type: JobType,
    pub level: u8,
    pub experience: u32,
    pub max_experience: u32,
    pub points: u32,
}

impl Job {
    pub fn new(job_type: JobType) -> Job {
        Job {
            job_type,
            level: 1,
            experience: 0,
            max_experience: 100,
            points: 0,
        }
    }

    pub fn level_up(&mut self) {
        if self.level < std::u8::MAX {
            self.level += 1;
            self.experience = 0;
            self.max_experience *= 3 / 2;
        }
    }
}