use crate::solution::*;
use crate::task::*;
use crate::wrapper::Wrapper;

pub struct DfsWrapper {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Square {
    Surface,
    WrappedSurface,
    Obstacle,
    Booster { code: BoosterCode },
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Field(pub Vec<Vec<Square>>);

impl Field {
    fn from(task: &Task) -> Self {
        let Map(map) = &task.map;
        let x = map.iter().map(|p| p.x).max().unwrap() as usize;
        let y = map.iter().map(|p| p.y).max().unwrap() as usize;
        let mut field = vec![vec![Square::Surface; x]; y];

        for b in &task.boosters {
            let y = b.point.y as usize;
            let x = b.point.x as usize;
            field[y][x] = Square::Booster {
                code: b.code.clone(),
            };
        }

        // TODO: fill obstacles inside ...
        for Map(points) in &task.obstacles {
            let mut prev = &points[0];
            let mut ps = points.clone();
            ps.push(points[0].clone());
            for p in &ps {
                if p.x > prev.x {
                    for x in prev.x..p.x {
                        field[p.y as usize][x as usize] = Square::Obstacle;
                    }
                }
                if p.y > prev.y {
                    for y in prev.y..p.y {
                        field[y as usize][(p.x - 1) as usize] = Square::Obstacle;
                    }
                }
                if p.x < prev.x {
                    for x in p.x..prev.x {
                        field[(p.y - 1) as usize][x as usize] = Square::Obstacle;
                    }
                }
                if p.y < prev.y {
                    for y in p.y..prev.y {
                        field[y as usize][p.x as usize] = Square::Obstacle;
                    }
                }
                prev = p;
            }
        }
        Field(field)
    }
}

impl Wrapper for DfsWrapper {
    fn wrap(&mut self, task: &Task) -> Solution {
        let mut solution = vec![];
        let mut field = Field::from(task);
        let mut current = task.point.clone();
        while let Some(Solution(s)) = self.dfs(&mut current, &mut field) {
            let Field(field) = &mut field;
            field[current.y as usize][current.x as usize] = Square::WrappedSurface;
            solution.extend(s);
        }
        Solution(solution)
    }
}

impl DfsWrapper {
    fn dfs(&mut self, current: &mut Point, field: &mut Field) -> Option<Solution> {
        let Field(field) = field;
        let w = field[0].len();
        let h = field.len();

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((current.clone(), 0));

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
                        for (ny, nx, a) in &ns {
                            if *ny < 0 || *ny >= h as i32 || *nx < 0 || *nx >= w as i32 {
                                continue;
                            }
                            let ncost = visited[*ny as usize][*nx as usize];
                            if cost == ncost + 1 {
                                actions.push(a.clone());
                                y = *ny;
                                x = *nx;
                                break;
                            }
                        }
                    }

                    actions.reverse();
                    return Some(Solution(actions));
                }
                _ => {}
            }

            let ns = [
                (p.y - 1, p.x),
                (p.y + 1, p.x),
                (p.y, p.x - 1),
                (p.y, p.x + 1),
            ];
            for (ny, nx) in &ns {
                if *ny < 0 || *ny >= h as i32 || *nx < 0 || *nx >= w as i32 {
                    continue;
                }
                if visited[*ny as usize][*nx as usize] != -1 {
                    continue;
                }
                if field[*ny as usize][*nx as usize] == Square::Obstacle {
                    continue;
                }
                queue.push_back((Point::new(*nx, *ny), cost + 1));
            }
        }
        None
    }
}

#[test]
fn test_field_from() {
    // .X..
    // .**.
    // .F..

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

    let field = Field::from(&task);
    let expected = Field(vec![
        vec![
            Square::Surface,
            Square::Booster {
                code: BoosterCode::FastWheels,
            },
            Square::Surface,
            Square::Surface,
        ],
        vec![
            Square::Surface,
            Square::Obstacle,
            Square::Obstacle,
            Square::Surface,
        ],
        vec![
            Square::Surface,
            Square::Booster {
                code: BoosterCode::MysteriousPoint,
            },
            Square::Surface,
            Square::Surface,
        ],
    ]);
    assert_eq!(field, expected);
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
    let solution = wrapper.wrap(&task);
}
