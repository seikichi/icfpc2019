use super::solution::*;
use super::task::*;

pub mod dfs;

pub trait Wrapper {
    fn wrap(&mut self, task: &Task) -> Solution;
}
