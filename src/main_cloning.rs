use std::io::{self, Read};

use lib::task::Task;
use lib::wrapper::cloning::CloningWrapper;
use lib::wrapper::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let task = Task::from(&buffer);
    let mut wrapper = CloningWrapper::new(&task, 1 << 30);
    let mut best_solution = wrapper.wrap(&task);
    eprintln!("{} {}", 1 << 30, best_solution.step());
    let random_move_ratios = vec![
        10, 100, 1000, 10000,
    ];
    for &r in random_move_ratios.iter() {
        let mut wrapper = CloningWrapper::new(&task, r);
        let solution= wrapper.wrap(&task);
        eprintln!("{} {}", r, solution.step());
        if solution.step() < best_solution.step() {
            best_solution = solution;
        }
    }
    println!("{}", best_solution.to_string());

    Ok(())
}
