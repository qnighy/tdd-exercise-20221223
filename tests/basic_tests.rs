use tdd_exercise_20221223::{Point, PointSet};

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

#[test]
fn test_point_to_string() {
    let point = Point::new(1, 3);
    assert_eq!(point.to_string(), "(1, 3)");
}

#[test]
fn test_point_eq() {
    assert_eq!(Point::new(1, 3), Point::new(1, 3));
}

#[test]
fn test_point_ne_y() {
    assert_ne!(Point::new(1, 3), Point::new(1, 4));
}

#[test]
fn test_point_ne_x() {
    assert_ne!(Point::new(1, 3), Point::new(2, 3));
}

#[test]
fn test_not_neighbor_if_equal() {
    assert!(!Point::new(1, 3).is_neighbor(&Point::new(1, 3)));
}

#[test]
fn test_neighbor_x_minus_1() {
    assert!(Point::new(1, 3).is_neighbor(&Point::new(0, 3)));
}

#[test]
fn test_neighbor_x_plus_1() {
    assert!(Point::new(1, 3).is_neighbor(&Point::new(2, 3)));
}

#[test]
fn test_neighbor_y_minus_1() {
    assert!(Point::new(1, 3).is_neighbor(&Point::new(1, 2)));
}
#[test]
fn test_neighbor_y_plus_1() {
    assert!(Point::new(1, 3).is_neighbor(&Point::new(1, 4)));
}

#[test]
fn test_not_neighbor_x_y_minus_1() {
    assert!(!Point::new(1, 3).is_neighbor(&Point::new(0, 2)));
}

#[test]
fn test_point_set_new() {
    PointSet::new(&Point::new(1, 3), &Point::new(5, 9));
}

#[test]
fn test_point_set_new3() {
    PointSet::new3(&Point::new(1, 3), &Point::new(5, 9), &Point::new(-3, 3));
}

#[test]
#[should_panic(expected = "assertion failed: `(left != right)`")]
fn test_point_set_new_with_same_point() {
    PointSet::new(&Point::new(1, 3), &Point::new(1, 3));
}

#[test]
#[should_panic(expected = "assertion failed: `(left != right)`")]
fn test_point_set_new3_with_same_point_12() {
    PointSet::new3(&Point::new(1, 3), &Point::new(1, 3), &Point::new(1, 3));
}

#[test]
fn test_point_set_contains_left() {
    let s = PointSet::new(&Point::new(1, 3), &Point::new(5, 9));
    assert!(s.contains(&Point::new(1, 3)));
}

#[test]
fn test_point_set_contains_right() {
    let s = PointSet::new(&Point::new(1, 3), &Point::new(5, 9));
    assert!(s.contains(&Point::new(5, 9)));
}

#[test]
fn test_point_set_contains_3() {
    let s = PointSet::new3(&Point::new(1, 3), &Point::new(5, 9), &Point::new(-3, 3));
    assert!(s.contains(&Point::new(-3, 3)));
}

#[test]
fn test_point_set_non_containment() {
    let s = PointSet::new(&Point::new(1, 3), &Point::new(5, 9));
    assert!(!s.contains(&Point::new(3, 3)));
}

#[test]
fn test_point_set_connected_up() {
    let s = PointSet::new(&Point::new(1, 3), &Point::new(1, 4));
    assert!(s.is_connected());
}

#[test]
fn test_point_set_connected_right() {
    let s = PointSet::new(&Point::new(1, 3), &Point::new(2, 3));
    assert!(s.is_connected());
}

#[test]
fn test_point_set_not_connected() {
    let s = PointSet::new(&Point::new(1, 3), &Point::new(2, 4));
    assert!(!s.is_connected());
}
