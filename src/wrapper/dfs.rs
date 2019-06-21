use crate::solution::*;
use crate::task::*;
use crate::wrapper::Wrapper;

pub struct DfsWrapper {
    field: Field,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Square {
    Surface,
    WrappedSurface,
    Obstacle,
    Booster { code: BoosterCode },
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Field(pub Vec<Vec<Square>>);

impl Field {
    fn from(task: &Task) -> Self {
        let Map(map) = &task.map;
        let x = map.iter().map(|p| p.x).max().unwrap() as usize;
        let y = map.iter().map(|p| p.y).max().unwrap() as usize;
        let mut field = vec![vec![Square::Surface; x]; y];
        Field(field)
    }
}

impl Wrapper for DfsWrapper {
    fn wrap(&mut self, task: &Task) -> Solution {
        let mut solution = vec![];
        let mut field = Field::from(task);
        while let Some(Solution(s)) = self.dfs(&mut field) {
            solution.extend(s);
        }
        Solution(solution)
    }
}

impl DfsWrapper {
    fn dfs(&mut self, field: &mut Field) -> Option<Solution> {
        None
    }
}

#[test]
fn test_field_from() {
    //     let map = Map(vec![
    //         Point::new(0, 0),
    //         Point::new(5, 0),
    //         Point::new(5, 4),
    //         Point::new(0, 4),
    //     ]);
    //     let obstacles = vec![
    //         Map(vec![
    //             Point::new(0, 2),
    //             Point::new(1, 2),
    //             Point::new(1, 4),
    //             Point::new(0, 4),
    //         ]),
    //         Map(vec![
    //             Point::new(2, 0),
    //             Point::new(4, 0),
    //             Point::new(4, 1),
    //             Point::new(3, 1),
    //             Point::new(3, 2),
    //             Point::new(4, 2),
    //             Point::new(4, 3),
    //             Point::new(2, 3),
    //         ]),
    //     ];
    //     let boosters = vec![
    //         BoosterLocation::new(BoosterCode::FastWheels, Point::new(1, 1)),
    //         BoosterLocation::new(BoosterCode::MysteriousPoint, Point::new(3, 1)),
    //     ];
    //     let point = Point::new(0, 0);
    //     let task = Task {
    //         point,
    //         map,
    //         obstacles,
    //         boosters,
    //     };

    //     let field = Field::from(&task);
    //     let expected = Field(vec![
    //         vec![
    //             Square::Surface,
    //             Square::Surface,
    //             Square::Obstacle,
    //             Square::Obstacle,
    //             Square::Surface,
    //         ],
    //         vec![
    //             Square::Surface,
    //             Square::Booster {
    //                 code: BoosterCode::FastWheels,
    //             },
    //             Square::Obstacle,
    //             Square::Booster {
    //                 code: BoosterCode::MysteriousPoint,
    //             },
    //             Square::Surface,
    //         ],
    //         vec![
    //             Square::Obstacle,
    //             Square::Surface,
    //             Square::Obstacle,
    //             Square::Obstacle,
    //             Square::Surface,
    //         ],
    //         vec![
    //             Square::Obstacle,
    //             Square::Surface,
    //             Square::Surface,
    //             Square::Surface,
    //             Square::Surface,
    //         ],
    //     ]);
    //     assert_eq!(field, expected);
}
