use crate::field::*;
use crate::solution::*;
use crate::task::*;
use crate::wrapper::Wrapper;

use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum GoalKind {
    GetCloningBooster,
    Cloning,
    Wrap,
    _RandomMove,
    Nothing,
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct WorkerGoal {
    kind: GoalKind,
    p: Point,
    actions: VecDeque<Action>,
}
impl WorkerGoal {
    fn new(kind: GoalKind, p: Point, actions: Vec<Action>) -> WorkerGoal {
        WorkerGoal {
            kind,
            p,
            actions: VecDeque::from_iter(actions.into_iter()),
        }
    }
    fn nop() -> WorkerGoal {
        WorkerGoal {
            kind: GoalKind::Nothing,
            p: Point::new(0, 0),
            actions: VecDeque::from_iter([Action::DoNothing].iter().cloned()),
        }
    }
}

pub struct CloningWrapper {
    workers: Vec<Worker>,
    booster_cnts: Vec<usize>,
    field: Field,
    worker_goals: Vec<WorkerGoal>,
    next_turn_workers: Vec<Worker>, // Cloneされた直後のWorker、次のターンからworkersに入る
}

impl Wrapper for CloningWrapper {
    fn new(task: &Task) -> Self {
        let workers = vec![Worker::new(task.point)];
        let field = Field::from(task);
        let booster_cnts = vec![0; 10];
        CloningWrapper {
            workers,
            booster_cnts,
            field,
            worker_goals: vec![WorkerGoal::nop()],
            next_turn_workers: vec![],
        }
    }
    fn wrap(&mut self, _task: &Task) -> Solution {
        let mut solution = vec![vec![]];
        while !self.field.is_finished() {
            for i in 0..self.workers.len() {
                self.one_worker_action(i, &mut solution);
            }
            for w in self.next_turn_workers.iter() {
                self.workers.push(w.clone());
                self.worker_goals.push(WorkerGoal::nop());
                solution.push(vec![]);
            }
            self.next_turn_workers = vec![];
            // println!("{:?}", self.workers);
            // println!("{:?}", self.worker_goals);
        }
        return Solution(solution);
    }
}

impl CloningWrapper {
    // cloning boosterがあって他の人がcloningしようとしてなければやるべき
    fn should_cloning(&self, index: usize) -> bool {
        if self.field.rest_booster_cnts[BoosterCode::MysteriousPoint as usize] == 0
            || self.booster_cnts[BoosterCode::Cloning as usize] == 0
        {
            return false;
        }
        if self.worker_goals[index].kind == GoalKind::Cloning {
            return true;
        }
        for i in 0..self.workers.len() {
            if i == index {
                continue;
            }
            if self.worker_goals[i].kind == GoalKind::Cloning {
                return false;
            }
        }
        return true;
    }
    // cloning boosterがfieldにあって他の人が取ろうとしている数よりもまだ多かったら取りに行く
    fn should_get_cloning_booster(&self, index: usize) -> bool {
        if self.field.rest_booster_cnts[BoosterCode::MysteriousPoint as usize] == 0
            || self.field.rest_booster_cnts[BoosterCode::Cloning as usize] == 0
        {
            return false;
        }
        if self.worker_goals[index].kind == GoalKind::Cloning {
            return true;
        }
        let other_goal_cnt = self.worker_goals.iter().fold(0, |sum, goal| {
            sum + if goal.kind == GoalKind::Cloning { 1 } else { 0 }
        });
        return other_goal_cnt < self.field.rest_booster_cnts[BoosterCode::Cloning as usize];
    }

    fn one_worker_action(&mut self, index: usize, solution: &mut Vec<Vec<Action>>) {
        // 塗ろうとして行っている箇所がすでに塗られていたら考え直す
        if self.is_already_wrapped_goal(index) {
            self.worker_goals[index] = WorkerGoal::nop();
        }
        if self.worker_goals[index].kind == GoalKind::Nothing {
            // goalを決めてアクション列を作る
            let (kind, target) = if self.should_cloning(index) {
                (
                    GoalKind::Cloning,
                    Square::Booster {
                        code: BoosterCode::MysteriousPoint,
                    },
                )
            } else if self.should_get_cloning_booster(index) {
                (
                    GoalKind::GetCloningBooster,
                    Square::Booster {
                        code: BoosterCode::Cloning,
                    },
                )
            } else {
                (GoalKind::Wrap, Square::Surface)
            };
            if let Some((p, mut actions)) =
                CloningWrapper::bfs(&self.field, &self.workers[index], target, &vec![])
            {
                if kind == GoalKind::Cloning {
                    actions.push(Action::Cloning);
                }
                self.worker_goals[index] = WorkerGoal::new(kind, p, actions);
            }
        }
        // Action実行
        let action = self.worker_goals[index].actions[0];
        self.workers[index].act(action, &mut self.field, &mut self.booster_cnts);
        if action == Action::Cloning {
            // Cloneの作成
            self.next_turn_workers
                .push(Worker::new(self.workers[index].p));
        }
        solution[index].push(action);
        self.worker_goals[index].actions.pop_front();
        if self.worker_goals[index].actions.len() == 0 {
            self.worker_goals[index] = WorkerGoal::nop();
        }
    }

    fn is_already_wrapped_goal(&self, index: usize) -> bool {
        let goal = &self.worker_goals[index];
        return goal.kind == GoalKind::Wrap
            && self.field[goal.p.y as usize][goal.p.x as usize] == Square::WrappedSurface;
    }

    // TODO lockをちゃんと実装する
    // lockはそのsquareがtargetであっても無視する
    fn bfs(
        field: &Field,
        worker: &Worker,
        target: Square,
        lock: &Vec<Point>,
    ) -> Option<(Point, Vec<Action>)> {
        let w = field.width();
        let h = field.height();

        let mut current = worker.p;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((current, 0));

        let mut visited = vec![vec![-1; w]; h];

        while let Some((p, cost)) = queue.pop_front() {
            let y = p.y as usize;
            let x = p.x as usize;
            if visited[y][x] != -1 {
                continue;
            }
            visited[y][x] = cost;
            if field[y][x] == target || field.booster_field[y][x] == target {
                let end_p = Point::new(x as i32, y as i32);
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
                if actions.is_empty() {
                    actions.push(Action::DoNothing);
                }
                return Some((end_p, actions));
            }

            let ns = [
                (p.y - 1, p.x),
                (p.y + 1, p.x),
                (p.y, p.x - 1),
                (p.y, p.x + 1),
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
fn test_cloning_nocloning() {
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

    let mut wrapper = CloningWrapper::new(&task);
    let _solution = wrapper.wrap(&task);
}

#[test]
fn test_cloning_with_cloning() {
    // .X..
    // C**.
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
        BoosterLocation::new(BoosterCode::Cloning, Point::new(0, 1)),
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

    let mut wrapper = CloningWrapper::new(&task);
    let _solution = wrapper.wrap(&task);
}