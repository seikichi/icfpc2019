use crate::field::*;
use crate::puzzle::*;
use crate::task::*;

use rand::Rng;

pub struct PuzzleSolver {
    puzzle: Puzzle,
    field: Field,
    pub start_point: Point,
    pub boosters: Vec<BoosterLocation>,
}

impl PuzzleSolver {
    pub fn new(puzzle: &Puzzle) -> Self {
        let mut field = Field {
            field: vec![vec![Square::Unknown; puzzle.xy_max()]; puzzle.xy_max()],
            booster_field: vec![vec![Square::Unknown; puzzle.xy_max()]; puzzle.xy_max()],
            rest_booster_cnts: vec![0; 10],
            rest_surface_cnt: 1 << 30,
        };
        for &p in puzzle.o_seq.iter() {
            field[p.y as usize][p.x as usize] = Square::Obstacle;
        }
        for &p in puzzle.i_seq.iter() {
            field[p.y as usize][p.x as usize] = Square::Surface;
        }
        'outer_loop: for y in (0..field.height()).rev() {
            for x in (0..field.width()).rev() {
                if field[y][x] == Square::Obstacle {
                    continue;
                }
                field[y][x] = Square::Surface;
                break 'outer_loop;
            }
        }

        // for &p in puzzle.o_seq.iter() {
        //     for dy in -1..2 {
        //         for dx in -1..2 {
        //             let ny = p.y + dy;
        //             let nx = p.x + dx;
        //             if !field.in_map(Point::new(ny, nx))
        //                 || field[ny as usize][nx as usize] == Square::Obstacle
        //             {
        //                 continue;
        //             }
        //             field[ny as usize][nx as usize] = Square::Surface;
        //         }
        //     }
        // }

        // // *.. => **.
        // // .*.    .*.
        // for y in 0..field.height() {
        //     for x in 0..field.width() {
        //         if field[y][x] != Square::Obstacle {
        //             continue;
        //         }
        //         let dx = [-1, 1];
        //         let dy = [1, 1];
        //         for d in 0..2 {
        //             let corner = Point::new((x as i32) + dx[d], (y as i32) + dy[d]);
        //             let side = Point::new((x as i32) + dx[d], (y as i32));

        //             if !field.in_map(corner) {
        //                 continue;
        //             }
        //             if field[corner.y as usize][corner.x as usize] == Square::Obstacle
        //                 && field[side.y as usize][side.x as usize] != Square::Obstacle
        //             {
        //                 if field[side.y as usize][x] == Square::Surface {
        //                     panic!("Failed to fix obstacle intersect");
        //                 }
        //                 field[corner.y as usize][x] = Square::Obstacle;
        //             }
        //         }
        //     }
        // }

