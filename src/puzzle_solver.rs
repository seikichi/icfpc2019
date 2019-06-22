
use crate::field::*;
use crate::puzzle::*;
use crate::task::*;
pub struct PuzzleSolver {
    puzzle: Puzzle,
    field: Field,
}

impl PuzzleSolver {
    pub fn new(puzzle: &Puzzle) -> Self {
        let max_x = puzzle.max_x() as usize;
        let max_y = puzzle.max_y() as usize;
        let mut field = Field {
            field: vec![vec![Square::Unknown; max_x + 50]; max_y + 50],
            booster_field: vec![vec![Square::Unknown; max_x + 50]; max_y + 50],
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
            // current: current,
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
        //
    }

    pub fn get_map(&self) -> Map {
        let mut pos = Point::new(0, 0);
        'yloop: for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                if self.field[y][x] == Square::WrappedSurface {
                    pos = Point::new(x as i32, y as i32);
                    break 'yloop;
                }
            }
        }
        let mut vertexs = vec![pos];
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        let mut dir = 0;
        let mut first = true;
        'outer_loop: loop {
            dir = (dir + 3) % 4;
            for d in 0..4 {
                let npos = Point::new(pos.x + dx[dir], pos.y + dy[dir]);
                if !first && pos == vertexs[0] && dir == 0 {
                    break 'outer_loop;
                }
                if !self.field.in_map(npos)
                    || self.field[npos.y as usize][npos.x as usize] != Square::WrappedSurface
                {
                    if d == 1 || d == 2 {
                        let corner = match dir {
                            0 => pos + Point::new(1, 0),
                            1 => pos + Point::new(1, 1),
                            2 => pos + Point::new(0, 1),
                            3 => pos,
                            _ => panic!(""),
                        };
                        vertexs.push(corner);
                    }
                    dir = (dir + 1) % 4;
                    continue;
                }
                if d == 0 {
                    let corner = match dir {
                        0 => pos + Point::new(1, 0),
                        1 => pos + Point::new(1, 1),
                        2 => pos + Point::new(0, 1),
                        3 => pos,
                        _ => panic!(""),
                    };
                    vertexs.push(corner);
                }
                pos = npos;
                first = false;
                break;
            }
        }
        vertexs.pop();
        return Map(vertexs);
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
    // solver.field.print(0, 0, 150, 150);
}