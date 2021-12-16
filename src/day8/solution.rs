use std::collections::HashSet;

fn parse_segments_output(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|x| x.split(" | ").last().unwrap())
        .flat_map(|x| x.split_whitespace().map(|y| y.to_string()))
        .collect()
}

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|x| {
            x.split(" | ")
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
        })
        .map(|x| {
            vec_to_tuple(
                &x.into_iter()
                    .map(|j| {
                        j.split_whitespace()
                            .map(|y| y.chars().collect::<SegmentType>())
                            .collect::<Vec<SegmentType>>()
                    })
                    .collect::<Vec<Vec<SegmentType>>>(),
            )
        })
        .collect()
}

fn vec_to_tuple<T: Clone>(xs: &[Vec<T>]) -> (Vec<T>, Vec<T>) {
    assert_eq!(xs.len(), 2);
    (xs[0].clone(), xs[1].clone())
}

pub fn solution1(input: String) -> usize {
    let values = parse_segments_output(&input);
    let unique_digits: Vec<usize> = vec![2, 3, 4, 7];

    values
        .into_iter()
        .filter(|x| unique_digits.contains(&x.len()))
        .count()
}

type Input = Vec<(Vec<SegmentType>, Vec<SegmentType>)>;

pub fn solution2(input: Input) -> u32 {
    input.iter().map(decode_input_line).sum()
}

fn decode_input_line((noise, output): &(Vec<SegmentType>, Vec<SegmentType>)) -> u32 {
    let all_segments = noise
        .iter()
        .chain(output)
        .map(|i| i.to_owned())
        .collect::<Vec<SegmentType>>();
    decode(&all_segments, output)
}

type SegmentType = HashSet<char>;

fn decode(all_segments: &[SegmentType], output: &[SegmentType]) -> u32 {
    let uniques = find_uniques(all_segments);
    output
        .iter()
        .map(|segment| decode_segment(&uniques, segment).to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn intersect_segments(left: &SegmentType, right: &SegmentType) -> SegmentType {
    left.intersection(right).copied().collect::<SegmentType>()
}

fn decode_segment(uniques: &[SegmentType; 3], segment: &SegmentType) -> u32 {
    let [one_segment, four_segment, seven_segment] = uniques;

    match segment.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if intersect_segments(segment, one_segment) == *one_segment {
                3
            } else if intersect_segments(segment, four_segment).len() == 3 {
                5
            } else {
                2
            }
        }
        6 => {
            if intersect_segments(segment, seven_segment) != *seven_segment {
                6
            } else if intersect_segments(segment, four_segment) == *four_segment {
                9
            } else {
                0
            }
        }
        7 => 8,
        _ => panic!("je to v pici"),
    }
}

fn find_uniques(all_segments: &[HashSet<char>]) -> [SegmentType; 3] {
    let (mut one, mut four, mut seven) = (None, None, None);

    for segment in all_segments {
        match segment.len() {
            2 => one = Some(segment.clone()),
            4 => four = Some(segment.clone()),
            3 => seven = Some(segment.clone()),
            _ => continue,
        }
    }

    [one, four, seven].map(|i| i.unwrap())
}

#[cfg(test)]
mod tests {
    use crate::day8::solution::*;
    use std::collections::HashSet;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let test_input = read_to_string("data/day8_test.txt").unwrap();
        assert_eq!(solution1(test_input), 26);
        let test_input2 = read_to_string("data/day8.txt").unwrap();
        assert_eq!(solution1(test_input2), 272);
    }

    #[test]
    fn test_input_data() {
        let test_input = read_to_string("data/day8.txt").unwrap();
        let values = parse_input(&test_input);
        let unique_digits = HashSet::from([2, 3, 4, 7]);
        for (kok1, kok2) in values {
            let lengths: Vec<i32> = [kok1, kok2]
                .concat()
                .iter()
                .map(|x| x.len() as i32)
                .collect();

            let check = HashSet::from_iter(lengths.into_iter());
            let inter = check
                .intersection(&unique_digits)
                .collect::<HashSet<&i32>>();
            if inter.len() != 4 {
                panic!("Je to v pici")
            }
        }
    }

    #[test]
    fn test_solution2() {
        let test_input = read_to_string("data/day8_test.txt").unwrap();
        let parsed_input = parse_input(&test_input);
        assert_eq!(solution2(parsed_input), 61229);

        let test_input = read_to_string("data/day8.txt").unwrap();
        let parsed_input = parse_input(&test_input);
        assert_eq!(solution2(parsed_input), 1007675);
    }
}