        PuzzleSolver {
            puzzle: puzzle.clone(),
            field: field,
            start_point: Point::new(0, 0),
            boosters: vec![],
        }
    }

    pub fn solve(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let initial_field = self.field.clone();
            let r = rng.gen::<usize>() % self.puzzle.i_seq.len();
            let start_pos = self.puzzle.i_seq[r];
            let mut candidate_poss = vec![start_pos];
            self.field[start_pos.y as usize][start_pos.x as usize] = Square::WrappedSurface;
            loop {
                loop {
                    let r = rng.gen::<usize>() % candidate_poss.len();
                    let pos = candidate_poss[r];
                    let mut worker = Worker::new(pos);
                    worker.manipulators = vec![];
                    if let Some((_end_pos, actions)) = self.field.dijkstra(&worker, Square::Surface)
                    {
                        for &action in actions.iter() {
                            worker.act(action, &mut self.field, &mut vec![0; 10]);
                            candidate_poss.push(worker.p);
                        }
                    } else {
                        break;
                    }
                }
                break;
                // TODO
                let need_area = self.puzzle.area_min() as i32 - self.count_area() as i32;
                if need_area <= 0 {
                    break;
                }
                loop {
                    let y = rng.gen::<usize>() % self.field.height();
                    let x = rng.gen::<usize>() % self.field.width();
                    if self.field[y][x] == Square::Unknown {
                        self.field[y][x] = Square::Surface;
                        break;
                    }
                }
            }
            break;
            // x
        }
        // let initial_field = self.field.clone();
        loop {
            let need_vertex = self.puzzle.v_min as i32 - self.count_vertex() as i32;
            if need_vertex <= 0 {
                break;
            }
            let mut first = true;
            let mut pos = self.get_left_bottom_position();
            let mut vertexs = vec![pos];
            let mut dir = 0;
            let mut cnt = need_vertex / 2 + 1;
            loop {
                if cnt == 0 {
                    break;
                }
                let move_cnt = rng.gen::<usize>() % 30 + 3;
                for _iter2 in 0..move_cnt {
                    first = true;
                    self.one_move(
                        &mut pos,
                        &mut dir,
                        &mut vertexs,
                        &mut first,
                        Square::WrappedSurface,
                    );
                }
                let dx = [1, 0, -1, 0];
                let dy = [0, 1, 0, -1];
                dir = (dir + 3) % 4;
                let npos = pos + Point::new(dx[dir], dy[dir]);
                if !self.field.movable(npos) || !self.can_set(npos, dir) {
                    dir = (dir + 1) % 4;
                    continue;
                }
                self.field[npos.y as usize][npos.x as usize] = Square::WrappedSurface;
                pos = npos;
                cnt -= 1;
            }
            // self.field.print(0, 0, 150, 150);
        }
        let mut first = true;
        let mut rest_boosters = vec![];
        for i in 0..self.puzzle.booster_num.len() {
            let booster = match i {
                0 => BoosterCode::ExtensionOfTheManipulator,
                1 => BoosterCode::FastWheels,
                2 => BoosterCode::Drill,
                3 => BoosterCode::MysteriousPoint,
                4 => BoosterCode::Teleport,
                5 => BoosterCode::Cloning,
                _ => panic!(""),
            };
            for _j in 0..self.puzzle.booster_num[i] {
                rest_boosters.push(booster);
            }
        }
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if self.field[y][x] == Square::WrappedSurface {
                    if first {
                        self.start_point = Point::new(x as i32, y as i32);
                        first = false;
                    } else {
                        if let Some(booster) = rest_boosters.pop() {
                            self.boosters.push(BoosterLocation {
                                code: booster,
                                point: Point::new(x as i32, y as i32),
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn can_set(&self, pos: Point, dir: usize) -> bool {
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        let pos2 = pos + Point::new(dx[dir], dy[dir]);
        if self.field.in_map(pos2)
            && self.field[pos2.y as usize][pos2.x as usize] == Square::WrappedSurface
        {
            return false;
        }
        for d1 in -1..2 {
            if d1 == 0 {
                continue;
            }
            let ndir = (dir as i32 + 4 + d1) as usize % 4;
            let npos = pos + Point::new(dx[ndir], dy[ndir]);
            if self.field.in_map(npos)
                && self.field[npos.y as usize][npos.x as usize] == Square::WrappedSurface
            {
                return false;
            }
            let npos = pos2 + Point::new(dx[ndir], dy[ndir]);
            if self.field.in_map(npos)
                && self.field[npos.y as usize][npos.x as usize] == Square::WrappedSurface
            {
                return false;
            }
        }
        return true;
    }

    pub fn get_left_bottom_position(&self) -> Point {
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if self.field[y][x] == Square::WrappedSurface {
                    return Point::new(x as i32, y as i32);
                }
            }
        }
        panic!("failed");
    }

    pub fn count_vertex(&self) -> usize {
        let Map(vertexs) = self.get_map();
        vertexs.len()
    }

    pub fn count_area(&self) -> usize {
        let mut ret = 0;
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if self.field[y][x] == Square::WrappedSurface {
                    ret += 1;
                }
            }
        }
        return ret;
    }

    pub fn max_xy(&self) -> usize {
        let mut ret = 0;
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if self.field[y][x] == Square::WrappedSurface {
                    ret = std::cmp::max(ret, x);
                    ret = std::cmp::max(ret, y);
                }
            }
        }
        ret
    }

    pub fn get_map(&self) -> Map {
        let pos = self.get_left_bottom_position();
        return self.get_area(pos, Square::WrappedSurface);
    }

    pub fn get_obstacles(&mut self) -> Vec<Map> {
        let mut obstacles = vec![];
        let mut visited = vec![vec![false; self.field.width()]; self.field.height()];
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if self.field[y][x] != Square::WrappedSurface {
                    self.field[y][x] = Square::Obstacle;
                }
            }
        }
        for y in 0..self.field.height() {
            if y != 0 && y != self.field.height() - 1 {
                continue;
            }
            for x in 0..self.field.width() {
                if visited[y][x] || self.field[y][x] != Square::Obstacle {
                    continue;
                }
                self.bfs(Point::new(x as i32, y as i32), &mut visited);
            }
        }
        for x in 0..self.field.width() {
            if x != 0 && x != self.field.width() - 1 {
                continue;
            }
            for y in 0..self.field.height() {
                if visited[y][x] || self.field[y][x] != Square::Obstacle {
                    continue;
                }
                self.bfs(Point::new(x as i32, y as i32), &mut visited);
            }
        }

        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if visited[y][x] || self.field[y][x] != Square::Obstacle {
                    continue;
                }
                let obstacle = self.get_area(Point::new(x as i32, y as i32), Square::Obstacle);
                obstacles.push(obstacle);
                self.bfs(Point::new(x as i32, y as i32), &mut visited);
            }
        }
        return obstacles;
    }

    pub fn get_area(&self, pos: Point, target: Square) -> Map {
        let mut pos = pos;
        let mut vertexs = vec![pos];
        let mut dir = 0;
        let mut first = true;
        loop {
            if self.one_move(&mut pos, &mut dir, &mut vertexs, &mut first, target) {
                break;
            }
        }
        vertexs.pop();
        return Map(vertexs);
    }

    fn bfs(&self, p: Point, visited: &mut Vec<Vec<bool>>) {
        let mut que = std::collections::VecDeque::new();
        que.push_back(p);

        while let Some(cur) = que.pop_back() {
            visited[cur.y as usize][cur.x as usize] = true;

            let dx = [1, 0, -1, 0];
            let dy = [0, 1, 0, -1];
            for d in 0..4 {
                let npos = cur + Point::new(dx[d], dy[d]);
                if !self.field.in_map(npos) {
                    continue;
                }
                if self.field[npos.y as usize][npos.x as usize] != Square::Obstacle {
                    continue;
                }
                if visited[npos.y as usize][npos.x as usize] {
                    continue;
                }
                que.push_back(npos);
            }
        }
    }

    fn one_move(
        &self,
        pos: &mut Point,
        dir: &mut usize,
        vertexs: &mut Vec<Point>,
        first: &mut bool,
        target: Square,
    ) -> bool {
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        *dir = (*dir + 3) % 4;
        for d in 0..4 {
            let npos = Point::new(pos.x + dx[*dir], pos.y + dy[*dir]);
            if !*first && *pos == vertexs[0] && *dir == 0 {
                return true;
            }
            if !self.field.in_map(npos) || self.field[npos.y as usize][npos.x as usize] != target {
                if d == 1 || d == 2 {
                    let corner = match *dir {
                        0 => *pos + Point::new(1, 0),
                        1 => *pos + Point::new(1, 1),
                        2 => *pos + Point::new(0, 1),
                        3 => *pos,
                        _ => panic!(""),
                    };
                    vertexs.push(corner);
                }
                *dir = (*dir + 1) % 4;
                if d == 3 {
                    *vertexs = vec![
                        *pos,
                        *pos + Point::new(1, 0),
                        *pos + Point::new(1, 1),
                        *pos + Point::new(0, 1),
                        *pos,
                    ];
                    return true;
                }
                continue;
            }
            if d == 0 {
                let corner = match *dir {
                    0 => *pos + Point::new(1, 0),
                    1 => *pos + Point::new(1, 1),
                    2 => *pos + Point::new(0, 1),
                    3 => *pos,
                    _ => panic!(""),
                };
                vertexs.push(corner);
            }
            *pos = npos;
            *first = false;
            break;
        }
        return false;
    }
}

