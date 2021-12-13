use std::cmp::{min, max};
use crate::lib::file_reader::read_file;
use crate::lib::file_reader::parse_input;



fn get_answer1(mut values: Vec<i32>) -> i64 {
    let min = *values.iter().min().unwrap();
    let max = *values.iter().max().unwrap();

    let mut fuel = None;

    for i in min..max {
        let mut temp_fuel = 0;
        for val in values.iter() {
            temp_fuel += (i - val).abs()
        }
        if fuel.is_none() {
            fuel = Some(temp_fuel)
        }

        if temp_fuel < fuel.unwrap() {
            fuel = Some(temp_fuel)
        }
    }



    (fuel.unwrap()) as i64
}

fn get_answer2(mut values: Vec<i32>) -> i64 {
    let min = *values.iter().min().unwrap();
    let max = *values.iter().max().unwrap();

    let mut fuel = None;

    for i in min..max {
        let mut temp_fuel = 0;
        for val in values.iter() {
            let steps = (i - val).abs();
            for i in 1..steps+1 {
                temp_fuel += i;
            }
        }
        if fuel.is_none() {
            fuel = Some(temp_fuel)
        }

        if temp_fuel < fuel.unwrap() {
            fuel = Some(temp_fuel)
        }
    }



    (fuel.unwrap()) as i64
}

pub fn solve() {
    let input: String = read_file("day_7.txt");
    let mut values: Vec<i32> = input.trim().split(',').map(|s| s.parse().unwrap()).collect();

    let mut res1 = get_answer1(values.clone());
    let mut res2 = get_answer2(values.clone());

    println!("Day 7:\n1: {} 2:{}", res1, res2)
}