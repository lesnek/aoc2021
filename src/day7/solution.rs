fn parse_crab(input: String) -> Vec<i128> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
}

pub fn solution1(input: String) -> i128 {
    let crabs = parse_crab(input);
    let mut consumption = i128::MAX;

    for crab in *crabs.iter().min().unwrap()..*crabs.iter().max().unwrap() {
        let cost = crabs.iter().map(|x| (x - crab).abs()).sum();
        if cost < consumption {
            consumption = cost
        }
    }
    consumption
}

pub fn solution2(input: String) -> i128 {
    let crabs: Vec<i128> = parse_crab(input);
    let mut consumption = i128::MAX;

    for crab in *crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap() {
        let cost = crabs
            .iter()
            .map(|x| (1..=(x - crab).abs()).fold(0, |a, b| a + b))
            .sum();
        if cost < consumption {
            consumption = cost
        }
    }
    consumption
}

#[cfg(test)]
mod tests {
    use crate::day7::solution::{solution1, solution2};
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let test_input = "16,1,2,0,4,2,7,1,2,14".to_string();
        assert_eq!(solution1(test_input), 37);
        let test_input2 = read_to_string("data/day7.txt").unwrap();
        assert_eq!(solution1(test_input2), 341558);
    }

    #[test]
    fn test_solution2() {
        let test_input = "16,1,2,0,4,2,7,1,2,14".to_string();
        assert_eq!(solution2(test_input), 168);
        // TODO: Optimize, too slow, but works.
        // let test_input2 = read_to_string("data/day7.txt").unwrap();
        // assert_eq!(solution2(test_input2), 93214037);
    }
}
