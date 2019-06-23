use crate::field::*;
use crate::solution::*;
use crate::task::*;
use crate::wrapper::Wrapper;

use rand::prelude::*;

use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum BigGoalKind {
    MoveToGrid,
    FillGrid,
    Nothing,
    Stop,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum GoalKind {
    GetCloningBooster,
    Cloning,
    GetBooster,
    UseManipulatorBooster,
    UseDrill,
    UseWheel,
    Wrap,
    Rotate,
    RandomMove,
    Nothing,
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct WorkerGoal {
    big_kind: BigGoalKind,
    kind: GoalKind,
    p: Point,
    actions: VecDeque<Action>,
    grid_id: Option<i32>,
}
impl WorkerGoal {
    fn new(kind: GoalKind, p: Point, actions: Vec<Action>, grid_id: i32) -> WorkerGoal {
        WorkerGoal {
            big_kind: BigGoalKind::FillGrid,
            kind,
            p,
            actions: VecDeque::from_iter(actions.into_iter()),
            grid_id: Some(grid_id),
        }
    }
    fn nop(grid_id: i32) -> WorkerGoal {
        WorkerGoal {
            big_kind: BigGoalKind::FillGrid,
            kind: GoalKind::Nothing,
            p: Point::new(0, 0),
            actions: VecDeque::from_iter([Action::DoNothing].iter().cloned()),
            grid_id: Some(grid_id),
        }
    }
    fn random(l: usize, grid_id: i32) -> WorkerGoal {
        WorkerGoal {
            big_kind: BigGoalKind::FillGrid,
            kind: GoalKind::RandomMove,
            p: Point::new(0, 0),
            actions: VecDeque::from_iter(vec![Action::DoNothing; l].iter().cloned()),
            grid_id: Some(grid_id),
        }
    }
    fn initialize() -> WorkerGoal {
        WorkerGoal {
            big_kind: BigGoalKind::Nothing,
            kind: GoalKind::Nothing,
            p: Point::new(0, 0),
            actions: VecDeque::from_iter(vec![Action::DoNothing].iter().cloned()),
            grid_id: None,
        }
    }
    fn stop() -> WorkerGoal {
        WorkerGoal {
            big_kind: BigGoalKind::Stop,
            kind: GoalKind::Nothing,
            p: Point::new(0, 0),
            actions: VecDeque::new(),
            grid_id: None,
        }
    }
    fn move_to_grid(p: Point, actions: Vec<Action>, grid_id: i32) -> WorkerGoal {
        WorkerGoal {
            big_kind: BigGoalKind::MoveToGrid,
            kind: GoalKind::Nothing,
            p,
            actions: VecDeque::from_iter(actions.into_iter()),
            grid_id: Some(grid_id),
        }
    }
}

pub struct CloningWrapper {
    task: Task,
    workers: Vec<Worker>,
    booster_cnts: Vec<usize>,
    field: Field,
    worker_goals: Vec<WorkerGoal>,
    next_turn_workers: Vec<Worker>, // Cloneされた直後のWorker、次のターンからworkersに入る
    rng: ThreadRng,
    random_move_ratio: usize,
    grids: Grids,
    rest_grid_ids: Vec<i32>,
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
                self.worker_goals.push(WorkerGoal::initialize());
                solution.push(vec![]);
            }
            self.next_turn_workers = vec![];
            // println!("{:?}", self.workers);
            // println!("{:?}", self.worker_goals);
            // self.field.print(0, 0, 40, 40);
        }
        eprintln!("{:?}", self.booster_cnts);
        return Solution(solution);
    }
}

