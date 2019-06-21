use crate::solution::Solution;
use crate::task::{BoosterCode, Task};
use crate::wrapper::Wrapper;

pub struct DfsWrapper {}

enum Square {
    Surface,
    WrappedSurface,
    Obstacle,
    Booster { code: BoosterCode },
}

impl Wrapper for DfsWrapper {
    fn wrap(&mut self, task: &mut Task) -> Solution {
        let mut solution = vec![];
        while let Some(Solution(s)) = self.dfs(task) {
            solution.extend(s);
        }
        Solution(solution)
    }
}

impl DfsWrapper {
    fn dfs(&mut self, task: &mut Task) -> Option<Solution> {
        None
    }
}
