#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum BoosterCode {
    ExtensionOfTheManipulator,
    FastWheels,
    Drill,
    MysteriousPoint,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BoosterLocation {
    pub code: BoosterCode,
    pub point: Point,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Task {
    pub map: Map,
    pub point: Point,
    pub obstacles: Vec<Map>,
    pub boosters: Vec<BoosterLocation>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Map(pub Vec<Point>);

impl Map {
    pub fn from(s: &str) -> Self {
        let mut points = vec![];
        let s = s.chars().collect::<Vec<char>>();

        let mut start = 0;
        let mut end = 0;
        while end < s.len() {
            if s[end] == ')' {
                let p = s[start..end + 1].iter().collect::<String>();
                points.push(Point::from(&p));
                end += 2;
                start = end;
            } else {
                end += 1;
            }
        }
        Map(points)
    }
}

impl Task {
    pub fn from(s: &str) -> Self {
        let s = s.split('#').collect::<Vec<_>>();
        let (map, point, obstacles, boosters) = (s[0], s[1], s[2], s[3]);
        let map = Map::from(map);
        let point = Point::from(point);
        let obstacles = obstacles.split(";").map(|o| Map::from(o)).collect();
        let boosters = boosters
            .split(';')
            .map(|b| BoosterLocation::from(b))
            .collect();
        Self {
            map,
            point,
            obstacles,
            boosters,
        }
    }
}

impl BoosterCode {
    pub fn from(s: &str) -> Self {
        match s {
            "B" => BoosterCode::ExtensionOfTheManipulator,
            "F" => BoosterCode::FastWheels,
            "L" => BoosterCode::Drill,
            "X" => BoosterCode::MysteriousPoint,
            _ => panic!("failed to parse BoosterCode"),
        }
    }
}

impl BoosterLocation {
    pub fn from(s: &str) -> Self {
        let code = BoosterCode::from(&s[0..1]);
        let point = Point::from(&s[1..]);
        Self { code, point }
    }

    pub fn new(code: BoosterCode, point: Point) -> Self {
        Self { code, point }
    }
}

impl Point {
    pub fn from(s: &str) -> Self {
        let s = s[1..s.len() - 1].split(',').collect::<Vec<_>>();
        let x = s[0].parse::<i32>().expect("failed to parse x");
        let y = s[1].parse::<i32>().expect("failed to parse y");
        Self { x, y }
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[test]
fn test_task_from() {
    let s = "(0,0),(10,0),(10,10),(0,10)#(0,0)#(4,2),(6,2),(6,7),(4,7);(5,8),(6,8),(6,9),(5,9)#B(0,1);B(1,1);F(0,2);F(1,2);L(0,3);X(0,9)";
    let task = Task::from(s);
    // TODO
}
