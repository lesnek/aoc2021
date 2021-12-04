mod day1;
mod read;

fn main() {
    let data1 = read::file_to_vec("data/day1.txt");
    println!("Total increased is: {}", day1::solution::solution1(data1.clone()));
    // println!("Total increased sums is: {}", day1::solution::solution2(data1.clone()));
    println!("Total increased sums is: {}", day1::solution::solution2(vec![1,2,3,4]));
}


#[test]
fn final_tests() {
    let data1 = read::file_to_vec("data/day1.txt");
    assert_eq!(day1::solution::solution1(data1.clone()), 1298);
    assert_eq!(day1::solution::solution2(data1.clone()), 1248);
}
