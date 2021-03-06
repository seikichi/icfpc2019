use std::io::{self, Read};

use lib::puzzle::Puzzle;
use lib::puzzle_solver::PuzzleSolver;
use lib::task::Task;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let puzzle = Puzzle::from(&buffer);
    let mut solver = PuzzleSolver::new(&puzzle);
    solver.solve();
    let _obstacles = solver.get_obstacles();
    let task = Task {
        map: solver.get_map(),
        point: solver.start_point,
        obstacles: vec![],
        boosters: solver.boosters,
    };
    println!("{}", task.to_string());

    Ok(())
}
