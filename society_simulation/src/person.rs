use crate::team::Team;

static COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub struct Person {
    id: u32,
    team: Team,
}

impl Person {
    pub fn new() -> Person {
        Person {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            team: Team::White,
        }
    }

    pub fn get_team(&self) -> &Team {
        &self.team
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
