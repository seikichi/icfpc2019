use getopts::Options;
use std::env;
use std::io::{self, Read};

use lib::solution::*;
use lib::task::{BoosterCode, Task};
use lib::wrapper::cloning::CloningWrapper;
use lib::wrapper::*;

use rand::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("b", "", "set boosters", "BOOSTERS");
    opts.optopt("c", "", "counts", "COUNTS");
    opts.optopt("u", "", "bucket size", "BUCKET");
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
        None => 500,
    };
    let bucket_size = match matches.opt_str("u") {
        Some(c) => c.parse::<usize>().expect("failed to parse -u"),
        None => 10,
    };

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let task = Task::from(&buffer);

    let mut wrapper = CloningWrapper::new(&task, &boosters, 1 << 30, 1);
    let mut best_solution = wrapper.wrap(&task);
    eprintln!("{} {}", 1 << 30, best_solution.step());
    let max_step_size = best_solution.step() as usize / 30 + 1;

    let mut rng = rand::thread_rng();
    let mut wrappers = vec![];
    let wrapper = CloningWrapper::new(&task, &boosters, 1 << 30, 1);
    wrappers.push((best_solution.step(), wrapper));

    for _s in 0..count {
        eprintln!("{}", _s);
        let r = rng.gen::<usize>() % wrappers.len();
        let mut wrapper = wrappers[r].1.clone();
        let random_move_ratios = vec![10, 100, 1000, 10000];
        let r = rng.gen::<usize>() % random_move_ratios.len();
        wrapper.random_move_ratio = random_move_ratios[r];
        small_step(&mut wrapper, max_step_size);
        let copy_wrapper = wrapper.clone();

        wrapper.random_move_ratio = 1 << 30;
        let solution = full_step(&task, &mut wrapper);
        eprintln!("{} {}", _s, solution.step());
        if !wrapper.is_finished() {
            wrappers.push((solution.step(), copy_wrapper));
        }
        if solution.step() < best_solution.step() {
            best_solution = solution;
        }

        wrappers.sort_by_key(|k| k.0);
        if wrappers.len() > bucket_size {
            wrappers.pop();
        }
    }
    eprintln!("{}", best_solution.step());
    println!("{}", best_solution.to_string());

    Ok(())
}

fn small_step(wrapper: &mut CloningWrapper, step: usize) {
    for _i in 0..step {
        if wrapper.is_finished() {
            break;
        }
        wrapper.wrap_one_step()
    }
}

fn full_step(task: &Task, wrapper: &mut CloningWrapper) -> Solution {
    wrapper.wrap(&task)
}