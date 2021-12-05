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

pub fn solve1() -> i32 {
    let mut matrix = get_parsed_data();

    let matrix_processed: Vec<Vec<u32>> = (0..matrix[0].len()).map(|i| matrix
        .iter()
        .map(|c| c[i])
        .collect()
    ).collect();

    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");


    let mut i = 0;
    for list in matrix_processed {
        let one_count = list.iter().filter(|x| x == &&1).count();
        if one_count > list.len() / 2 {
            gamma_rate += "1";
            epsilon_rate += "0";
        } else {
            gamma_rate += "0";
            epsilon_rate += "1";
        }
        i += 1;
    }


    let gammai32 = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsiloni32 = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    gammai32 * epsiloni32
}

fn solve2() -> i32 {
    let mut matrix = get_parsed_data();

    let i = 0;
    let mut oxygen_generator_rating = matrix.clone();
    let mut co2_scrubber_rating = matrix.clone();

    let mut i = 0;

    // NOTE: This is super ugly and repetitive code but I'm too lazy to do anything about it right now 🤷
    while oxygen_generator_rating.len() > 1 {
        let one_count = &oxygen_generator_rating.iter().filter(|x| x[i] == 1).count();
        let zero_count = &oxygen_generator_rating.iter().filter(|x| x[i] == 0).count();
        if one_count >= zero_count {
            oxygen_generator_rating = oxygen_generator_rating.into_iter().filter(|x| x[i] != 0).collect();
        } else {
            oxygen_generator_rating = oxygen_generator_rating.into_iter().filter(|x| x[i] == 0).collect();
        }
        i += 1;
    }
    let mut i = 0;
    while co2_scrubber_rating.len() > 1 {
        let one_count = &co2_scrubber_rating.iter().filter(|x| x[i] == 1).count();
        let zero_count = &co2_scrubber_rating.iter().filter(|x| x[i] == 0).count();
        if one_count >= zero_count {
            co2_scrubber_rating = co2_scrubber_rating.into_iter().filter(|x| x[i] != 1).collect();
        } else {
            co2_scrubber_rating = co2_scrubber_rating.into_iter().filter(|x| x[i] == 1).collect();
        }
        i += 1;
    }
    let ox_str: String = oxygen_generator_rating[0].iter().map( |x| (x).to_string()).collect();
    let co_2_str: String = co2_scrubber_rating[0].iter().map( |x| (x).to_string()).collect();
    i32::from_str_radix(&ox_str, 2).unwrap() * i32::from_str_radix(&co_2_str, 2).unwrap()
}

pub fn solve() {
    let solution1 = solve1();
    let solution2 = solve2();

    println!("Day3:\n1. {}\n2. {}", solution1, solution2);
}