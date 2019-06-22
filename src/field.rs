use crate::solution::*;
use crate::task::*;
use crate::union_find::*;
pub struct Worker {
    pub p: Point,
    pub manipulators: Vec<Point>,
    pub cw_rotation_count: i32,
    pub fast_time: i32,
    pub drill_time: i32,
}

impl Worker {
    pub fn new(p: Point) -> Self {
        Worker {
            p: p,
            manipulators: vec![Point::new(1, -1), Point::new(1, 0), Point::new(1, 1)],
            cw_rotation_count: 0,
            fast_time: 0,
            drill_time: 0,
        }
    }
    pub fn movement(&mut self, p: Point, field: &mut Field, booster_cnts: &mut Vec<usize>) {
        let cnt = if self.fast_time > 0 { 2 } else { 1 };
        for iter in 0..cnt {
            let np = self.p + p;
            let movable = if self.drill_time > 0 {
                field.in_map(np)
            } else {
                field.movable(np)
            };
            if iter == 0 && !movable {
                panic!("can't move!")
            }
            if !movable {
                continue;
            }
            self.p = np;
            field.update_surface(self, booster_cnts);
        }
    }
    pub fn act(&mut self, action: Action, field: &mut Field, booster_cnts: &mut Vec<usize>) {
        match action {
            Action::MoveUp => {
                self.movement(Point::new(0, 1), field, booster_cnts);
            }
            Action::MoveDown => {
                self.movement(Point::new(0, -1), field, booster_cnts);
            }
            Action::MoveLeft => {
                self.movement(Point::new(1, 0), field, booster_cnts);
            }
            Action::MoveRight => {
                self.movement(Point::new(-1, 0), field, booster_cnts);
            }
            Action::DoNothing => {
                field.update_surface(self, booster_cnts);
            }
            Action::AttachManipulator { dx, dy } => {
                booster_cnts[BoosterCode::ExtensionOfTheManipulator as usize] -= 1;
                self.manipulators.push(Point::new(dx, dy));
                self.check_manipulator_constraint();
                field.update_surface(self, booster_cnts);
            }
            Action::AttachFastWheels => {
                booster_cnts[BoosterCode::FastWheels as usize] -= 1;
                self.fast_time = 50;
            }
            Action::AttachDrill => {
                booster_cnts[BoosterCode::Drill as usize] -= 1;
                self.drill_time = 30;
            }
            Action::Cloning => {
                let mystery_point = Square::Booster {
                    code: BoosterCode::MysteriousPoint,
                };
                if field[self.p.y as usize][self.p.x as usize] != mystery_point {
                    panic!("Here is not Mysterious Point");
                }
                booster_cnts[BoosterCode::Cloning as usize] -= 1;
                unimplemented!();
            }
            Action::TurnCW => {
                self.cw_rotation_count = (self.cw_rotation_count + 1) % 4;
            }
            Action::TurnCCW => {
                self.cw_rotation_count = (self.cw_rotation_count + 3) % 4;
            }
            _ => unimplemented!(),
        }
        self.fast_time -= 1;
        self.drill_time -= 1;
    }
    fn check_manipulator_constraint(&self) {
        let n = self.manipulators.len();
        let mut uf = UnionFind::new(n);
        for i in 0..n {
            for j in i + 1..n {
                if self.manipulators[i] == self.manipulators[j] {
                    panic!("multi manipulator is same position");
                }
                if (self.manipulators[i] - self.manipulators[j]).manhattan_dist() == 1 {
                    uf.union_set(i, j);
                }
            }
        }
        if uf.size(0) != n {
            panic!("Manipulator constraint is not satisfied");
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Square {
    Surface,
    WrappedSurface,
    Obstacle,
    Booster { code: BoosterCode },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Field {
    pub field: Vec<Vec<Square>>,
    pub booster_field: Vec<Vec<Square>>,
    pub rest_booster_cnts: Vec<usize>,
    pub rest_surface_cnt: usize,
}

impl Field {
    pub fn height(&self) -> usize {
        return self.field.len();
    }
    pub fn width(&self) -> usize {
        return self.field[0].len();
    }
    pub fn in_map(&self, p: Point) -> bool {
        return 0 <= p.x && p.x < self.width() as i32 && 0 <= p.y && p.y < self.height() as i32;
    }
    pub fn movable(&self, p: Point) -> bool {
        return self.in_map(p) && self[p.y as usize][p.x as usize] != Square::Obstacle;
    }
    pub fn update_surface(&mut self, worker: &Worker, booster_cnts: &mut Vec<usize>) {
        if (worker.drill_time > 0 && !self.in_map(worker.p)) || !self.movable(worker.p) {
            panic!("can't move this postion");
        }
        // get booster
        let booster = match self.booster_field[worker.p.y as usize][worker.p.x as usize] {
            Square::Booster { code } if code == BoosterCode::MysteriousPoint => {}
            Square::Booster { code } => {
                self.rest_booster_cnts[code as usize] -= 1;
                booster_cnts[code as usize] += 1;
            }
            _ => {}
        };
        // update wrapped surface
        self.wrap(worker.p);
        for &p in worker.manipulators.iter() {
            let p = p.rotate(worker.cw_rotation_count);
            if !self.movable(p) {
                continue;
            }
            // TODO 見えるかどうかのチェック
            self.wrap(p);
        }
        return booster;
    }
    pub fn wrap(&mut self, p: Point) {
        if self[p.y as usize][p.x as usize] == Square::Surface {
            self.rest_surface_cnt -= 1;
        }
        self[p.y as usize][p.x as usize] = Square::WrappedSurface;
    }
    pub fn finished(&self) -> bool {
        self.rest_surface_cnt == 0
    }


    pub fn from(task: &Task) -> Self {
        let Map(map) = &task.map;
        let x = map.iter().map(|p| p.x).max().unwrap() as usize;
        let y = map.iter().map(|p| p.y).max().unwrap() as usize;
        let mut field = vec![vec![Square::Unknown; x]; y];

        {
            let mut prev = &map[0];
            let mut ps = map.clone();
            ps.push(map[0].clone());
            for p in &ps {
                if p.x > prev.x && p.y > 0 {
                    for x in prev.x..p.x {
                        field[(p.y - 1) as usize][x as usize] = Square::Obstacle;
                    }
                }
                if p.y > prev.y && p.x < x as i32 {
                    for y in prev.y..p.y {
                        field[y as usize][p.x as usize] = Square::Obstacle;
                    }
                }
                if p.x < prev.x && p.y < y as i32 {
                    for x in p.x..prev.x {
                        field[p.y as usize][x as usize] = Square::Obstacle;
                    }
                }
                if p.y < prev.y && p.x > 0 {
                    for y in p.y..prev.y {
                        field[y as usize][(p.x - 1) as usize] = Square::Obstacle;
                    }
                }
                prev = p;
            }
        }

        // fill obstacle edges
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


        // Find Surfaces with bfs from start point
        let w = field[0].len();
        let h = field.len();
        let mut rest_surface_cnt = 0;

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(task.point.clone());

        while let Some(p) = queue.pop_front() {
            let y = p.y as usize;
            let x = p.x as usize;
            if field[y][x] != Square::Unknown {
                continue;
            }
            field[y][x] = Square::Surface;
            rest_surface_cnt += 1;

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
                if field[*ny as usize][*nx as usize] != Square::Unknown {
                    continue;
                }
                queue.push_back(Point::new(*nx, *ny));
            }
        }

        // fill boosters
        let mut booster_field = vec![vec![Square::Unknown; w]; h];
        let mut rest_booster_cnts = vec![0; 10];
        for b in &task.boosters {
            let y = b.point.y as usize;
            let x = b.point.x as usize;
            booster_field[y][x] = Square::Booster { code: b.code };
            rest_booster_cnts[b.code as usize] += 1;
        }

        // Mark cells as obstacles if not marked yet
        for y in 0..field.len() {
            for x in 0..field[0].len() {
                if field[y][x] == Square::Unknown {
                    field[y][x] = Square::Obstacle;
                }
            }
        }
        Field {
            field,
            booster_field,
            rest_booster_cnts,
            rest_surface_cnt,
        }
    }
}

// field[y][x] = Square::WrappedSurface とかできる
// []operator
impl std::ops::Index<usize> for Field {
    type Output = Vec<Square>;
    #[inline]
    fn index(&self, rhs: usize) -> &Vec<Square> {
        &self.field[rhs]
    }
}
impl std::ops::IndexMut<usize> for Field {
    #[inline]
    fn index_mut(&mut self, rhs: usize) -> &mut Vec<Square> {
        &mut self.field[rhs]
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
    let expected_field = vec![
        vec![
            Square::Surface,
            Square::Surface,
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
            Square::Surface,
            Square::Surface,
            Square::Surface,
        ],
    ];

    let expected_booster_field = vec![
        vec![
            Square::Unknown,
            Square::Booster {
                code: BoosterCode::FastWheels,
            },
            Square::Unknown,
            Square::Unknown,
        ],
        vec![
            Square::Unknown,
            Square::Obstacle,
            Square::Obstacle,
            Square::Unknown,
        ],
        vec![
            Square::Unknown,
            Square::Booster {
                code: BoosterCode::MysteriousPoint,
            },
            Square::Unknown,
            Square::Unknown,
        ],
    ];

    assert_eq!(field.field, expected_field);
    assert_eq!(field.booster_field, expected_booster_field);
    assert_eq!(
        field.rest_booster_cnts[BoosterCode::ExtensionOfTheManipulator as usize],
        0
    );
    assert_eq!(field.rest_booster_cnts[BoosterCode::FastWheels as usize], 1);
    assert_eq!(
        field.rest_booster_cnts[BoosterCode::MysteriousPoint as usize],
        1
    );
    assert_eq!(field.rest_surface_cnt, 10);
}
