use crate::lib::file_reader::read_file;
use crate::lib::file_reader::parse_input;

struct SubMarine {
    depth: i32,
    horizontal: i32,
    aim: i32
}

impl SubMarine {

    fn new() -> Self {
        SubMarine {
            horizontal: 0,
            depth: 0,
            aim: 0
        }
    }

    pub fn go_forward(&mut self, amount: i32) {
        self.horizontal += amount;
        self.depth += (amount*self.aim);
    }

    pub fn go_down(&mut self, amount: i32) {
        self.adjust_aim(amount);
    }

    pub fn go_up(&mut self, amount: i32) {
        self.adjust_aim(-amount);
    }

    fn adjust_aim(&mut self, amount: i32) {
        self.aim += amount;
    }

    pub fn get_answer(&self) -> i32 {
        self.horizontal * self.depth
    }
}

pub fn solve() {
    let input: Vec<String> = parse_input(read_file("day_2.txt"));
    let mut depth = 0;
    let mut horizontal = 0;

    let mut part2_sub = SubMarine::new();

    for direction in input.iter() {
        let parsed: Vec<&str> = direction.split_whitespace().collect();
        let dir = parsed.get(0).unwrap();
        let amount: i32 = parsed.get(1).unwrap().parse().unwrap();

        if dir == &"forward" {
            horizontal += amount;
            part2_sub.go_forward(amount);

        } else if dir == &"down" {
            depth += amount;
            part2_sub.go_down(amount);

        } else {
            depth -= amount;
            part2_sub.go_up(amount);
        }
    }

    println!("Day 2: \n1. {}\n2. {}", horizontal*depth, part2_sub.get_answer());

}