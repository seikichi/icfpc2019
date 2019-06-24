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
    opts.optopt("c", "", "counts", "COUNTS");
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
    let count = match matches.opt_str("c") {
        Some(c) => c.parse::<usize>().expect("failed to parse -c"),
        None => 1,
    };

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let task = Task::from(&buffer);
    let mut wrapper = CloningWrapper::new(&task, &boosters, 1 << 30, 1);
    let mut best_solution = wrapper.wrap(&task);
    eprintln!("{} {}", 1 << 30, best_solution.step());

    for _s in 0..count {
        eprintln!("{}", _s);
        let random_move_ratios = vec![10, 100, 1000, 10000, 1 << 30];
        for &r in random_move_ratios.iter() {
            let mut wrapper = CloningWrapper::new(&task, &boosters, r, _s);
            let solution = wrapper.wrap(&task);
            eprintln!("{} {}", r, solution.step());
            if solution.step() < best_solution.step() {
                best_solution = solution;
            }
        }
    }
    eprintln!("{}", best_solution.step());

    Ok(())
}