#[test]
fn test_puzzle_solve() {
    let puzzle = Puzzle {
        b_num: 0,
        e_num: 0,
        t_size: 8,

        v_min: 1,
        v_max: 300,
        booster_num: vec![0; 6],

        i_seq: vec![Point::new(5, 2), Point::new(1, 1), Point::new(3, 7)],
        o_seq: vec![Point::new(2, 1), Point::new(4, 2)],
    };
    let mut solver = PuzzleSolver::new(&puzzle);
    solver.solve();
    solver.field.print(0, 0, 40, 40);
    let map = solver.get_map();
    let obstacles = solver.get_obstacles();
    // println!("{:?}", map);
    // println!("{:?}", obstacles);
    let task = Task {
        map: map,
        point: Point::new(5, 2),
        obstacles: obstacles,
        boosters: vec![],
    };
    let field2 = Field::from(&task);
    field2.print(0, 0, 40, 40);
}

#[test]
fn test_puzzle_example_solve() {
    let s = "1,1,150,400,1200,6,10,5,1,3,4#(73,61),(49,125),(73,110),(98,49),(126,89),(68,102),(51,132),(101,123),(22,132),(71,120),(97,129),(118,76),(85,100),(88,22),(84,144),(93,110),(96,93),(113,138),(91,52),(27,128),(84,140),(93,143),(83,17),(123,85),(50,74),(139,97),(101,110),(77,56),(86,23),(117,59),(133,126),(83,135),(76,90),(70,12),(12,141),(116,87),(102,76),(19,138),(86,129),(86,128),(83,60),(100,98),(60,105),(61,103),(94,99),(130,124),(141,132),(68,84),(86,143),(72,119)#(145,82),(20,65),(138,99),(38,137),(85,8),(125,104),(117,48),(57,48),(64,119),(3,25),(40,22),(82,54),(121,119),(1,34),(43,98),(97,120),(10,90),(15,32),(41,13),(86,40),(3,83),(2,127),(4,40),(139,18),(96,49),(53,22),(5,103),(112,33),(38,47),(16,121),(133,99),(113,45),(50,5),(94,144),(16,0),(93,113),(18,141),(36,25),(56,120),(3,126),(143,144),(99,62),(144,117),(48,97),(69,9),(0,9),(141,16),(55,68),(81,3),(47,53)";
    let puzzle = Puzzle::from(&s);
    let mut solver = PuzzleSolver::new(&puzzle);
    solver.solve();
    solver.field.print(0, 0, 150, 150);
    println!("vertex: {}", solver.count_vertex());
    println!("{} < vertex < {}", puzzle.v_min, puzzle.v_max);
    println!("max_xy: {}", solver.max_xy());
    println!("{} < max_xy < {}", puzzle.xy_min(), puzzle.xy_max());
    println!("area: {}", solver.count_area());
    println!("{} < area", puzzle.area_min());
}
