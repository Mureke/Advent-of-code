use std::fs;

pub fn read_file(file_path: &str) -> String {
    let input_data = fs::read_to_string(file_path).expect("File doesn't exist");
    input_data
}

pub fn parse_input(input: String) -> Vec<i32> {
    input.trim().split('\n').map(|s| s.parse().unwrap()).collect()
}