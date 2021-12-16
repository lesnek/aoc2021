use std::collections::HashMap;

struct Data {
    openers: [char; 4],
    mappings: HashMap<char, char>,
    bug_costs: HashMap<char, u128>,
    fill_scores: HashMap<char, u128>,
}

impl Data {
    fn new() -> Data {
        Data {
            openers: ['(', '[', '{', '<'],
            mappings: HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]),
            bug_costs: HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]),
            fill_scores: HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]),
        }
    }
}

pub fn solution1(input: String) -> u128 {
    let data = Data::new();
    input.lines().map(|line| get_bug_cost(line, &data)).sum()
}

fn get_bug_cost(line: &str, data: &Data) -> u128 {
    let mut stack = vec![];
    let mut result = 0;

    for char in line.chars() {
        if data.openers.contains(&char) {
            stack.push(data.mappings[&char]);
        } else if let Some(stored) = stack.pop() {
            if char != stored {
                result = data.bug_costs[&char];
                break;
            }
        }
    }
    result
}

pub fn solution2(input: String) -> u128 {
    let mut completion_scores = vec![];
    let data = Data::new();

    for line in input.lines() {
        let mut stack = vec![];
        let mut is_corrupt = false;

        for character in line.chars() {
            if data.openers.contains(&character) {
                stack.push(data.mappings[&character]);
            } else if let Some(stored) = stack.pop() {
                if character != stored {
                    is_corrupt = true;
                    break;
                }
            }
        }

        if !is_corrupt {
            let mut line_score = 0;
            while let Some(stored) = stack.pop() {
                line_score = 5 * line_score + data.fill_scores[&stored];
            }
            completion_scores.push(line_score);
        }
    }

    completion_scores.sort_unstable();
    completion_scores[completion_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::day10::solution::*;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let test_input = read_to_string("data/day10_test.txt").unwrap();
        assert_eq!(solution1(test_input), 26397);
    }

    #[test]
    fn test_solution1_2() {
        let test_input = read_to_string("data/day10.txt").unwrap();
        assert_eq!(solution1(test_input), 339411);
    }

    #[test]
    fn test_solution_2_1() {
        let test_input = read_to_string("data/day10_test.txt").unwrap();
        assert_eq!(solution2(test_input), 288957);
    }

    #[test]
    fn test_solution_2_2() {
        let test_input = read_to_string("data/day10.txt").unwrap();
        assert_eq!(solution2(test_input), 2289754624);
    }
}
