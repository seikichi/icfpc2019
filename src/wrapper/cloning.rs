use crate::field::*;
use crate::solution::*;
use crate::task::*;
use crate::wrapper::Wrapper;

use rand::prelude::*;

use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum GoalKind {
    GetCloningBooster,
    Cloning,
    Wrap,
    Rotate,
    RandomMove,
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
    fn random(l: usize) -> WorkerGoal {
        WorkerGoal {
            kind: GoalKind::RandomMove,
            p: Point::new(0, 0),
            actions: VecDeque::from_iter(vec![Action::DoNothing; l].iter().cloned()),
        }
    }
}

pub struct CloningWrapper {
    workers: Vec<Worker>,
    booster_cnts: Vec<usize>,
    field: Field,
    worker_goals: Vec<WorkerGoal>,
    next_turn_workers: Vec<Worker>, // Cloneされた直後のWorker、次のターンからworkersに入る
    rng: ThreadRng,
    random_move_ratio: usize,
}

impl Wrapper for CloningWrapper {
    fn wrap(&mut self, _task: &Task) -> Solution {
        let mut solution = vec![vec![]];
        while !self.field.is_finished() {
            for i in 0..self.workers.len() {
                self.one_worker_action(i, &mut solution);
            }
            for w in self.next_turn_workers.iter() {
                self.workers.push(w.clone());
                self.worker_goals.push(WorkerGoal::new(
                    GoalKind::Rotate,
                    Point::new(0, 0),
                    vec![Action::TurnCW],
                ));
                solution.push(vec![]);
            }
            self.next_turn_workers = vec![];
            // println!("{:?}", self.workers);
            // println!("{:?}", self.worker_goals);
            // self.field.print(0, 0, 40, 40);
        }
        return Solution(solution);
    }
}

impl CloningWrapper {
    pub fn new(task: &Task, random_move_ratio: usize) -> Self {
        let mut workers = vec![Worker::new(task.point)];
        let mut field = Field::from(task);
        let mut booster_cnts = vec![0; 10];
        field.update_surface(&mut workers[0], &mut booster_cnts);
        CloningWrapper {
            workers,
            booster_cnts,
            field,
            worker_goals: vec![WorkerGoal::nop()],
            next_turn_workers: vec![],
            rng: rand::thread_rng(),
            random_move_ratio: random_move_ratio,
        }
    }
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
        if self.worker_goals[index].kind == GoalKind::Cloning
            || self.worker_goals[index].kind == GoalKind::GetCloningBooster
        {
            return true;
        }
        let other_goal_cnt = self.worker_goals.iter().fold(0, |sum, goal| {
            sum + if goal.kind == GoalKind::GetCloningBooster {
                1
            } else {
                0
            }
        });
        return other_goal_cnt < self.field.rest_booster_cnts[BoosterCode::Cloning as usize];
    }

    fn one_worker_action(&mut self, index: usize, solution: &mut Vec<Vec<Action>>) {
        // ランダムな確率で今やる事を忘れてランダムムーブさせる
        if self.rng.gen::<usize>() % self.random_move_ratio == 0 {
            let l = self.rng.gen::<usize>() % 2 + 1;
            self.worker_goals[index] = WorkerGoal::random(l);
        }
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
            let mut lock = vec![];
            for i in 0..self.workers.len() {
                if i == index
                    || self.worker_goals[i].kind == GoalKind::Nothing
                    || self.worker_goals[i].kind == GoalKind::RandomMove
                {
                    continue;
                }
                lock.push(self.worker_goals[i].p);
            }
            if let Some((p, mut actions)) = self.field.bfs(&self.workers[index], target, &lock) {
                if kind == GoalKind::Cloning {
                    actions.push(Action::Cloning);
                }
                self.worker_goals[index] = WorkerGoal::new(kind, p, actions);
            } else {
                let r = self.rng.gen::<usize>() % 5 + 1;
                self.worker_goals[index] = WorkerGoal::random(r);
            }
        }
        // Action実行
        let mut action = self.worker_goals[index].actions.pop_front().unwrap();
        if self.worker_goals[index].kind == GoalKind::RandomMove {
            action = self.get_random_action(index);
        }
        self.workers[index].act(action, &mut self.field, &mut self.booster_cnts);
        if action == Action::Cloning {
            // Cloneの作成
            self.next_turn_workers
                .push(Worker::new(self.workers[index].p));
        }
        solution[index].push(action);
        if self.worker_goals[index].actions.len() == 0 {
            self.worker_goals[index] = WorkerGoal::nop();
        }
    }

    fn is_already_wrapped_goal(&self, index: usize) -> bool {
        let goal = &self.worker_goals[index];
        return goal.kind == GoalKind::Wrap
            && self.field[goal.p.y as usize][goal.p.x as usize] == Square::WrappedSurface;
    }

    fn get_random_action(&mut self, index: usize) -> Action {
        let candidate = vec![
            Action::MoveUp,
            Action::MoveDown,
            Action::MoveLeft,
            Action::MoveRight,
            Action::TurnCW,
            Action::TurnCCW,
        ];
        let r = self.rng.gen::<usize>() % candidate.len();
        let action = candidate[r];
        if !self.workers[index].can_act(action, &self.field, &self.booster_cnts) {
            return Action::DoNothing;
        }
        return action;
    }
}

#[test]
fn test_cloning_two_cloning() {
    // .X...
    // ..#..
    // C.#..
    // ..#..
    // s.C..

    let map = Map(vec![
        Point::new(0, 0),
        Point::new(5, 0),
        Point::new(5, 5),
        Point::new(0, 5),
    ]);
    let obstacles = vec![Map(vec![
        Point::new(2, 1),
        Point::new(3, 1),
        Point::new(3, 4),
        Point::new(2, 4),
    ])];
    let boosters = vec![
        BoosterLocation::new(BoosterCode::Cloning, Point::new(2, 0)),
        BoosterLocation::new(BoosterCode::Cloning, Point::new(0, 2)),
        BoosterLocation::new(BoosterCode::MysteriousPoint, Point::new(1, 4)),
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
