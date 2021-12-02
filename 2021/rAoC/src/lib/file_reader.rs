use std::fmt::Debug;
use std::fs;

pub fn read_file(file_path: &str) -> String {
    let input_data = fs::read_to_string(file_path).expect("File doesn't exist");
    input_data
}

pub fn parse_input<T>(input: String) -> Vec<T>
    where <
          T as std::str::FromStr>::Err: Debug,
          T: std::str::FromStr
{

    input.trim().split('\n').map(|s| s.parse().unwrap()).collect()
}