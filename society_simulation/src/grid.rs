use crate::person::Person;
use rand::random;

pub struct Grid {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: u8, height: u8) -> Grid {
        let mut cells = Vec::new();
        for x in 0..width {
            for y in 0..height {
                cells.push(Cell::new(x, y));
            }
        }

        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn get_cell(&self, x: u8, y: u8) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.x == x && cell.y == y)
    }

    pub fn display(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                match self.get_cell(x, y) {
                    Some(cell) => match cell.get_person() {
                        Some(person) => print!("{} ", person.get_team()),
                        None => print!(". "),
                    },
                    None => print!("  "),
                }
            }
            println!();
        }
    }
}

pub struct Cell {
    x: u8,
    y: u8,
    person: Option<Person>,
}

impl Cell {
    pub fn new(x: u8, y: u8) -> Cell {
        Cell {
            x: x,
            y: y,
            person: if random() {
                Some(Person::new("Bob"))
            } else {
                None
            },
        }
    }

    pub fn set_person(&mut self, person: Person) {
        self.person = Some(person);
    }

    pub fn remove_person(&mut self) -> Option<Person> {
        self.person.take()
    }

    pub fn get_person(&self) -> Option<&Person> {
        self.person.as_ref()
    }
}
