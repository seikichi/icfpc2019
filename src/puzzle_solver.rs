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
        let max_x = puzzle.max_x() as usize;
        let max_y = puzzle.max_y() as usize;
        let mut field = Field {
            field: vec![vec![Square::Unknown; max_x + 100]; max_y + 100],
            booster_field: vec![vec![Square::Unknown; max_x + 100]; max_y + 100],
            rest_booster_cnts: vec![0; 10],
            rest_surface_cnt: 1 << 30,
        };
        for &p in puzzle.i_seq.iter() {
            field[p.y as usize][p.x as usize] = Square::Surface;
        }
        for &p in puzzle.o_seq.iter() {
            field[p.y as usize][p.x as usize] = Square::Obstacle;
        }
        // let current = vec![vec![Square::Unknown; field.width()]; field.height()];
        PuzzleSolver {
            puzzle: puzzle.clone(),
            field: field,
            start_point: Point::new(0, 0),
            boosters: vec![],
        }
    }

    pub fn solve(&mut self) {
        let start_pos = self.puzzle.i_seq[0];
        self.field[start_pos.y as usize][start_pos.x as usize] = Square::WrappedSurface;
        for i in 1..self.puzzle.i_seq.len() {
            let mut worker = Worker::new(start_pos);
            worker.manipulators = vec![];
            let (_end_pos, actions) = self.field.bfs(&worker, Square::Surface, &vec![]).unwrap();
            for &action in actions.iter() {
                worker.act(action, &mut self.field, &mut vec![0; 10]);
            }
        }
        let mut rng = rand::thread_rng();
        let initial_field = self.field.clone();
        loop {
            let need_vertex = self.puzzle.v_min as i32 - self.count_vertex() as i32;
            let need_area = self.puzzle.area_min() as i32 - self.count_area() as i32;
            if need_vertex <= 0 && need_area <= 0 {
                break;
            }
            let mut first = true;
            let mut pos = self.get_left_bottom_position();
            let mut vertexs = vec![pos];
            let mut dir = 0;
            let cnt = need_vertex + 20;
            for _iter in 0..cnt {
                let move_cnt = rng.gen::<usize>() % 10 + 3;
                for _iter2 in 0..move_cnt {
                    self.one_move(&mut pos, &mut dir, &mut vertexs, &mut first);
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
        if self.field[pos2.y as usize][pos2.x as usize] == Square::WrappedSurface {
            return false;
        }
        for d1 in -1..2 {
            if d1 == 0 {
                continue;
            }
            let ndir = (dir as i32 + 4 + d1) as usize % 4;
            let npos = pos + Point::new(dx[ndir], dy[ndir]);
            if self.field[npos.y as usize][npos.x as usize] == Square::WrappedSurface {
                return false;
            }
            let npos = pos2 + Point::new(dx[ndir], dy[ndir]);
            if self.field[npos.y as usize][npos.x as usize] == Square::WrappedSurface {
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

    pub fn get_map(&self) -> Map {
        let mut pos = self.get_left_bottom_position();
        let mut vertexs = vec![pos];
        let mut dir = 0;
        let mut first = true;
        loop {
            if self.one_move(&mut pos, &mut dir, &mut vertexs, &mut first) {
                break;
            }
        }
        vertexs.pop();
        return Map(vertexs);
    }

    fn one_move(
        &self,
        pos: &mut Point,
        dir: &mut usize,
        vertexs: &mut Vec<Point>,
        first: &mut bool,
    ) -> bool {
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        *dir = (*dir + 3) % 4;
        for d in 0..4 {
            let npos = Point::new(pos.x + dx[*dir], pos.y + dy[*dir]);
            if !*first && *pos == vertexs[0] && *dir == 0 {
                return true;
            }
            if !self.field.in_map(npos)
                || self.field[npos.y as usize][npos.x as usize] != Square::WrappedSurface
            {
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
        t_size: 10,

        v_min: 10,
        v_max: 100,
        booster_num: vec![0; 10],

        i_seq: vec![Point::new(5, 2), Point::new(1, 1), Point::new(3, 7)],
        o_seq: vec![Point::new(2, 1), Point::new(3, 2)],
    };
    let mut solver = PuzzleSolver::new(&puzzle);
    solver.solve();
    solver.field.print(0, 0, 9, 9);
    let map = solver.get_map();
    // println!("{:?}", map);
    let task = Task {
        map: map,
        point: Point::new(5, 2),
        obstacles: vec![],
        boosters: vec![],
    };
    let field2 = Field::from(&task);
    field2.print(0, 0, 9, 9);
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
    println!("area: {}", solver.count_area());
    println!("{} < area < {}", puzzle.area_min(), puzzle.area_max());
}
