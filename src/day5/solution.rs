use std::collections::HashMap;
use serde_scan::scan;
use std::fs::read_to_string;


pub fn solution1(data: String) -> i32 {
    let mut total: i32 = 0;
    let mut map = HashMap::new();
    for ((x1, y1), (x2, y2)) in parse(&data) {
        if x1 != x2 && y1 != y2{
            continue
        }

        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                let key = format!("{},{}", x, y);
                if map.contains_key(&key) {
                    map.insert(key.clone(), map[&key] + 1);
                } else {
                    map.insert(key.clone(), 1);
                }
            }
        }
    }
    for (_, value) in map.into_iter() {
        if value >= 2 {
            total += 1;
        }
    }
    total
}

pub fn solution2(data: String) -> i32 {
    let mut total: i32 = 0;
    let mut map = HashMap::new();
    for ((x1, y1), (x2, y2)) in parse(&data) {
        let start = (x1.min(x2), y1.min(y2));
        let end = (x2.max(x1), y2.max(y1));

        let (start_x, start_y) = start;
        let (end_x, end_y) = end;

        let direction = ((end_x - start_x).signum(), (end_y - start_y).signum());
        let mut current = start;

        loop {
            let key = format!("{},{}", current.0, current.1);
            if map.contains_key(&key) {
                map.insert(key.clone(), map[&key] + 1);
            } else {
                map.insert(key.clone(), 1);
            }
            current = (current.0 + direction.0, current.1 + direction.1);
            if current == end {
                let key2 = format!("{},{}", current.0, current.1);
                if map.contains_key(&key2) {
                    map.insert(key2.clone(), map[&key2] + 1);
                } else {
                    map.insert(key2.clone(), 1);
                }
                break
            }
        }
    }
    for (_, value) in map.into_iter() {
        if value >= 2 {
            total += 1;
        }
    }
    total
}

fn parse(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .lines()
        .map(|line| scan!("{},{} -> {},{}" <- line).unwrap())
        .map(|values: (i32, i32, i32, i32)| {
            (
                (
                    values.0,
                    values.1,
                ),
                (
                    values.2,
                    values.3,
                ),
            )
        })
        .collect()
}

#[test]
fn test_solution1() {
    let data = read_to_string("data/day5_test.txt").unwrap();
    assert_eq!(solution1(data.clone()), 5);
    assert_eq!(solution2(data), 12);
}
