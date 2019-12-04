
fn main() {
    let data: Vec<i32> = vec![1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 2, 9, 19, 23, 2, 13, 23, 27, 1, 6, 27, 31, 2, 6, 31, 35, 2, 13, 35, 39, 1, 39, 10, 43, 2, 43, 13, 47, 1, 9, 47, 51, 1, 51, 13, 55, 1, 55, 13, 59, 2, 59, 13, 63, 1, 63, 6, 67, 2, 6, 67, 71, 1, 5, 71, 75, 2, 6, 75, 79, 1, 5, 79, 83, 2, 83, 6, 87, 1, 5, 87, 91, 1, 6, 91, 95, 2, 95, 6, 99, 1, 5, 99, 103, 1, 6, 103, 107, 1, 107, 2, 111, 1, 111, 5, 0, 99, 2, 14, 0, 0];
    let mut second_value = 0;
    'outer: loop {
        let mut first_value: i32 = 0;
        'inner1: loop {
            let mut values: Vec<i32> = data.clone();
            values[1] = first_value;
            values[2] = second_value;
            let mut i: usize = 0;

            'inner2: loop {
                match values[i] {
                    1 => { values = handle(values, i, 1) }
                    2 => { values = handle(values, i, 2) }
                    99 => { break 'inner2; }
                    _ => println!("error!")
                }
                i += 4;
            }

            if values[0] == 19690720 {
                println!("Answer to 2.2 = {}", 100 * first_value + second_value);
                break 'outer;
            }
            // increase first value and break if > 99
            first_value += 1;
            if first_value > 99 {
                break 'inner1;
            }
        }

        // increase 2nd value and break if > 99
        second_value += 1;
        if second_value > 99 {
            println!("Error! Couldn't find the answer");
            break 'outer;
        }
    }
}

fn handle(mut values: Vec<i32>, i: usize, t: i8) -> Vec<i32> {
    let pos1: usize = values[i + 1] as usize;
    let pos2: usize = values[i + 2] as usize;
    let index: usize = values[i + 3] as usize;

    if t == 1 {
        values[index] = values[pos1] + values[pos2];
    } else if t == 2 {
        values[index] = values[pos1] * values[pos2];
    }
    values
}

