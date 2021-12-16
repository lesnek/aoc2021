use std::collections::{HashMap, HashSet};

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

pub fn solution2(input: String) -> u32 {
    let values = parse_matrix(&input);

    let width = values[0].len();
    let height = values.len();
    let basin_center = get_min(&values, width, height);

    let mut basin_min = vec![];
    for &(x, y) in basin_center.keys() {
        basin_min.push(get_basin_size(
            &values,
            width,
            height,
            x,
            y,
            &mut HashSet::new(),
        ));
    }
    basin_min.sort_unstable();
    basin_min.reverse();
    basin_min.into_iter().take(3).product()
}

fn get_min(grid: &[Vec<u32>], width: usize, height: usize) -> HashMap<(usize, usize), u32> {
    let mut result = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            let neighbors = get_neighbors(width, height, x, y);
            if !neighbors
                .into_iter()
                .any(|(n_x, n_y)| grid[n_y][n_x] <= grid[y][x])
            {
                result.insert((x, y), grid[y][x]);
            }
        }
    }
    result
}

fn get_basin_size(
    grid: &[Vec<u32>],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    already_considered: &mut HashSet<(usize, usize)>,
) -> u32 {
    let mut result = 1;
    let neighbors = get_neighbors(width, height, x, y);
    already_considered.insert((x, y));
    for (x_neighbor, y_neighbor) in neighbors {
        let n_height = grid[y_neighbor][x_neighbor];
        if n_height < 9
            && n_height >= grid[y][x]
            && !already_considered.contains(&(x_neighbor, y_neighbor))
        {
            result += get_basin_size(
                grid,
                width,
                height,
                x_neighbor,
                y_neighbor,
                already_considered,
            );
        }
    }
    result
}

fn get_neighbors(width: usize, height: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < width - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < height - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use crate::day9::solution::*;
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

    #[test]
    fn test_solution_2_1() {
        let test_input = read_to_string("data/day9_test.txt").unwrap();
        assert_eq!(solution2(test_input), 1134);
    }

    #[test]
    fn test_solution_2_2() {
        let test_input = read_to_string("data/day9.txt").unwrap();
        assert_eq!(solution2(test_input), 950600);
    }
}
