
use crate::lib::file_reader::parse_input;
use crate::lib::file_reader::read_file;

fn get_answer1(mut values: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for (key, val) in values.iter().enumerate() {
        for (k, v) in val.iter().enumerate() {
            let left = if k > 0 {val.get(k-1) } else { None };
            let right = if k+1 < val.len() {val.get(k+1) } else { None };
            let mut top: Option<&u32 > = None;
            let mut bot: Option<&u32 > = None;

            if key > 0 {
                let top_row = values.get(key-1).unwrap();
                top = top_row.get(k);
            }
            if key+1 <= values.len()-1 {
                let bot_row = values.get(key+1).unwrap();
                bot = bot_row.get(k);
            }

            let mut smaller = [false; 4];
            for (k, i) in [left, right, bot, top].iter().enumerate() {
                match i {
                    Some(i) => if i > &v { smaller[k] = true },
                    None => smaller[k] = true
                }
            }
            let smaller_count  = smaller.iter().filter(|x| **x).count();
            if smaller_count == 4 {
                sum += v+1
            }
        }
    }
    sum
}

fn get_answer2(mut values: Vec<Vec<&str>>) -> i32 {
    3
}

fn get_parsed_data() -> Vec<Vec<u32>> {
    let mut input: Vec<String> = parse_input(read_file("day_9.txt"));
    let mut matrix: Vec<Vec<u32>> = vec![];
    for i in input.iter() {
        let str_as_vec: Option<Vec<u32>> = i.chars().map(|ch| ch.to_digit(10)).collect();
        matrix.append(&mut vec![str_as_vec.unwrap()]);
    }
    return matrix;
}
pub fn solve() {
    let values = get_parsed_data();

    //
    let mut res1 = get_answer1(values.clone());
    // let mut res2 = get_answer2(values.clone());

    println!("Day 7:\n1: {} 2:{}", res1, 3)
}