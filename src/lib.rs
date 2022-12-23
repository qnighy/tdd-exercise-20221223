use std::{collections::HashSet, fmt, hash::Hash};

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

pub trait IsNeighbor {
    fn is_neighbor(&self, pt: &Self) -> bool;
}
impl IsNeighbor for Point {
    fn is_neighbor(&self, pt: &Self) -> bool {
        Point::is_neighbor(self, pt)
    }
}

pub type PointSet = NeighborSet<Point>;

#[derive(Debug, Clone)]
pub struct NeighborSet<T>
where
    T: Hash + Eq,
{
    pts: HashSet<T>,
}

impl<T> NeighborSet<T>
where
    T: Clone + Hash + Eq + Ord + IsNeighbor,
{
    pub fn new(pt1: &T, pt2: &T) -> Self {
        Self {
            pts: [pt1, pt2].iter().copied().cloned().collect(),
        }
    }

    pub fn new3(pt1: &T, pt2: &T, pt3: &T) -> Self {
        Self {
            pts: [pt1, pt2, pt3].iter().copied().cloned().collect(),
        }
    }

    pub fn contains(&self, pt: &T) -> bool {
        self.pts.contains(pt)
    }

    pub fn is_connected(&self) -> bool {
        let mut pts = self.pts.iter().cloned().collect::<Vec<_>>();
        // For stability
        pts.sort();
        if pts.len() <= 1 {
            true
        } else if pts.len() == 2 {
            pts[0].is_neighbor(&pts[1])
        } else if pts.len() == 3 {
            pts[0].is_neighbor(&pts[1]) && pts[1].is_neighbor(&pts[2])
                || pts[0].is_neighbor(&pts[1]) && pts[0].is_neighbor(&pts[2])
                || pts[0].is_neighbor(&pts[2]) && pts[1].is_neighbor(&pts[2])
        } else {
            todo!("more than 3 points")
        }
    }
}
