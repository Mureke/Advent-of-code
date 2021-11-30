use std::fs;

pub fn read_file(file_path: &str) -> Vec<String> {
    let input_data = fs::read_to_string(file_path).expect("File doesn't exist");

    input_data.trim().split('\n').map(|s| s.parse().unwrap()).collect()
}