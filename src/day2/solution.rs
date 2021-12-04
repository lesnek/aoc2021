pub fn solution1(data: Vec<(String, i32)>) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    for (direction, value) in data {
        match direction.as_str() {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Je to v pici")
        }
    }
    (horizontal, depth)
}

#[test]
fn test_solution_1() {
    assert_eq!(
        solution1(vec![
            ("forward".to_string(), 5),
            ("down".to_string(), 5),
            ("forward".to_string(), 8),
            ("up".to_string(), 3),
            ("down".to_string(), 8),
            ("forward".to_string(), 2),
        ]),
        (15, 10)
    )
}