pub fn solution1(data: Vec<String>) -> (u32, u32) {
    let mut gamma_list: Vec<u32> = vec![0; data[0].len()];
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    let half_of_total = (data.len() as u32) / 2;
    for line in data {
        for (index, value) in line.chars().into_iter().enumerate() {
            if value.to_string() == "1" {
                gamma_list[index] += 1
            }
        }
    }
    for value in gamma_list {
        if value < half_of_total {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    let bin_gamma = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    let bin_epsilon = u32::from_str_radix(epsilon.as_str(), 2).unwrap();
    (bin_gamma, bin_epsilon)
}

#[test]
fn test_solution_1() {
    assert_eq!(
        solution1(vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ]),
        (22, 9)
    )
}

#[derive(Clone, Copy)]
enum ChemType {
    Oxygen,
    CO2,
}

pub fn solution2(data: Vec<String>) -> (u32, u32) {
    let bin_oxygen = u32::from_str_radix(
        filter_by_chem_type(data.clone(), ChemType::Oxygen)[0].as_str(),
        2,
    )
    .unwrap();

    let bin_co2 =
        u32::from_str_radix(filter_by_chem_type(data, ChemType::CO2)[0].as_str(), 2).unwrap();
    (bin_oxygen, bin_co2)
}

fn filter_by_chem_type(mut data: Vec<String>, chem_type: ChemType) -> Vec<String> {
    let mut position = 0;
    while data.len() > 1 {
        let (data0, data1) = filter_data(data, position);
        data = choose_data(data0, data1, chem_type);
        position += 1;
    }
    data
}

fn choose_data(data0: Vec<String>, data1: Vec<String>, chem_type: ChemType) -> Vec<String> {
    match chem_type {
        ChemType::Oxygen => {
            if data1.len() >= data0.len() {
                return data1;
            }
            data0
        }
        ChemType::CO2 => {
            if data0.len() <= data1.len() {
                return data0;
            }
            data1
        }
    }
}

fn filter_data(data: Vec<String>, position: u32) -> (Vec<String>, Vec<String>) {
    let mut data1 = Vec::new();
    let mut data0 = Vec::new();
    for line in data {
        if line.chars().nth(position as usize).unwrap().to_string() == "1" {
            data1.push(line);
        } else {
            data0.push(line);
        }
    }
    (data0, data1)
}

#[test]
fn test_solution_2() {
    assert_eq!(
        solution2(vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ]),
        (23, 10)
    )
}
