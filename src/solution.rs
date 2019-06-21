use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::task::Point;

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
    pub fn from(s: &str) -> Self {
        let s = s.chars().collect::<Vec<char>>();

        let mut actions = vec![];
        let mut pos = 0;
        while pos < s.len() {
            let action = match s[pos] {
                'W' => Action::MoveUp,
                'S' => Action::MoveDown,
                'A' => Action::MoveLeft,
                'D' => Action::MoveRight,
                'Z' => Action::DoNothing,
                'E' => Action::TurnCW,
                'Q' => Action::TurnCCW,
                'B' => {
                    pos += 1;
                    let mut end = pos;
                    while s[end] != ')' {
                        end += 1;
                    }
                    let p = s[pos..end + 1].iter().collect::<String>();
                    let p = Point::from(&p);
                    pos = end;
                    Action::AttachManipulator { dx: p.x, dy: p.y }
                }
                'F' => Action::AttachFastWheels,
                'L' => Action::AttachDrill,
                _ => panic!("wrong character"),
            };
            pos += 1;
            actions.push(action);
        }

        return Solution(actions);
    }

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
    pub fn load(path: &Path) -> Solution{
        let mut f = fs::File::open(path).unwrap();
        let mut s = "".to_string();
        f.read_to_string(&mut s).unwrap();
        let ret = Solution::from(&s);
        ret
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
    assert!(s == "WSADZEQB(1,2)FL".to_string());
    let sol2 = Solution::from(&s);
    assert!(sol == sol2);
}