impl CloningWrapper {
    pub fn new(task: &Task, boosters: &Vec<BoosterCode>, random_move_ratio: usize) -> Self {
        let mut workers = vec![Worker::new(task.point)];
        let mut field = Field::from(task);
        let mut booster_cnts = vec![0; 10];
        for b in boosters {
            booster_cnts[*b as usize] += 1;
        }
        let grid_num = 12;
        let mut grids = Grids::from(&field, grid_num);
        field.update_surface(&mut workers[0], &mut grids);
        CloningWrapper {
            task: task.clone(),
            workers,
            booster_cnts,
            field,
            worker_goals: vec![WorkerGoal::initialize()],
            next_turn_workers: vec![],
            rng: rand::thread_rng(),
            random_move_ratio: random_move_ratio,
            grids,
            rest_grid_ids: (0..(grid_num as i32)).collect(),
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

    fn should_get_booster(&self, index: usize) -> Option<Point> {
        for booster_location in self.task.boosters.iter() {
            let p = booster_location.point;
            if booster_location.code == BoosterCode::Cloning
                || booster_location.code == BoosterCode::MysteriousPoint
                || self.field.get_booster_square(p) == Square::Unknown
                || (p - self.workers[index].p).manhattan_dist() > 2
                || self.is_locked(p, index)
            {
                continue;
            }
            return Some(p);
        }
        return None;
    }

    fn pop_grid_id(&mut self) -> Option<i32> {
        // TODO
        // - 近いやつにする
        // - pop じゃなくて flag を持たせて最後協力して grid を複数 worker で倒す
        self.rest_grid_ids.pop()
    }

    fn one_worker_action(&mut self, index: usize, solution: &mut Vec<Vec<Action>>) {
        self.field
            .get_booster(&mut self.workers[index], &mut self.booster_cnts);

        // 大目標を見て...
        // Nothing -> Grid を決める
        // MoveToGrid -> Grid への移動を決めて終わり (BFS)
        // FillGrid -> 以下の処理
        if self.worker_goals[index].big_kind == BigGoalKind::Nothing {
            eprintln!("Nothing");
            match self.pop_grid_id() {
                None => self.worker_goals[index] = WorkerGoal::stop(),
                Some(grid_id) => {
                    eprintln!("move to grid: {:?}", grid_id);
                    if let Some((p, actions)) = self.field.bfs(
                        &self.workers[index],
                        Square::Surface,
                        Point::new(-1, -1),
                        &vec![],
                        Some(&self.grids),
                        Some(grid_id),
                    ) {
                        eprintln!("{:?}", actions);
                        self.worker_goals[index] = WorkerGoal::move_to_grid(p, actions, grid_id);
                    } else {
                        panic!("Faild to move grid");
                    }
                }
            }
        }
        if self.worker_goals[index].big_kind == BigGoalKind::Stop {
            eprintln!("Stop");
            solution[index].push(Action::DoNothing);
            return;
        }
        if self.worker_goals[index].big_kind == BigGoalKind::MoveToGrid {
            eprintln!("MoveToGrid");
            let action = self.worker_goals[index].actions.pop_front().unwrap();
            self.workers[index].act(
                action,
                &mut self.field,
                &mut self.booster_cnts,
                &mut self.grids,
            );
            solution[index].push(action);
            if self.worker_goals[index].actions.len() == 0 {
                let l = self.rng.gen::<usize>() % 2 + 1;
                self.worker_goals[index] =
                    WorkerGoal::random(l, self.worker_goals[index].grid_id.unwrap());
            }
            return;
        }
        eprintln!("FillGrid");
        let grid_id = self.worker_goals[index].grid_id.unwrap();
        if self.grids.is_finished(grid_id) {
            eprintln!("grids.is_finished()");
            self.worker_goals[index] = WorkerGoal::initialize();
            return;
        }
        // ランダムな確率で今やる事を忘れてランダムムーブさせる
        if self.rng.gen::<usize>() % self.random_move_ratio == 0 {
            let l = self.rng.gen::<usize>() % 2 + 1;
            self.worker_goals[index] = WorkerGoal::random(l, grid_id);
        }
        // 塗ろうとして行っている箇所がすでに塗られていたら考え直す
        if self.is_already_wrapped_goal(index) {
            self.worker_goals[index] = WorkerGoal::nop(grid_id);
        }
        // GoalとActionを決める
        if self.worker_goals[index].kind == GoalKind::Nothing {
            self.decide_goal_and_action(index);
        }
        // Action実行
        let mut action = self.worker_goals[index].actions.pop_front().unwrap();
        if self.worker_goals[index].kind == GoalKind::RandomMove {
            action = self.get_random_action(index);
        }
        self.workers[index].act(
            action,
            &mut self.field,
            &mut self.booster_cnts,
            &mut self.grids,
        );
        if action == Action::Cloning {
            // Cloneの作成
            self.next_turn_workers
                .push(Worker::new(self.workers[index].p));
        }
        solution[index].push(action);
        if self.worker_goals[index].actions.len() == 0 {
            self.worker_goals[index] = WorkerGoal::nop(grid_id);
        }
    }

    // goalを決めてアクション列を作る
    fn decide_goal_and_action(&mut self, index: usize) {
        let grid_id = self.worker_goals[index].grid_id.unwrap();

        let (kind, target, target_point) = if self.should_cloning(index) {
            (
                GoalKind::Cloning,
                Square::Booster {
                    code: BoosterCode::MysteriousPoint,
                },
                Point::new(-1, -1),
            )
        } else if self.should_get_cloning_booster(index) {
            (
                GoalKind::GetCloningBooster,
                Square::Booster {
                    code: BoosterCode::Cloning,
                },
                Point::new(-1, -1),
            )
        } else if index == 0
            && self.booster_cnts[BoosterCode::ExtensionOfTheManipulator as usize] > 0
        {
            (
                GoalKind::UseManipulatorBooster,
                Square::Unknown,
                Point::new(-1, -1),
            )
        } else if let Some(p) = self.should_get_booster(index) {
            (GoalKind::GetBooster, Square::Unknown, p)
        } else {
            (GoalKind::Wrap, Square::Surface, Point::new(-1, -1))
        };

        if kind == GoalKind::UseManipulatorBooster {
            let dy = self.workers[index].manipulators.len() - 2;
            let p = Point::new(0, dy as i32).rotate(self.workers[index].cw_rotation_count);
            let actions = vec![Action::AttachManipulator { dx: p.x, dy: p.y }];
            self.worker_goals[index] = WorkerGoal::new(kind, target_point, actions, grid_id);
            return;
        }
        let lock = self.get_lock(index);
        if let Some((p, mut actions)) = self.field.bfs(
            &self.workers[index],
            target,
            target_point,
            &lock,
            Some(&self.grids),
            self.worker_goals[index].grid_id,
        ) {
            if kind == GoalKind::Cloning {
                actions.push(Action::Cloning);
            }
            self.worker_goals[index] = WorkerGoal::new(kind, p, actions, grid_id);
        } else {
            let r = self.rng.gen::<usize>() % 5 + 1;
            self.worker_goals[index] = WorkerGoal::random(r, grid_id);
        }

        // WheelとDrillのチェック
        // 使う場合は終点のロックをとったままgoalを書き換える
        // 次のターンにactionsが空になるので直ぐにGoalが再計算される
        let action_cnt = self.worker_goals[index].actions.len();
        let r = self.rng.gen::<usize>() % 2;
        if action_cnt > 40 && r == 0 {
            let target_p = self.worker_goals[index].p;
            if self.booster_cnts[BoosterCode::Drill as usize] > 0
                && self.workers[index].drill_time <= 0
                && ((self.workers[index].p - self.worker_goals[index].p).manhattan_dist() as f64
                    / action_cnt as f64)
                    < 0.7f64
            {
                // マンハッタン距離は近いのにアクションの系列が長い場合はドリルを使うことにする
                self.worker_goals[index] = WorkerGoal::new(
                    GoalKind::UseDrill,
                    target_p,
                    vec![Action::AttachDrill],
                    grid_id,
                );
            } else if self.booster_cnts[BoosterCode::FastWheels as usize] > 0
                && self.workers[index].fast_time <= 0
            {
                // 純粋に遠い場合はFastWheelを使う
                self.worker_goals[index] = WorkerGoal::new(
                    GoalKind::UseWheel,
                    target_p,
                    vec![Action::AttachFastWheels],
                    grid_id,
                );
            }
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
    fn is_locked(&self, p: Point, index: usize) -> bool {
        let lock = self.get_lock(index);
        for &np in lock.iter() {
            if np == p {
                return true;
            }
        }
        return false;
    }
    fn get_lock(&self, index: usize) -> Vec<Point> {
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
        return lock;
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

    let mut wrapper = CloningWrapper::new(&task, &vec![], 1 << 30);
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

    let mut wrapper = CloningWrapper::new(&task, &vec![], 1 << 30);
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

    let mut wrapper = CloningWrapper::new(&task, &vec![], 1 << 30);
    let _solution = wrapper.wrap(&task);
}
