use core::str::Lines;
use std::collections::HashSet;
use std::fs::read_to_string;

const BOARD_SIZE: usize = 5;

fn get_called_numbers(mut input: &mut Lines) -> Vec<u32> {
    (&mut input)
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn solution1(input: String) -> u32 {
    let mut lines = input.lines();
    let called_nums: Vec<u32> = get_called_numbers(&mut lines);

    let mut earliest_win = usize::MAX; // Infinite.
    let mut winning_score: u32 = 0;
    loop {
        if (&mut lines).next().is_none() {
            // Fuck off, GAME OVER.
            break;
        }
        let game_data: Vec<&str> = (&mut lines).take(BOARD_SIZE).collect();
        let bingo = BingoBoard::new(game_data);

        let result = bingo.play(&called_nums);
        if result.winning_move_num < earliest_win {
            earliest_win = result.winning_move_num;
            winning_score = result.score;
        }
    }

    winning_score
}

pub fn solution2(input: String) -> u32 {
    let mut lines = input.lines();
    let called_nums: Vec<u32> = get_called_numbers(&mut lines);

    let mut latest_win = usize::MIN;
    let mut winning_score: u32 = 0;
    loop {
        if (&mut lines).next().is_none() {
            // Fuck off, GAME OVER.
            break;
        }
        let game_data: Vec<&str> = (&mut lines).take(BOARD_SIZE).collect();
        let bingo = BingoBoard::new(game_data);

        let result = bingo.play(&called_nums);
        if result.winning_move_num > latest_win {
            latest_win = result.winning_move_num;
            winning_score = result.score;
        }
    }

    winning_score
}

struct BingoBoard {
    lines: Vec<HashSet<u32>>,
}

struct BingoResult {
    winning_move_num: usize,
    score: u32,
}

impl BingoBoard {
    fn new(board_data: Vec<&str>) -> BingoBoard {
        let mut lines = vec![];
        let mut number_lines = vec![];

        for line in board_data {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            number_lines.push(nums.clone());
            lines.push(HashSet::from_iter(nums));
        }

        let mut vertical = vec![];
        for i in 0..BOARD_SIZE {
            let vertical_line: HashSet<u32> = number_lines.iter().map(|l| l[i]).collect();
            vertical.push(vertical_line);
        }
        lines.append(&mut vertical);

        BingoBoard { lines }
    }

    fn play(&self, called_numbers: &[u32]) -> BingoResult {
        for turn in 0..called_numbers.len() {
            let mut called_so_far = HashSet::new();
            for called_number in called_numbers.iter().take(turn + 1) {
                called_so_far.insert(*called_number);
            }
            for line in self.lines.iter() {
                if called_so_far.intersection(line).count() == BOARD_SIZE {
                    let last_called = called_numbers[turn];
                    let mut score = 0;

                    for line in self.lines.iter().take(BOARD_SIZE) {
                        for val in line {
                            if !called_so_far.contains(val) {
                                score += val;
                            }
                        }
                    }

                    score *= last_called;
                    return BingoResult {
                        winning_move_num: turn,
                        score,
                    };
                }
            }
        }
        panic!("Je to v pici")
    }
}

#[test]
fn test_solution1() {
    let input = read_to_string("data/day4_test.txt");
    assert_eq!(solution1(input.unwrap()), 4512);
}

#[test]
fn test_solution2() {
    let input = read_to_string("data/day4_test.txt");
    assert_eq!(solution2(input.unwrap()), 1924);
}
