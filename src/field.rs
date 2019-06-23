use crate::solution::*;
use crate::task::*;
use crate::union_find::*;

use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
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
    pub fn movement(&mut self, p: Point, field: &mut Field) {
        let cnt = if self.fast_time > 0 { 2 } else { 1 };
        for iter in 0..cnt {
            let np = self.p + p;
            let movable = if self.drill_time > 0 {
                field.in_map(np)
            } else {
                field.movable(np)
            };
            if iter == 0 && !movable {
                eprintln!("{:?} {:?}", self, p);
                panic!("can't move!")
            }
            if !movable {
                continue;
            }
            self.p = np;
            field.update_surface(self);
        }
    }
    pub fn can_act(&self, action: Action, field: &Field, booster_cnts: &Vec<usize>) -> bool {
        match action {
            Action::MoveUp => field.movable(self.p + Point::new(0, 1)),
            Action::MoveDown => field.movable(self.p + Point::new(0, -1)),
            Action::MoveLeft => field.movable(self.p + Point::new(-1, 0)),
            Action::MoveRight => field.movable(self.p + Point::new(1, 0)),
            Action::DoNothing => true,
            Action::TurnCW => true,
            Action::TurnCCW => true,
            Action::AttachManipulator { dx, dy } => {
                let p = Point::new(dx, dy);
                booster_cnts[BoosterCode::ExtensionOfTheManipulator as usize] > 0
                    && self.check_manipulator_constraint(p)
            }
            Action::AttachFastWheels => booster_cnts[BoosterCode::FastWheels as usize] > 0,
            Action::AttachDrill => booster_cnts[BoosterCode::Drill as usize] > 0,
            Action::Cloning => {
                booster_cnts[BoosterCode::Cloning as usize] > 0
                    && field.booster_field[self.p.y as usize][self.p.x as usize]
                        == Square::Booster {
                            code: BoosterCode::MysteriousPoint,
                        }
            }
            _ => unimplemented!(),
        }
    }
    pub fn act(&mut self, action: Action, field: &mut Field, booster_cnts: &mut Vec<usize>) {
        match action {
            Action::MoveUp => {
                self.movement(Point::new(0, 1), field);
            }
            Action::MoveDown => {
                self.movement(Point::new(0, -1), field);
            }
            Action::MoveLeft => {
                self.movement(Point::new(-1, 0), field);
            }
            Action::MoveRight => {
                self.movement(Point::new(1, 0), field);
            }
            Action::DoNothing => {
                field.update_surface(self);
            }
            Action::AttachManipulator { dx, dy } => {
                booster_cnts[BoosterCode::ExtensionOfTheManipulator as usize] -= 1;
                let p = Point::new(dx, dy);
                if !self.check_manipulator_constraint(p) {
                    panic!("manipulator constraint is not satified");
                }
                self.manipulators.push(p);
            }
            Action::AttachFastWheels => {
                booster_cnts[BoosterCode::FastWheels as usize] -= 1;
                self.fast_time = 51;
            }
            Action::AttachDrill => {
                booster_cnts[BoosterCode::Drill as usize] -= 1;
                self.drill_time = 31;
            }
            Action::Cloning => {
                let mystery_point = Square::Booster {
                    code: BoosterCode::MysteriousPoint,
                };
                if field.booster_field[self.p.y as usize][self.p.x as usize] != mystery_point {
                    panic!("Here is not Mysterious Point");
                }
                booster_cnts[BoosterCode::Cloning as usize] -= 1;
                // Workerを増やす処理は呼び出し側がする事
            }
            Action::TurnCW => {
                self.manipulators = self.manipulators.iter().map(|p| p.rotate(1)).collect();
                self.cw_rotation_count = (self.cw_rotation_count + 1) % 4;
            }
            Action::TurnCCW => {
                self.manipulators = self.manipulators.iter().map(|p| p.rotate(3)).collect();
                self.cw_rotation_count = (self.cw_rotation_count + 3) % 4;
            }
            _ => unimplemented!(),
        }
        self.fast_time -= 1;
        self.drill_time -= 1;
    }
    fn check_manipulator_constraint(&self, p: Point) -> bool {
        let mut manipulators = self.manipulators.clone();
        manipulators.push(Point::new(0, 0));
        manipulators.push(p);
        let n = manipulators.len();
        let mut uf = UnionFind::new(n);
        for i in 0..n {
            for j in i + 1..n {
                if manipulators[i] == manipulators[j] {
                    return false;
                }
                if (manipulators[i] - manipulators[j]).manhattan_dist() == 1 {
                    uf.union_set(i, j);
                }
            }
        }
        if uf.size(0) != n {
            return false;
        }
        return true;
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Grids(Vec<Vec<i32>>);

impl Grids {
    pub fn find_point(&self, grid_id: i32) -> Point {
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if self.0[y][x] == grid_id {
                    return Point::new(x as i32, y as i32);
                }
            }
        }
        panic!("Failed to find ");
    }

    pub fn in_grid(&self, grid_id: i32, p: &Point) -> bool {
        self.0[p.y as usize][p.x as usize] == grid_id
    }

    pub fn from(field: &Field, num: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut initial_points: Vec<Point> = vec![];
        for _ in 0..num {
            loop {
                let x = rng.next_u32() % field.width() as u32;
                let y = rng.next_u32() % field.height() as u32;
                if field[y as usize][x as usize] == Square::Obstacle
                    || field[y as usize][x as usize] == Square::Unknown
                {
                    continue;
                }
                if initial_points
                    .iter()
                    .any(|p| p.x == x as i32 && p.y == y as i32)
                {
                    continue;
                }
                initial_points.push(Point::new(x as i32, y as i32));
                break;
            }
        }

        let mut grids = vec![];
        let mut ques = vec![];
        for i in 0..num {
            let mut que = std::collections::VecDeque::new();
            que.push_back(initial_points[i]);
            ques.push(que);
            grids.push(vec![]);
        }

        let mut visited = vec![vec![false; field.width()]; field.height()];
        while !ques.iter().all(|q| q.is_empty()) {
            for i in 0..num {
                if let Some(cur) = ques[i].pop_back() {
                    if visited[cur.y as usize][cur.x as usize] {
                        continue;
                    }
                    visited[cur.y as usize][cur.x as usize] = true;
                    grids[i].push(cur);

                    let dx = [1, 0, -1, 0];
                    let dy = [0, 1, 0, -1];
                    for d in 0..4 {
                        let npos = cur + Point::new(dx[d], dy[d]);

                        if !field.in_map(npos) {
                            continue;
                        }
                        if visited[npos.y as usize][npos.x as usize] {
                            continue;
                        }
                        let s = field[npos.y as usize][npos.x as usize];
                        if s == Square::Obstacle || s == Square::Unknown {
                            continue;
                        }
                        ques[i].push_back(npos);
                    }
                }
            }
        }
        let mut ids = vec![vec![-1; field.width()]; field.height()];
        for i in 0..num {
            for p in &grids[i] {
                ids[p.y as usize][p.x as usize] = i as i32;
            }
        }
        Grids(ids)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Square {
    Surface,
    WrappedSurface,
    Obstacle,
    Booster { code: BoosterCode },
    Unknown,
}
impl Square {
    pub fn get_char(&self) -> char {
        match self {
            Square::Surface => '*',
            Square::WrappedSurface => '@',
            Square::Obstacle => '#',
            Square::Booster { code } => match code {
                BoosterCode::ExtensionOfTheManipulator => 'B',
                BoosterCode::FastWheels => 'F',
                BoosterCode::Drill => 'L',
                BoosterCode::MysteriousPoint => 'X',
                BoosterCode::Teleport => 'R',
                BoosterCode::Cloning => 'C',
            },
            Square::Unknown => '.',
        }
    }
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
    pub fn get_square(&self, p: Point) -> Square {
        self[p.y as usize][p.x as usize]
    }
    pub fn get_booster_square(&self, p: Point) -> Square {
        self.booster_field[p.y as usize][p.x as usize]
    }
    pub fn update_surface(&mut self, worker: &Worker) {
        if (worker.drill_time > 0 && !self.in_map(worker.p))
            || (worker.drill_time <= 0 && !self.movable(worker.p))
        {
            eprintln!("{:?}", worker);
            panic!("can't move this postion");
        }
        // update wrapped surface
        self.wrap(worker.p);
        for &p in worker.manipulators.iter() {
            let p = worker.p + p;
            if !self.none_block(worker.p, p) {
                continue;
            }
            self.wrap(p);
        }
    }
    // p1, p2の視線がmovableかどうかチェックする
    pub fn none_block(&self, p1: Point, p2: Point) -> bool {
        let sx = std::cmp::min(p1.x, p2.x);
        let sy = std::cmp::min(p1.y, p2.y);
        let ex = std::cmp::max(p1.x, p2.x);
        let ey = std::cmp::max(p1.y, p2.y);
        if (ex - sx).abs() <= 1 && (ey - sy).abs() <= 1 {
            return self.movable(p1) && self.movable(p2);
        }
        for y in sy..ey + 1 {
            for x in sx..ex + 1 {
                if !self.movable(Point::new(x, y)) {
                    return false;
                }
            }
        }
        return true;
    }
    pub fn get_booster(&mut self, worker: &Worker, booster_cnts: &mut Vec<usize>) {
        match self.booster_field[worker.p.y as usize][worker.p.x as usize] {
            Square::Booster { code } if code == BoosterCode::MysteriousPoint => {}
            Square::Booster { code } => {
                self.rest_booster_cnts[code as usize] -= 1;
                self.booster_field[worker.p.y as usize][worker.p.x as usize] = Square::Unknown;
                booster_cnts[code as usize] += 1;
            }
            _ => {}
        };
    }
    pub fn wrap(&mut self, p: Point) {
        if self[p.y as usize][p.x as usize] == Square::Surface {
            self.rest_surface_cnt -= 1;
        }
        self[p.y as usize][p.x as usize] = Square::WrappedSurface;
    }
    pub fn is_finished(&self) -> bool {
        self.rest_surface_cnt == 0
    }
    pub fn print(&self, sx: usize, sy: usize, w: usize, h: usize) {
        for y in (sy..sy + h).rev() {
            if y >= self.height() {
                continue;
            }
            for x in sx..sx + w {
                if x >= self.width() {
                    break;
                }
                let square = if self[y][x] != Square::Unknown {
                    self[y][x]
                } else {
                    self.booster_field[y][x]
                };
                print!("{}", square.get_char());
            }
            println!("");
        }
    }

    // lockはそのsquareがtargetであっても無視する(通らない)
    pub fn bfs(
        &self,
        worker: &Worker,
        target: Square,
        target_point: Point,
        lock: &Vec<Point>,
        grids: Option<&Grids>,
        grid_id: Option<i32>,
    ) -> Option<(Point, Vec<Action>)> {
        let w = self.width();
        let h = self.height();

        let current = worker.p;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((current, 0));

        let mut visited = vec![vec![-1; w]; h];
        let mut parents = vec![vec![Point::new(-1, -1); w]; h];
        for p in lock.iter() {
            visited[p.y as usize][p.x as usize] = -2;
        }
        visited[current.y as usize][current.x as usize] = 0;

        while let Some((p, cost)) = queue.pop_front() {
            let y = p.y as usize;
            let x = p.x as usize;
            if (target != Square::Unknown
                && (self[y][x] == target || self.booster_field[y][x] == target))
                || p == target_point
            {
                let mut p = p;
                let end_p = Point::new(x as i32, y as i32);
                let mut actions = vec![];
                while visited[p.y as usize][p.x as usize] != 0 {
                    let ppos = parents[p.y as usize][p.x as usize];

                    let a = if p.y > ppos.y {
                        Action::MoveUp
                    } else if p.y < ppos.y {
                        Action::MoveDown
                    } else if p.x > ppos.x {
                        Action::MoveRight
                    } else if p.x < ppos.x {
                        Action::MoveLeft
                    } else {
                        eprintln!("{:?} {:?} {:?}", p, ppos, worker);
                        assert!(visited[p.y as usize][p.x as usize] >= 0);
                        panic!("wrong move");
                    };
                    p = ppos;
                    actions.push(a);
                }

                actions.reverse();
                if actions.is_empty() {
                    actions.push(Action::DoNothing);
                }
                return Some((end_p, actions));
            }

            let move_distance = if worker.fast_time - cost > 0 { 2 } else { 1 };
            let drill = worker.drill_time - cost > 0;
            let dy = vec![1, -1, 0, 0];
            let dx = vec![0, 0, 1, -1];
            'outer_loop: for dir in 0..4 {
                let mut np = p;
                for _d in 0..move_distance {
                    np.x += dx[dir];
                    np.y += dy[dir];
                    let movable = if drill {
                        self.in_map(np)
                    } else {
                        self.movable(np)
                    };
                    if !movable {
                        np.x -= dx[dir];
                        np.y -= dy[dir];
                        break;
                    }
                }
                if let (Some(grids), Some(grid_id)) = (grids, grid_id) {
                    if !grids.in_grid(grid_id, &np) {
                        continue;
                    }
                }
                if visited[np.y as usize][np.x as usize] != -1 {
                    continue;
                }
                visited[np.y as usize][np.x as usize] = cost + 1;
                parents[np.y as usize][np.x as usize] = p;
                queue.push_back((np, cost + 1));
            }
        }
        None
    }

    pub fn dijkstra(&self, worker: &Worker, target: Square) -> Option<(Point, Vec<Action>)> {
        let w = self.width();
        let h = self.height();

        let mut queue = std::collections::BinaryHeap::new();
        queue.push((0, worker.p));

        let mut visited = vec![vec![false; w]; h];
        let mut costs = vec![vec![1 << 30; w]; h];
        let mut parents = vec![vec![Point::new(0, 0); w]; h];

        while let Some((cost, p)) = queue.pop() {
            let cost = -cost;
            let y = p.y as usize;
            let x = p.x as usize;
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            if self[y][x] == target || self.booster_field[y][x] == target {
                let end_p = Point::new(x as i32, y as i32);
                let mut actions = vec![];
                let mut p = p;
                while p != worker.p {
                    let ppos = parents[p.y as usize][p.x as usize];
                    let ns = [
                        (p.y - 1, p.x, Action::MoveUp),
                        (p.y + 1, p.x, Action::MoveDown),
                        (p.y, p.x - 1, Action::MoveRight),
                        (p.y, p.x + 1, Action::MoveLeft),
                    ];
                    for &(ny, nx, a) in &ns {
                        if ppos.y != ny || ppos.x != nx {
                            continue;
                        }
                        actions.push(a);
                        p = ppos;
                        break;
                    }
                }

                actions.reverse();
                if actions.is_empty() {
                    actions.push(Action::DoNothing);
                }
                return Some((end_p, actions));
            }

            let ns = [
                (p.y + 1, p.x),
                (p.y - 1, p.x),
                (p.y, p.x + 1),
                (p.y, p.x - 1),
            ];
            for &(ny, nx) in &ns {
                let np = Point::new(nx, ny);
                if !self.movable(np) {
                    continue;
                }
                let ncost = if self[ny as usize][nx as usize] == Square::WrappedSurface {
                    cost
                } else {
                    cost + 1
                };
                if visited[ny as usize][nx as usize] || ncost >= costs[ny as usize][nx as usize] {
                    continue;
                }
                costs[ny as usize][nx as usize] = ncost;
                parents[ny as usize][nx as usize] = p;
                queue.push((-ncost, np));
            }
        }
        None
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
            Square::Unknown,
            Square::Unknown,
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
