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

pub struct Solution(Vec<Action>);
