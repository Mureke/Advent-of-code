enum Mode {
    Position,
    Immediate,
}

pub struct Computer {
    pointer: usize,
    memory: Vec<i32>,

    input: Vec<i32>,
    pub output: Vec<i32>,
}

impl Computer {
    pub fn new(memory: Vec<i32>, input: Vec<i32>) -> Self {
        Self {
            pointer: 0,
            memory,
            input,
            output: Vec::new(),
        }
    }

    fn get_mode(&self, parameter_index: usize) -> Mode {
        let mut num_vec: Vec<u32> = self.memory[self.pointer]
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        num_vec.reverse();
        match num_vec.get(parameter_index + 1) {
            Some(&1) => {
                Mode::Immediate
            }
            None => {
                Mode::Position
            }
            _ => { Mode::Position }
        }
    }

    fn get_parameter(&self, index: usize) -> i32 {
        let mode = self.get_mode(index);
        let val_index = self.pointer + index;

        match mode {
            Mode::Position => {
                self.memory[self.memory[val_index] as usize]
            }
            Mode::Immediate => {
                self.memory[val_index]
            }
        }
    }
    fn get_address(&self, index: usize) -> usize {
        self.memory[self.pointer + index] as usize
    }

    fn get_opcode(&self) -> i32 {
        self.memory[self.pointer] % 100
    }

    pub fn execute(&mut self) {
        loop {
            let pointer = self.pointer;
            let opcode = self.get_opcode();

            let steps = match opcode {
                1 => {
                    // 1 = sum of two params
                    let (val1, val2) = (self.get_parameter(1), self.get_parameter(2));
                    let target = self.memory[pointer + 3];
                    self.memory[target as usize] = val1 + val2;
                    4
                }
                2 => {
                    // 2 = Multiply two params
                    let (val1, val2) = (self.get_parameter(1), self.get_parameter(2));
                    let target = self.memory[pointer + 3];
                    self.memory[target as usize] = val1 * val2;
                    4
                }
                3 => {
                    // Take an input and save it to position given by param
                    let address = self.get_address(1);
                    self.memory[address] = self.input.remove(0);
                    2
                }
                4 => {
                    // Output value (error code) of only param
                    self.output.push(self.get_parameter(1));
                    2
                }
                5 => {
                   // Jump-if-true
                    let (val1, val2) = (self.get_parameter(1), self.get_parameter(2));
                    if val1 != 0 {
                        self.pointer = val2 as usize;
                        0 // Increase steps by  0 if pointer index changed
                    }else {
                        3 // Increase steps by 3 if pointer index wasn't changed
                    }
                }
                6 => {
                    // Jump-if-false
                    let (val1, val2) = (self.get_parameter(1), self.get_parameter(2));
                    if val1 == 0 {
                        self.pointer = val2 as usize;
                        0 // Increase steps by  0 if pointer index changed
                    }else {
                        3 // Increase steps by 3 if pointer index wasn't changed
                    }
                }
                7 => {
                    // less-than
                    let (val1, val2) = (self.get_parameter(1), self.get_parameter(2));
                    if val1 < val2 {
                        let target = self.memory[pointer + 3];
                        self.memory[target as usize] = 1
                    } else {
                        let target = self.memory[pointer + 3];
                        self.memory[target as usize] = 0
                    }
                    4
                }
                8 => {
                    // equals
                    let (val1, val2) = (self.get_parameter(1), self.get_parameter(2));
                    if val1 == val2 {
                        let target = self.memory[pointer + 3];
                        self.memory[target as usize] = 1
                    } else {
                        let target = self.memory[pointer + 3];
                        self.memory[target as usize] = 0
                    }
                    4
                }
                99 => { break; }
                _ => { panic!("Invalid opcode error! Opcode: {}, Pointer {}", opcode, self.pointer) }
            };
            self.pointer += steps;
        }
    }
}

#[test]
fn test_halt() {
    let mut comp = Computer::new(vec![99, 3], vec![1]);
    comp.execute();
    assert_eq!(comp.pointer, 0);
}

#[test]
fn test_opcode_one() {
    let mut comp = Computer::new(vec![1, 2, 3, 0, 99, 1], vec![1]);
    comp.execute();
    assert_eq!(comp.memory[3], 3);
}

#[test]
fn test_opcode_two() {
    let mut comp = Computer::new(vec![2, 2, 3, 4, 99, 1], vec![1]);
    comp.execute();
    assert_eq!(comp.memory[3], 12);
}

#[test]
fn test_opcode_three() {
    let mut comp = Computer::new(vec![3, 1, 4, 0, 99], vec![4]);
    comp.execute();
    assert_eq!(comp.memory[1], 4);
}

#[test]
fn test_opcode_four_position_mode() {
    let mut comp = Computer::new(vec![3, 1, 4, 1, 99], vec![44]);
    comp.execute();
    assert_eq!( comp.output, vec![44]);
}

#[test]
fn test_opcode_four_immmediate_mode() {
    let mut comp = Computer::new(vec![3, 1, 104, 1, 99], vec![44]);
    comp.execute();
    assert_eq!( comp.output, vec![1]);
}

/// Jump-if-true
/// Should ony move pointer to position 6 and halt after that. Input doesn't matter.
#[test]
fn test_opcode_five_true() {
    let mut comp = Computer::new(vec![1105, 1, 6, 0, 01, 22, 99], vec![23]);
    comp.execute();
    assert_eq!(comp.pointer, 6);
}

/// Jump-if-false
/// Should ony move pointer to position 6 and halt after that. Input doesn't matter.
#[test]
fn test_opcode_six() {
    let mut comp = Computer::new(vec![1106, 0, 6, 0, 01, 22, 99], vec![23]);
    comp.execute();
    assert_eq!(comp.pointer, 6);
}

/// less-than
#[test]
fn test_opcode_seven() {
    let mut comp = Computer::new(vec![1107, 4, 5, 0, 99], vec![23]);
    comp.execute();
    assert_eq!(comp.memory[0], 1);

    let mut comp2 = Computer::new(vec![1107, 6, 5, 0, 99], vec![23]);
    comp2.execute();
    assert_eq!(comp2.memory[0], 0);
}


/// equals
#[test]
fn test_opcode_eight() {
    let mut comp = Computer::new(vec![1108, 5, 5, 0, 99], vec![23]);
    comp.execute();
    assert_eq!(comp.memory[0], 1);

    let mut comp2 = Computer::new(vec![1108, 6, 5, 0, 99], vec![23]);
    comp2.execute();
    assert_eq!(comp2.memory[0], 0);
}