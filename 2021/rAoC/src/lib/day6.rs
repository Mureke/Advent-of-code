use crate::lib::file_reader::read_file;
use crate::lib::file_reader::parse_input;

// fn count_fishes(mut values: Vec<i32>, days: i32) -> usize {
//     for i in 0..days {
//         for key in 0..values.iter().len() {
//             if values[key] == 0 {
//                 values[key] = 6;
//                 values.push(8)
//             } else {
//                 values[key] -= 1
//             }
//         }
//     }
//     values.len()
// }


fn count_fishes2(mut values: Vec<i32>, days: i32) -> i64 {
    let mut data: [usize; 9] = [0; 9];
    for val in values {
        data[val as usize] += 1
    }
    for _ in 0..days {
        let first = data[0];
        let temp = data.clone();
        for (key, _) in temp.iter().enumerate() {
            if key < 8 {
                data[key] = data[key + 1]
            }
        }
        data[8] = first;
        data[6] += first;


    }
    let mut sum: i64 = 0;
    for i in data {
        sum += i as i64;
    }
    sum
}

pub fn solve() {
    let input: String = read_file("day_6.txt");
    let mut values: Vec<i32> = input.trim().split(',').map(|s| s.parse().unwrap()).collect();

    let mut res1 = count_fishes2(values.clone(), 80);
    let mut res2 = count_fishes2(values.clone(), 256);

    println!("Day 6:\n1: {} 2: {}", res1, res2)
}