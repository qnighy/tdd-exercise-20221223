use std::{collections::HashSet, fmt};

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
    pts: HashSet<Point>,
}

impl PointSet {
    pub fn new(pt1: &Point, pt2: &Point) -> Self {
        Self {
            pts: vec![*pt1, *pt2].into_iter().collect(),
        }
    }

    pub fn new3(pt1: &Point, pt2: &Point, pt3: &Point) -> Self {
        Self {
            pts: vec![*pt1, *pt2, *pt3].into_iter().collect(),
        }
    }

    pub fn contains(&self, pt: &Point) -> bool {
        self.pts.contains(pt)
    }

    pub fn is_connected(&self) -> bool {
        let mut pts = self.pts.iter().copied().collect::<Vec<_>>();
        // For stability
        pts.sort();
        pts.len() < 2 || pts[0].is_neighbor(&pts[1])
    }
}
