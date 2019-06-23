use crate::field::*;
use crate::solution::*;
use crate::task::*;
use crate::wrapper::Wrapper;

pub struct DfsWrapper {}

impl Wrapper for DfsWrapper {
    fn new(_task: &Task) -> Self {
        DfsWrapper {}
    }
    fn wrap(&mut self, task: &Task) -> Solution {
        let mut solution = vec![];
        let mut field = Field::from(task);
        let mut current = task.point;
        let manipulators = vec![Point::new(1, -1), Point::new(1, 0), Point::new(1, 1)];
        while let Some(s) = self.dfs(&mut current, &mut field) {
            field[current.y as usize][current.x as usize] = Square::WrappedSurface;
            for &p in manipulators.iter() {
                let np = current + p;
                if !field.movable(np) {
                    continue;
                }
                field[np.y as usize][np.x as usize] = Square::WrappedSurface
            }
            solution.extend(s);
        }
        Solution(vec![solution])
    }
}

impl DfsWrapper {
    fn dfs(&mut self, current: &mut Point, field: &mut Field) -> Option<Vec<Action>> {
        let w = field.width();
        let h = field.height();

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((*current, 0));

        let mut visited = vec![vec![-1; w]; h];

        while let Some((p, cost)) = queue.pop_front() {
            let y = p.y as usize;
            let x = p.x as usize;
            if visited[y][x] != -1 {
                continue;
            }
            visited[y][x] = cost;
            match field[y][x] {
                Square::Surface | Square::Booster { .. } => {
                    let mut actions = vec![];
                    let mut y = y as i32;
                    let mut x = x as i32;
                    current.y = y;
                    current.x = x;
                    while visited[y as usize][x as usize] != 0 {
                        let cost = visited[y as usize][x as usize];

                        let ns = [
                            (y - 1, x, Action::MoveUp),
                            (y + 1, x, Action::MoveDown),
                            (y, x - 1, Action::MoveRight),
                            (y, x + 1, Action::MoveLeft),
                        ];
                        for &(ny, nx, a) in &ns {
                            if !field.in_map(Point::new(nx, ny)) {
                                continue;
                            }
                            let ncost = visited[ny as usize][nx as usize];
                            if cost == ncost + 1 {
                                actions.push(a);
                                y = ny;
                                x = nx;
                                break;
                            }
                        }
                    }

                    actions.reverse();
                    return Some(actions);
                }
                _ => {}
            }

            let ns = [
                (p.y, p.x + 1),
                (p.y, p.x - 1),
                (p.y - 1, p.x),
                (p.y + 1, p.x),
            ];
            for &(ny, nx) in &ns {
                let np = Point::new(nx, ny);
                if !field.movable(np) {
                    continue;
                }
                if visited[ny as usize][nx as usize] != -1 {
                    continue;
                }
                queue.push_back((np, cost + 1));
            }
        }
        None
    }
}

#[test]
fn test_dfs() {
    // .X..
    // .**.
    // sF..

    let map = Map(vec![
        Point::new(0, 0),
        Point::new(4, 0),
        Point::new(4, 3),
        Point::new(0, 3),
    ]);
    let obstacles = vec![Map(vec![
        Point::new(1, 1),
        Point::new(3, 1),
        Point::new(3, 2),
        Point::new(1, 2),
    ])];
    let boosters = vec![
        BoosterLocation::new(BoosterCode::FastWheels, Point::new(1, 0)),
        BoosterLocation::new(BoosterCode::MysteriousPoint, Point::new(1, 2)),
    ];
    let point = Point::new(0, 0);
    let task = Task {
        point,
        map,
        obstacles,
        boosters,
    };

    let mut wrapper = DfsWrapper {};
    let _solution = wrapper.wrap(&task);
}
