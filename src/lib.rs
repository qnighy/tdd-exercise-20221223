use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn is_neighbor(&self, pt: &Point) -> bool {
        [
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ]
        .contains(pt)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct PointSet {
    pt1: Point,
    pt2: Point,
}

impl PointSet {
    pub fn new(pt1: &Point, pt2: &Point) -> Self {
        assert_ne!(pt1, pt2);
        Self {
            pt1: *pt1,
            pt2: *pt2,
        }
    }

    pub fn new3(pt1: &Point, pt2: &Point, pt3: &Point) -> Self {
        assert_ne!(pt1, pt2);
        Self {
            pt1: *pt1,
            pt2: *pt2,
        }
    }

    pub fn contains(&self, pt: &Point) -> bool {
        [self.pt1, self.pt2].contains(pt)
    }

    pub fn is_connected(&self) -> bool {
        self.pt1.is_neighbor(&self.pt2)
    }
}
