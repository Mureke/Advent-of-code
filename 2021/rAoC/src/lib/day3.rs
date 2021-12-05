use crate::lib::file_reader::read_file;
use crate::lib::file_reader::parse_input;

fn get_parsed_data() -> Vec<Vec<u32>> {
    let mut input: Vec<String> = parse_input(read_file("day_3.txt"));
    let mut matrix: Vec<Vec<u32>> = vec![];
    for i in input.iter() {
        let str_as_vec: Option<Vec<u32>> = i.chars().map(|ch| ch.to_digit(10)).collect();
        matrix.append(&mut vec![str_as_vec.unwrap()]);
    }
    return matrix;
}

fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
}

pub fn solve() {
    let mut matrix = get_parsed_data();

    let matrix_processed : Vec<Vec<u32>> = (0..matrix[0].len()).map(|i| matrix
        .iter()
        .map(|c| c[i])
        .collect()
    ).collect();

    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");

    for list in matrix_processed {
        let one_count = list.iter().filter(|x| x == &&1).count();
        if one_count > list.len() / 2 {
            gamma_rate += "1";
            epsilon_rate += "0";
            continue
        }
        gamma_rate += "0";
        epsilon_rate += "1";
    }

    let gammai32 = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsiloni32 = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("Day3:\n1. {}",  gammai32*epsiloni32);
}