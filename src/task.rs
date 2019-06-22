use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum BoosterCode {
    ExtensionOfTheManipulator,
    FastWheels,
    Drill,
    MysteriousPoint,
    Teleport,
    Cloning,
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

        let mut pos = 0;
        while pos < s.len() {
            if s[pos] == '(' {
                let (p, end) = Point::read(&s, pos);
                points.push(p);
                pos = end;
            } else {
                pos += 1;
            }
        }
        Map(points)
    }
    pub fn to_string(&self) -> String {
        let mut ret = "".to_string();
        let Map(map) = self;
        for (i, point) in map.iter().enumerate() {
            ret.push_str(&point.to_string());
            if i != map.len() - 1 {
                ret.push(',');
            }
        }
        return ret;
    }
}

impl Task {
    pub fn from(s: &str) -> Self {
        let s = s.trim_end().split('#').collect::<Vec<_>>();
        let (map, point, obstacles, boosters) = (s[0], s[1], s[2], s[3]);
        let map = Map::from(map);
        let point = Point::from(point);
        let obstacles = obstacles
            .split(";")
            .filter(|b| !b.is_empty())
            .map(|o| Map::from(o))
            .collect();
        let boosters = boosters
            .split(';')
            .filter(|b| !b.is_empty())
            .map(|b| BoosterLocation::from(b))
            .collect();
        Self {
            map,
            point,
            obstacles,
            boosters,
        }
    }
    pub fn load(path: &Path) -> Self {
        let mut f = fs::File::open(path).unwrap();
        let mut s = "".to_string();
        f.read_to_string(&mut s).unwrap();
        let ret = Task::from(&s);
        ret
    }
    pub fn to_string(&self) -> String {
        let mut ret = "".to_string();
        ret.push_str(&self.map.to_string());
        ret.push('#');
        ret.push_str(&self.point.to_string());
        ret.push('#');

        for (i, map) in self.obstacles.iter().enumerate() {
            ret.push_str(&map.to_string());
            if i != self.obstacles.len() - 1 {
                ret.push(';');
            }
        }
        ret.push('#');
        for (i, booster) in self.boosters.iter().enumerate() {
            ret.push_str(&booster.to_string());
            if i != self.boosters.len() - 1 {
                ret.push(';');
            }
        }
        return ret;
    }
}

