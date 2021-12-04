pub mod day1;
pub mod day2;
pub mod read;

fn main() {
    // println!("DAY 1");
    // let data1 = read::file_to_vec("data/day1.txt");
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
}


#[test]
fn final_tests() {
    let data1 = read::file_to_vec("data/day1.txt");
    assert_eq!(day1::solution::solution1(data1.clone()), 1298);
    assert_eq!(day1::solution::solution2(data1.clone()), 1248);
    let data2 = read::file_to_tuple("data/day2.txt");
    assert_eq!(day2::solution::solution1(data2.clone()), (1944, 1049));
    assert_eq!(day2::solution::solution2(data2.clone()), (1944, 954969));
}
