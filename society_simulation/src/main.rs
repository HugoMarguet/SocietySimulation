mod grid;
mod person;
mod team;

use grid::Grid;

fn main() {
    let grid = Grid::new(10, 10);
    grid.display();
}
