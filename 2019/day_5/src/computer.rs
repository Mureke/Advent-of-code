use std::fmt;

enum Mode {
    Position,
    Immediate,
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Computer {
    pointer: usize,
    pub memory: Vec<i32>,

    input: Vec<i32>,
    output: Vec<i32>,
}

impl Computer {
    pub fn new(memory: Vec<i32>, input: Vec<i32>) -> Self{
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
        let asd = num_vec.get(parameter_index+2);
        match num_vec.get(parameter_index+2) {
            Some(&1) => { Mode::Immediate },
            None => { Mode::Position },
            _ => { Mode::Position}
        }
    }

    fn get_parameter(&self, index: usize)->i32 {
        let mode = self.get_mode(index);
        let val_index = self.pointer + index;

        match mode {
            Mode::Position => {
                self.memory[self.memory[val_index] as usize]
            },
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

    pub fn execute(&mut self){
        
        loop {
            let pointer = self.pointer;
            let opcode = self.get_opcode();

            let steps = match opcode {
                1 => {
                    let (val1, val2) = (self.get_parameter(1), self.get_parameter(2));
                    self.memory[(self.pointer + 3) as usize] = val1 + val2;
                    4
                },
                2 => {
                    let (val1, val2) = (self.get_parameter(1), self.get_parameter(2));
                    self.memory[(self.pointer + 3) as usize] = val1 * val2;
                    4
                },
                3 => {
                    let address = self.get_address(1);
                    self.memory[address] = self.input.remove(0);
                    2
                },
                99 => { break; },
                _ => {panic!("Invalid opcode error! Opcode: {}, Pointer {}", opcode, self.pointer)}
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