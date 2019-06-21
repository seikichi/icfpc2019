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
    fn to_string(&self) -> String {
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
                Action::AttachManipulator { dx, dy } => format!("B({},{}))", dx, dy),
                Action::AttachFastWheels => "F".to_string(),
                Action::AttachDrill => "L".to_string(),
            };
            ret.push_str(&s);
        }
        return ret;
    }
}
