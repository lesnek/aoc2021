fn parse_fish(input: String) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
}

pub fn solution1(input: String, days: u32) -> u64 {
    let fish = parse_fish(input);
    let mut ages = vec![0; 9];
    for fish_age in fish {
        ages[fish_age] += 1;
    }

    for _ in 0..days {
        fish_lifecycle(&mut ages);
    }

    ages.iter().sum::<u64>()
}

fn fish_lifecycle(age: &mut Vec<u64>) {
    let new_fish = age[0];
    age[0] = age[1];
    age[1] = age[2];
    age[2] = age[3];
    age[3] = age[4];
    age[4] = age[5];
    age[5] = age[6];
    age[6] = age[7] + new_fish;
    age[7] = age[8];
    age[8] = new_fish;
}

#[cfg(test)]
mod tests {
    use crate::day6::solution::solution1;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let input = read_to_string("data/day6_test.txt").unwrap();
        assert_eq!(solution1("3".to_string(), 18), 5);
        assert_eq!(solution1(input.clone(), 18), 26);
        assert_eq!(solution1(input.clone(), 80), 5934);
        assert_eq!(solution1(input, 256), 26984457539);
    }
}
