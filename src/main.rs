use std::io::{self, Read};

use lib::task::Task;
use lib::wrapper::dfs::DfsWrapper;
use lib::wrapper::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let task = Task::from(&buffer);
    let mut wrapper = DfsWrapper {};
    let solution = wrapper.wrap(&task);
    println!("{}", solution.to_string());

    Ok(())
}
