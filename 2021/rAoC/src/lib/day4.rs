use crate::lib::file_reader::read_file;
use crate::lib::file_reader::parse_input;

pub fn solve() {
    let mut input: Vec<String> = parse_input(read_file("day_4.txt"));

    let bingo_numbers_str: String = input.get(0).unwrap().parse().unwrap();
    let bingo_numbers: Vec<i32> =  bingo_numbers_str.trim().split(',').map(|s| s.parse().unwrap()).collect();
    input.remove(0);
    input.remove(0);




    println!("Day 4");

}