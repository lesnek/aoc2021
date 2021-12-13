fn parse_matrix(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .into_iter()
                .map(|y| y.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solution1(input: String) -> u32 {
    let values = parse_matrix(&input);
    let matrix_size_x = values.len();
    let matrix_size_y = values[0].len();
    let mut total = 0;

    for x in 0..matrix_size_x {
        for y in 0..matrix_size_y {
            let mut is_small = true;
            let left = x as i32 - 1;
            let right = x as i32 + 1;
            let top = y as i32 + 1;
            let down = y as i32 - 1;
            let check: u32 = get_coord(x, y, &values);
            for coord_x in [left, right] {
                if !check_coord(coord_x, matrix_size_x as i32) {
                    continue;
                }
                if check >= get_coord(coord_x as usize, y, &values) {
                    is_small = false
                }
            }
            for coord_y in [top, down] {
                if !check_coord(coord_y, matrix_size_y as i32) {
                    continue;
                }
                if check >= get_coord(x, coord_y as usize, &values) {
                    is_small = false
                }
            }
            if is_small {
                total += check + 1
            }
        }
    }
    total
}

fn get_coord(x: usize, y: usize, data: &[Vec<u32>]) -> u32 {
    data.get(x).unwrap().get(y).unwrap().to_owned()
}

fn check_coord(coord: i32, max_size: i32) -> bool {
    !(max_size <= coord || coord < 0)
}

#[cfg(test)]
mod tests {
    use crate::day9::solution::solution1;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let test_input = read_to_string("data/day9_test.txt").unwrap();
        assert_eq!(solution1(test_input), 15);
    }
    #[test]
    fn test_solution1_2() {
        let test_input = read_to_string("data/day9.txt").unwrap();
        assert_eq!(solution1(test_input), 545);
    }
}
