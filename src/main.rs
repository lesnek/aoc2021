// use std::fs::read_to_string;
pub mod day1;
pub mod day10;
pub mod day13;
pub mod day14;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod read;

fn main() {
    // println!("DAY 1");
    // let data1 = read::file_to_u32_vec("data/day1.txt");
    // println!("Total increased is: {}", day1::solution::solution1(data1.clone()));
    // println!("Total increased sums is: {}", day1::solution::solution2(vec![1, 2, 3, 4]));
    //
    //
    // println!("DAY 2");
    // let data2 = read::file_to_tuple("data/day2.txt");
    // let result2_1 = day2::solution::solution1(data2.clone());
    // println!(
    //     "Horizontal is {} and depth is {} and their multiply is {}",
    //     result2_1.0, result2_1.1, result2_1.0 * result2_1.1
    // );
    // let result2_2 = day2::solution::solution2(data2.clone());
    // println!(
    //     "Horizontal is {} and depth is {} and their multiply is {}",
    //     result2_2.0, result2_2.1, result2_2.0 * result2_2.1
    // );
    // let data3 = read::file_to_str_vec("data/day3.txt");
    // let result3_1 = day3::solution::solution1(data3.clone());
    // println!(
    //     "Gamma is {} and epsilon is {} and their multiply is {}",
    //     result3_1.0,
    //     result3_1.1,
    //     result3_1.0 * result3_1.1
    // );
    // let result3_2 = day3::solution::solution2(data3);
    // println!(
    //     "Oxygen is {} and co2 is {} and their multiply is {}",
    //     result3_2.0,
    //     result3_2.1,
    //     result3_2.0 * result3_2.1
    // );
    // let test_input = read_to_string("data/day13.txt").unwrap();
    // let parsed_input = day13::solution::parse_input(&test_input);
    // day13::solution::solution2(&parsed_input);
}

#[cfg(test)]
mod tests {
    use crate::{day1, day2, day3, day4, day5, day6, read};
    use std::fs::read_to_string;

    #[test]
    fn final_tests() {
        let data1 = read::file_to_u32_vec("data/day1.txt");
        assert_eq!(day1::solution::solution1(data1.clone()), 1298);
        assert_eq!(day1::solution::solution2(data1.clone()), 1248);
        let data2 = read::file_to_tuple("data/day2.txt");
        assert_eq!(day2::solution::solution1(data2.clone()), (1944, 1049));
        assert_eq!(day2::solution::solution2(data2.clone()), (1944, 954969));
        let data3 = read::file_to_str_vec("data/day3.txt");
        assert_eq!(day3::solution::solution1(data3.clone()), (3437, 658));
        assert_eq!(day3::solution::solution2(data3), (3995, 1696));
        let data4 = read_to_string("data/day4.txt").unwrap();
        assert_eq!(day4::solution::solution1(data4.clone()), 50008);
        assert_eq!(day4::solution::solution2(data4), 17408);
        let data5 = read_to_string("data/day5.txt").unwrap();
        assert_eq!(day5::solution::solution1(data5.clone()), 7436);
        assert_eq!(day5::solution::solution2(data5), 21104);
        let data6 = read_to_string("data/day6.txt").unwrap();
        assert_eq!(day6::solution::solution1(data6.clone(), 80), 345387);
        assert_eq!(day6::solution::solution1(data6.clone(), 256), 1574445493136);
    }
}
