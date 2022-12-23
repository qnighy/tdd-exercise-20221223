use tdd_exercise_20221223::Point;

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
