mod day1;
mod day2;
mod read;

fn main() {
    let data1 = read::file_to_vec("data/day1.txt");
    println!("Total increased is: {}", day1::solution::solution1(data1.clone()));
    // println!("Total increased sums is: {}", day1::solution::solution2(data1.clone()));
    println!("Total increased sums is: {}", day1::solution::solution2(vec![1, 2, 3, 4]));
    let data2 = read::file_to_tuple("data/day2.txt");
    let result2_1 = day2::solution::solution1(data2);
    println!(
        "Horizontal is {} and depth is {} and their multiply is {}",
        result2_1.0, result2_1.1, result2_1.0 * result2_1.1
    )
}


#[test]
fn final_tests() {
    let data1 = read::file_to_vec("data/day1.txt");
    assert_eq!(day1::solution::solution1(data1.clone()), 1298);
    assert_eq!(day1::solution::solution2(data1.clone()), 1248);
    let data2 = read::file_to_tuple("data/day2.txt");
    assert_eq!(day2::solution::solution1(data2.clone()), (1944, 1049));
}
