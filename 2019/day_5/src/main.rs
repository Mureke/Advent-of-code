use std::iter::FromIterator;

mod utils;

fn main() {
    let data: Vec<i32> = vec![3, 1, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 11, 91, 225, 1002, 121, 77, 224, 101, -6314, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 3, 224, 1, 223, 224, 223, 1102, 74, 62, 225, 1102, 82, 7, 224, 1001, 224, -574, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 1101, 28, 67, 225, 1102, 42, 15, 225, 2, 196, 96, 224, 101, -4446, 224, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223, 1101, 86, 57, 225, 1, 148, 69, 224, 1001, 224, -77, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 1101, 82, 83, 225, 101, 87, 14, 224, 1001, 224, -178, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224, 224, 1, 223, 224, 223, 1101, 38, 35, 225, 102, 31, 65, 224, 1001, 224, -868, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5, 224, 1, 223, 224, 223, 1101, 57, 27, 224, 1001, 224, -84, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7, 224, 1, 223, 224, 223, 1101, 61, 78, 225, 1001, 40, 27, 224, 101, -89, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 1, 224, 1, 224, 223, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1008, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 329, 101, 1, 223, 223, 8, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 344, 101, 1, 223, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 359, 101, 1, 223, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 374, 101, 1, 223, 223, 7, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 389, 1001, 223, 1, 223, 108, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 404, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 419, 1001, 223, 1, 223, 1107, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 434, 1001, 223, 1, 223, 1108, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 449, 1001, 223, 1, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 464, 101, 1, 223, 223, 1008, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 479, 101, 1, 223, 223, 1007, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 494, 101, 1, 223, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 509, 101, 1, 223, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 524, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 539, 101, 1, 223, 223, 107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 554, 1001, 223, 1, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 569, 1001, 223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 584, 101, 1, 223, 223, 1107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 599, 101, 1, 223, 223, 1108, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 614, 101, 1, 223, 223, 8, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 629, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 644, 1001, 223, 1, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 659, 101, 1, 223, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226];


    let mut values: Vec<i32> = data.clone();

    let mut i: usize = 0;

    'inner2: loop {
        let (mut instruction_vector, number_count) = utils::number_to_vec(values[i]);
        instruction_vector = utils::left_pad_with_zeros(instruction_vector, 5 - number_count);

        let instruction_length = instruction_vector.len();
        let instruction = format!("{}{}", instruction_vector[instruction_length - 2], instruction_vector[instruction_length - 1]).parse().unwrap();
        let modes: Vec<i32> = Vec::from_iter(instruction_vector[0..(instruction_length - 2)].iter().cloned());

        match instruction {
            1 => { values = handle_instructions(values, i, 1, modes) }
            2 => { values = handle_instructions(values, i, 2, modes) }
            3 => { values = handle_instructions(values, i, 3, modes) }
            4 => { values = handle_instructions(values, i, 4, modes) }
            99 => {println!("Halting the program"); break 'inner2; }
            _ => println!("error!")
        }
        // Instructions 3 and 4 only take one parameter. Therefore we only increase index by 2
        if instruction == 3 || instruction == 4 {
            i += 2
        } else {
            i += 4;
        }
    }
}


fn get_value_by_mode(values: &Vec<i32>, i: usize, m: i32, val_index: usize) -> i32 {
    // Gets correct value from value vector based on mode
    let mut value: i32 = 0;
    if m == 0 {
        value = values[values[i + val_index] as usize];
    } else if m == 1 {
        value = values[i + val_index];
    }
    value
}

fn handle_instructions(mut values: Vec<i32>, i: usize, t: i8, modes: Vec<i32>) -> Vec<i32> {
    let modes_length = modes.len();
    let mut val2 = 0;
    let val1 = get_value_by_mode(&values, i, modes[modes_length - 1], 1);
    if t != 4 {
        val2 = get_value_by_mode(&values, i, modes[modes_length - 2], 2);
    }
    let mut changed_pos = values[i + 3] as usize;
    if modes[modes_length - 3] == 1 {
        changed_pos = values[i + 1] as usize;
    }
    // Handle single input when t = 3
    if t == 1 {
        // opcode 01 = sum
        values[changed_pos] = val1 + val2;
    } else if t == 2 {
        // opcode 02 = multiply
        values[changed_pos] = val1 * val2;
    } else if t == 3 {
        changed_pos = values[i + 1] as usize;
        values[changed_pos] = val1;
    } else if t == 4 {
        // opcode 04
        changed_pos = values[i + 1] as usize;
        println!("ERROR CODE: {}", values[changed_pos])
    }
    values
}