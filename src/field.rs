use crate::task::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Square {
    Surface,
    WrappedSurface,
    Obstacle,
    Booster { code: BoosterCode },
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Field(pub Vec<Vec<Square>>);

impl Field {
    pub fn from(task: &Task) -> Self {
        let Map(map) = &task.map;
        let x = map.iter().map(|p| p.x).max().unwrap() as usize;
        let y = map.iter().map(|p| p.y).max().unwrap() as usize;
        let mut field = vec![vec![Square::Surface; x]; y];

        {
            let mut prev = &map[0];
            let mut ps = map.clone();
            ps.push(map[0].clone());
            for p in &ps {
                if p.x > prev.x && p.y > 0 {
                    for x in prev.x..p.x {
                        field[(p.y - 1) as usize][x as usize] = Square::Obstacle;
                    }
                }
                if p.y > prev.y && p.x < x as i32 {
                    for y in prev.y..p.y {
                        field[y as usize][p.x as usize] = Square::Obstacle;
                    }
                }
                if p.x < prev.x && p.y < y as i32 {
                    for x in p.x..prev.x {
                        field[p.y as usize][x as usize] = Square::Obstacle;
                    }
                }
                if p.y < prev.y && p.x > 0 {
                    for y in p.y..prev.y {
                        field[y as usize][(p.x - 1) as usize] = Square::Obstacle;
                    }
                }
                prev = p;
            }
        }

        for b in &task.boosters {
            let y = b.point.y as usize;
            let x = b.point.x as usize;
            field[y][x] = Square::Booster {
                code: b.code.clone(),
            };
        }

        // TODO: fill obstacles inside ...
        for Map(points) in &task.obstacles {
            if points.len() < 2 {
                continue;
            }
            let mut prev = &points[0];
            let mut ps = points.clone();
            ps.push(points[0].clone());
            for p in &ps {
                if p.x > prev.x {
                    for x in prev.x..p.x {
                        field[p.y as usize][x as usize] = Square::Obstacle;
                    }
                }
                if p.y > prev.y {
                    for y in prev.y..p.y {
                        field[y as usize][(p.x - 1) as usize] = Square::Obstacle;
                    }
                }
                if p.x < prev.x {
                    for x in p.x..prev.x {
                        field[(p.y - 1) as usize][x as usize] = Square::Obstacle;
                    }
                }
                if p.y < prev.y {
                    for y in p.y..prev.y {
                        field[y as usize][p.x as usize] = Square::Obstacle;
                    }
                }
                prev = p;
            }
        }
        Field(field)
    }
}

#[test]
fn test_field_from() {
    // .X..
    // .**.
    // .F..

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

    let field = Field::from(&task);
    let expected = Field(vec![
        vec![
            Square::Surface,
            Square::Booster {
                code: BoosterCode::FastWheels,
            },
            Square::Surface,
            Square::Surface,
        ],
        vec![
            Square::Surface,
            Square::Obstacle,
            Square::Obstacle,
            Square::Surface,
        ],
        vec![
            Square::Surface,
            Square::Booster {
                code: BoosterCode::MysteriousPoint,
            },
            Square::Surface,
            Square::Surface,
        ],
    ]);
    assert_eq!(field, expected);
}