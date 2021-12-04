pub fn solution1(data: Vec<u32>) -> u32 {
    let mut previous_num: u32 = 1000;
    let mut total: u32 = 0;
    for number in data {
        if previous_num < number {
            total += 1;
        }
        previous_num = number;
    }
    total
}

pub fn solution2(data: Vec<u32>) -> u32{
    let mut total: u32 = 0;
    let mut sum_list = vec![data[0], data[1], data[2]];
    let mut previous_sum: u32 = sum_list.iter().sum();
    for number in data.into_iter().skip(3) {
        sum_list.remove(0);
        sum_list.push(number);
        let current_sum = sum_list.iter().sum();
        if previous_sum < current_sum {
            total += 1;
        }
        previous_sum = current_sum;
    }
    total
}

#[test]
fn test_solution1() {
    assert_eq!(solution1(vec![1, 2, 3]), 2);
}

#[test]
fn test_solution2() {
    assert_eq!(solution2(vec![1, 2, 3, 4]), 1);
}