impl BoosterCode {
    pub fn from(s: &str) -> Self {
        match s {
            "B" => BoosterCode::ExtensionOfTheManipulator,
            "F" => BoosterCode::FastWheels,
            "L" => BoosterCode::Drill,
            "X" => BoosterCode::MysteriousPoint,
            "R" => BoosterCode::Teleport,
            "C" => BoosterCode::Cloning,
            _ => panic!("failed to parse BoosterCode"),
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            BoosterCode::ExtensionOfTheManipulator => "B",
            BoosterCode::FastWheels => "F",
            BoosterCode::Drill => "L",
            BoosterCode::MysteriousPoint => "X",
            BoosterCode::Teleport => "R",
            BoosterCode::Cloning => "C",
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

    pub fn to_string(&self) -> String {
        let mut ret = "".to_string();
        ret.push_str(self.code.symbol());
        ret.push_str(&self.point.to_string());
        ret
    }
}

impl Point {
    pub fn from(s: &str) -> Self {
        let s = s[1..s.len() - 1].split(',').collect::<Vec<_>>();
        let x = s[0].parse::<i32>().expect("failed to parse x");
        let y = s[1].parse::<i32>().expect("failed to parse y");
        Self { x, y }
    }
    // startからPointを読み込んで、Pointと読み込み後の)の位置を返す
    pub fn read(s: &Vec<char>, start: usize) -> (Self, usize) {
        let mut end = start;
        while s[end] != ')' {
            end += 1;
        }
        let p = s[start..end + 1].iter().collect::<String>();
        let p = Point::from(&p);
        (p, end)
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn rotate(self, cw_rotation_count: i32) -> Self {
        let cw_rotation_count = (cw_rotation_count + (1 << 20)) % 4;
        return match cw_rotation_count {
            0 => self,
            1 => Point::new(self.y, -self.x),
            2 => Point::new(-self.x, -self.y),
            3 => Point::new(-self.y, self.x),
            _ => panic!("unknown value"),
        };
    }
    pub fn manhattan_dist(self) -> i32 {
        self.x + self.y
    }
    pub fn to_string(&self) -> String {
        let mut ret = "".to_string();
        ret.push('(');
        ret.push_str(&self.x.to_string());
        ret.push(',');
        ret.push_str(&self.y.to_string());
        ret.push(')');
        return ret;
    }
}
impl std::ops::Add<Point> for Point {
    type Output = Point;
    #[inline]
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Sub<Point> for Point {
    type Output = Point;
    #[inline]
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl std::ops::AddAssign<Point> for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl std::ops::SubAssign<Point> for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl std::ops::Mul<i32> for Point {
    type Output = Point;
    #[inline]
    fn mul(self, rhs: i32) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[test]
fn test_task_from() {
    let s = "(0,0),(10,0),(10,10),(0,10)#(0,0)#(4,2),(6,2),(6,7),(4,7);(5,8),(6,8),(6,9),(5,9)#B(0,1);B(1,1);F(0,2);F(1,2);L(0,3);X(0,9)";
    let _task = Task::from(s);
    // TODO
}

#[test]
fn test_point_rotate() {
    let p1 = Point::new(10, 1);
    assert!(p1.rotate(0) == Point::new(10, 1));
    assert!(p1.rotate(1) == Point::new(1, -10));
    assert!(p1.rotate(2) == Point::new(-10, -1));
    assert!(p1.rotate(3) == Point::new(-1, 10));

    assert!(p1.rotate(-12) == Point::new(10, 1));
}

#[test]
fn test_point_ops() {
    let p1 = Point::new(10, 20);
    let p2 = Point::new(1, 2);
    assert!(p1 + p2 == Point::new(11, 22));
    assert!(p1 - p2 == Point::new(9, 18));
    assert!(p1 * 2 == Point::new(20, 40));

    let mut p1 = Point::new(10, 20);
    p1 += p2;
    assert!(p1 == Point::new(11, 22));

    let mut p1 = Point::new(10, 20);
    p1 -= p2;
    assert!(p1 == Point::new(9, 18));
}

#[test]
fn test_task_to_string() {
    let map = Map(vec![
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ]);
    let point = Point::new(0, 0);
    let obstacles = vec![
        Map(vec![
            Point::new(4, 2),
            Point::new(6, 2),
            Point::new(6, 7),
            Point::new(4, 7),
        ]),
        Map(vec![
            Point::new(5, 8),
            Point::new(6, 8),
            Point::new(6, 9),
            Point::new(5, 9),
        ]),
    ];
    let boosters = vec![
        BoosterLocation::new(BoosterCode::ExtensionOfTheManipulator, Point::new(0, 1)),
        BoosterLocation::new(BoosterCode::ExtensionOfTheManipulator, Point::new(1, 1)),
        BoosterLocation::new(BoosterCode::FastWheels, Point::new(0, 2)),
        BoosterLocation::new(BoosterCode::FastWheels, Point::new(1, 2)),
        BoosterLocation::new(BoosterCode::Drill, Point::new(0, 3)),
        BoosterLocation::new(BoosterCode::MysteriousPoint, Point::new(0, 9)),
        BoosterLocation::new(BoosterCode::Teleport, Point::new(3, 9)),
        BoosterLocation::new(BoosterCode::Cloning, Point::new(1, 9)),
    ];
    let task = Task {
        map: map,
        point: point,
        obstacles: obstacles,
        boosters: boosters,
    };
    let expected = "(0,0),(10,0),(10,10),(0,10)#(0,0)#(4,2),(6,2),(6,7),(4,7);(5,8),(6,8),(6,9),(5,9)#B(0,1);B(1,1);F(0,2);F(1,2);L(0,3);X(0,9);R(3,9);C(1,9)";
    assert_eq!(expected, task.to_string())
}

#[test]
fn test_task_to_string_2() {
    let map = Map(vec![
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ]);
    let point = Point::new(0, 0);
    let obstacles = vec![];
    let boosters = vec![
        BoosterLocation::new(BoosterCode::ExtensionOfTheManipulator, Point::new(0, 1)),
        BoosterLocation::new(BoosterCode::ExtensionOfTheManipulator, Point::new(1, 1)),
        BoosterLocation::new(BoosterCode::FastWheels, Point::new(0, 2)),
        BoosterLocation::new(BoosterCode::FastWheels, Point::new(1, 2)),
        BoosterLocation::new(BoosterCode::Drill, Point::new(0, 3)),
        BoosterLocation::new(BoosterCode::MysteriousPoint, Point::new(0, 9)),
        BoosterLocation::new(BoosterCode::Teleport, Point::new(3, 9)),
        BoosterLocation::new(BoosterCode::Cloning, Point::new(1, 9)),
    ];
    let task = Task {
        map: map,
        point: point,
        obstacles: obstacles,
        boosters: boosters,
    };
    let expected = "(0,0),(10,0),(10,10),(0,10)#(0,0)##B(0,1);B(1,1);F(0,2);F(1,2);L(0,3);X(0,9);R(3,9);C(1,9)";
    assert_eq!(expected, task.to_string())
}