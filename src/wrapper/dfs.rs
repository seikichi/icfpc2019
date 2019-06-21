use crate::solution::Solution;
use crate::task::Task;
use crate::wrapper::Wrapper;

pub struct DfsWrapper {}

impl Wrapper for DfsWrapper {
    fn wrap(&mut self, task: &mut Task) -> Solution {
        Solution(vec![])
    }
}
