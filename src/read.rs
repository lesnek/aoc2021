use std::fs;

pub fn file_to_u32_vec(path: &'static str) -> Vec<u32> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|v| v.parse().expect("parse error"))
        .collect()
}

pub fn file_to_str_vec(path: &'static str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|v| v.to_owned())
        .collect()
}

pub fn file_to_tuple(path: &str) -> Vec<(String, i32)> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|dire| {
            let it_snow: Vec<&str> = dire.split_whitespace().collect();
            (it_snow[0].to_owned(), it_snow[1].parse::<i32>().unwrap())
        })
        .collect()
}
