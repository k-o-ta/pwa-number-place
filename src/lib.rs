mod utils;

use sudoku::{
    parse_errors::{BlockParseError, LineParseError},
    Sudoku as ISudoku,
};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("hoge");
}

#[wasm_bindgen]
pub struct Sudoku {
    problem: String,
    answer: String,
    problems: Vec<Option<u32>>,
    answers: Vec<u32>,
    fixed_indices: Vec<usize>,
}

#[wasm_bindgen]
impl Sudoku {
    pub fn new() -> Sudoku {
        let sudoku = ISudoku::generate_filled();
        let sudoku_str = "\
___|2__|_63
3__|__5|4_1
__1|__3|98_
___|___|_9_
___|538|___
_3_|___|___
_26|3__|5__
5_3|7__|__8
47_|__1|___";
        let sudoku_str =
            "....1....46....9.....39.867.....5.2.8...72..12.5.........63....7........13....27.";
        // let sudoku = Sudoku::from_str_line(sudoku_str).unwrap();
        let mut problem = String::from("");
        let mut answer = String::from("");
        if let Ok(sudoku) = ISudoku::from_str_line(sudoku_str) {
            // if let Some(solution) = sudoku.solve_unique() {
            //     println!("{}", solution);
            // }
            problem = String::from(sudoku.to_string());
            answer = String::from("ok");
        } else {
            problem = String::from("no");
            answer = String::from("no");
        }

        match ISudoku::from_str_line(sudoku_str) {
            Ok(sudoku) => {
                problem = String::from(sudoku.to_string());
                answer = String::from("ok");
            }
            Err(e) => match e {
                /// Accepted values are numbers 1...9 and '0', '.' or '_' for empty cells
                LineParseError::InvalidEntry(entry) => {
                    problem = String::from("0");
                }
                LineParseError::NotEnoughCells(num) => {
                    problem = String::from("1");
                }
                LineParseError::TooManyCells => {
                    problem = String::from("2");
                }
                LineParseError::MissingCommentDelimiter => {
                    problem = String::from("3");
                }
            },
        }
        let sudoku = ISudoku::generate_unique();
        let problem = sudoku.to_string();
        let problems = sudoku
            .to_string()
            .chars()
            .map(|c| match c.to_digit(10) {
                Some(num) if (1 <= num && num < 10) => Some(num),
                _ => None,
            })
            .collect();
        let mut fixed_indices = Vec::new();
        for (index, num) in sudoku.to_string().chars().enumerate() {
            if num.is_digit(10) {
                fixed_indices.push(index);
            }
        }
        let answers = sudoku
            .solve_one()
            .expect("unsolvable problem")
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).expect("none digit number"))
            .collect();
        // let problem = sudoku.to_string();
        Sudoku {
            problem,
            answer,
            problems,
            answers,
            fixed_indices,
        }
    }

    pub fn problem(&self) -> String {
        self.problem.to_string()
    }
    pub fn answer(&self) -> String {
        self.answer.to_string()
    }
    pub fn get_num(&self, index: usize) -> Option<u32> {
        self.problems[index]
    }
    pub fn set_num(&mut self, index: usize, num: u32) -> bool {
        if self.fixed_indices.contains(&index) {
            return false;
        } else {
            self.problems[index] = Some(num);
            return true;
        }
    }
    pub fn is_fixed(&self, index: usize) -> bool {
        self.fixed_indices.contains(&index)
    }
}
