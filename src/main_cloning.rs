use std::io::{self, Read};

use lib::task::Task;
use lib::wrapper::cloning::CloningWrapper;
use lib::wrapper::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let task = Task::from(&buffer);
    let mut wrapper = CloningWrapper::new(&task);
    let solution = wrapper.wrap(&task);
    println!("{}", solution.to_string());
    eprintln!("{}", solution.step());

    Ok(())
}
