use crate::team::Team;

pub struct Person {
    name: String,
    team: Team,
}

impl Person {
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
            team: Team::White,
        }
    }

    pub fn get_team(&self) -> &Team {
        &self.team
    }
}
