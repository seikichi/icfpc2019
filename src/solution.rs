use std::fs;
// use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    DoNothing,
    TurnCW,
    TurnCCW,
    AttachManipulator { dx: i32, dy: i32 },
    AttachFastWheels,
    AttachDrill,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Solution(pub Vec<Action>);

impl Solution {
    pub fn to_string(&self) -> String {
        let Solution(actions) = self;
        let mut ret = "".to_string();
        for action in actions.iter() {
            let s = match action {
                Action::MoveUp => "W".to_string(),
                Action::MoveDown => "S".to_string(),
                Action::MoveLeft => "A".to_string(),
                Action::MoveRight => "D".to_string(),
                Action::DoNothing => "Z".to_string(),
                Action::TurnCW => "E".to_string(),
                Action::TurnCCW => "Q".to_string(),
                Action::AttachManipulator { dx, dy } => format!("B({},{})", dx, dy),
                Action::AttachFastWheels => "F".to_string(),
                Action::AttachDrill => "L".to_string(),
            };
            ret.push_str(&s);
        }
        return ret;
    }
    pub fn save(&self, path: &Path) {
        let s = self.to_string();
        let mut buffer = fs::File::create(path).unwrap();
        buffer.write_fmt(format_args!("{}", s)).unwrap();
    }
}

#[test]
fn solution_to_string_test() {
    let sol = Solution(vec![
        Action::MoveUp,
        Action::MoveDown,
        Action::MoveLeft,
        Action::MoveRight,
        Action::DoNothing,
        Action::TurnCW,
        Action::TurnCCW,
        Action::AttachManipulator { dx: 1, dy: 2 },
        Action::AttachFastWheels,
        Action::AttachDrill,
    ]);
    let s = sol.to_string();
    println!("{}", s);
    assert!(s == "WSADZEQB(1,2)FL".to_string());
}
