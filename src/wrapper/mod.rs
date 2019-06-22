use super::solution::*;
use super::task::*;

pub mod cloning;
pub mod dfs;

pub trait Wrapper {
    fn new(task: &Task) -> Self;
    fn wrap(&mut self, task: &Task) -> Solution;
}
