use std::iter::FromIterator;
use std::fs;
use crate::computer::{Computer};

mod computer;



fn main() {
    let input_data = fs::read_to_string("input.txt").expect("File doesn't exist");

    let initial_memory: Vec<i32> = input_data.trim().split(',').map(|s| s.parse().unwrap()).collect();

    let mut computer: Computer = Computer::new(initial_memory, vec![12]);
    computer.execute();
    println!("{}", computer.memory[0]);
}
