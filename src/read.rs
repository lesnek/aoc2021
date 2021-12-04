use std::fs;

pub fn file_to_vec(path: &'static str) -> Vec<u32> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|v| v.parse().expect("parse error"))
        .collect()
}