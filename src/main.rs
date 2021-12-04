pub mod day1;
pub mod day2;
pub mod day3;
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
    let data3 = read::file_to_str_vec("data/day3.txt");
    let result3_1 = day3::solution::solution1(data3.clone());
    println!(
        "Gamma is {} and epsilon is {} and their multiply is {}",
        result3_1.0,
        result3_1.1,
        result3_1.0 * result3_1.1
    );
    let result3_2 = day3::solution::solution2(data3);
    println!(
        "Oxygen is {} and co2 is {} and their multiply is {}",
        result3_2.0,
        result3_2.1,
        result3_2.0 * result3_2.1
    );
}

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
}
