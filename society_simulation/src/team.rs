use std::fmt;

pub enum Team {
    Police,
    White,
    Black,
    Yellow,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Team::Police => write!(f, "P"),
            Team::White => write!(f, "W"),
            Team::Black => write!(f, "B"),
            Team::Yellow => write!(f, "Y"),
        }
    }
}