use crate::lib::file_reader::read_file;
use crate::lib::file_reader::parse_input;

#[derive(Debug, Clone)]
struct Board {
    values: [[usize; 5]; 5],
    marked: [[bool; 5]; 5],

}

impl Board {
    fn new(input: &[String]) -> Self {
        let mut values = [[0; 5]; 5];
        for (i, row) in input.iter().enumerate() {
            let cols: Vec<usize> = row
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            for (j, val) in cols.iter().enumerate() {
                values[i][j] = *val;
            }
        }
        Board {
            values,
            marked: [[false; 5]; 5],
        }
    }
    fn mark_number(&mut self, number: usize) {
        for row in 0..5 {
            for col in 0..5 {
                if self.values[row][col] == number {
                    self.marked[row][col] = true;
                }
            }
        }
    }

    fn check_winner(&self) -> bool {
        // check rows
        for row in 0..5 {
            if self.marked[row].iter().filter(|&m| *m).count() == 5 {
                return true;
            }
        }
        for col in 0..5 {
            if self.marked.iter().filter(|&row| row[col]).count() == 5 {
                return true;
            }
        }
        false
    }

    fn calculate_result(&mut self, sum: usize) -> i32 {
        let mut result = 0;
        for row in 0..5 {
            for col in 0..5 {
                if self.marked[row][col] == false {
                    result += self.values[row][col];
                }
            }
        };
        (result * sum) as i32
    }
}

fn solve1(bingo_numbers: Vec<usize>, input: Vec<String>) -> i32 {
    let mut boards: Vec<Board> = input[1..].chunks(5).map(|c| Board::new(c)).collect();
    let mut winner: Option<Board> = None;
    let mut last_num1 = 0;

    for number in bingo_numbers {
        boards = boards.iter().map(|mut x| {
            let mut boardc = x.clone();
            boardc.mark_number(number);
            if boardc.check_winner() {
                winner = Some(boardc.clone());
                last_num1 = number;
            }
            return boardc;
        }).collect();


        if winner.is_some() {
            break;
        }
    }
    winner.unwrap().calculate_result(last_num1)
}

fn solve2(bingo_numbers: Vec<usize>, input: Vec<String>) -> i32 {
    let mut boards: Vec<Board> = input[1..].chunks(5).map(|c| Board::new(c)).collect();
    let mut winner: Option<Board> = None;
    let mut last_num1 = 0;

    for number in bingo_numbers {
        boards = boards.iter().map(|mut x| {
            let mut boardc = x.clone();
            boardc.mark_number(number);
            return boardc;
        }).collect();

        if boards.len() > 1 {
            boards = boards.into_iter().filter(|x| !x.check_winner()).collect();
        }

        if boards.len() == 1 {
            let temp_winner = boards.get(0).unwrap().to_owned();
            winner = Some(temp_winner.clone());
            if temp_winner.check_winner() {
                last_num1 = number;
                break;
            }
        }

    }

    winner.unwrap().calculate_result(last_num1)
}

pub fn solve() {
    let mut input: Vec<String> = parse_input(read_file("day_4.txt"));
    let bingo_numbers_str: String = input.get(0).unwrap().parse().unwrap();
    let bingo_numbers: Vec<usize> = bingo_numbers_str.trim().split(',').map(|s| s.parse().unwrap()).collect();
    input = input.into_iter().filter(|l| l != &"").collect();


    println!("Day 4:\n1. {}\n2. {}", solve1(bingo_numbers.clone(), input.clone()), solve2(bingo_numbers, input));
}