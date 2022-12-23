#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[test]
fn test_create_point() {
    Point::new(1, 3);
}

#[test]
fn test_point_x() {
    let point = Point::new(1, 3);
    assert_eq!(point.x, 1);
}

#[test]
fn test_point_y() {
    let point = Point::new(1, 3);
    assert_eq!(point.y, 3);
}
