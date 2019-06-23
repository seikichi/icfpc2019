use getopts::Options;
use std::env;
use std::io::{self, Read};

use lib::task::{BoosterCode, Task};
use lib::wrapper::cloning::CloningWrapper;
use lib::wrapper::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("b", "", "set boosters", "BOOSTERS");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let boosters = match matches.opt_str("b") {
        Some(bs) => bs
            .chars()
            .map(|c| BoosterCode::from(&c.to_string()))
            .collect(),
        None => vec![],
    };

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let task = Task::from(&buffer);
    let mut wrapper = CloningWrapper::new(&task, &boosters, 1 << 30);
    let mut best_solution = wrapper.wrap(&task);
    // eprintln!("{} {}", 1 << 30, best_solution.step());
    // let random_move_ratios = vec![10, 100, 1000, 10000];
    // for &r in random_move_ratios.iter() {
    //     let mut wrapper = CloningWrapper::new(&task, &boosters, r);
    //     let solution = wrapper.wrap(&task);
    //     eprintln!("{} {}", r, solution.step());
    //     if solution.step() < best_solution.step() {
    //         best_solution = solution;
    //     }
    // }
    println!("{}", best_solution.to_string());

    Ok(())
}
