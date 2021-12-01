use crate::lib::file_reader::read_file;
use crate::lib::file_reader::parse_input;


pub fn solve() {
    let input: Vec<i32> = parse_input(read_file("day_1.txt"));
    let mut count = 0;
    let mut count2 = 0;

    let mut a_vec = &input[0..3];
    let mut b_vec = &input[1..4];

    let mut prev = &input[0];


    for i in input.iter().enumerate() {
        if i.1 > prev {
            count = count + 1
        }
        prev = i.1;

        let sum1: i32 = a_vec.iter().sum();
        let sum2 = b_vec.iter().sum();
        if sum1 < sum2 {
            count2 = count2 + 1
        }

        if i.0 + 4 > input.len() {
            break;
        }
        a_vec = b_vec;
        b_vec = &input[(i.0 + 1) as usize..(i.0 + 4) as usize]
    }

    println!("Day 1: \n1. {}\n2. {}", count, count2);
}