use std::fs;
use crate::utils::{int_from_str, is_more_squared, read_int, round_up_to_even};


#[derive(Debug)]
pub struct Garden {
    interested: i64,
    height: i64,
    width: i64,
}

impl Garden {
    pub fn new(interested: i64, height: i64, width: i64) -> Garden {
        // round the number of interested persons up to an even number
        Garden {
            interested: round_up_to_even(interested),
            height,
            width,
        }
    }

    pub fn from_file(path: String) -> Garden {
        let file: String = match fs::read_to_string(path) {
            Ok(f) => f,
            Err(_err) => panic!("File not found or wrong encoding."),
        };

        //split the file content into a vector of strings
        let file_content: Vec<&str> = file.trim().split("\n").collect::<Vec<&str>>();

        //create struct and parse string values to int
        Garden::new(int_from_str(file_content[0]), int_from_str(file_content[1]), int_from_str(file_content[2]))
    }

    pub fn from_input() -> Garden {
        Garden::new(read_int("Gib die Anzahl an Interesenten an:"),
                    read_int("Gib die Höhe des Grundstückes an:"),
                    read_int("Gib die Breite des Grundstückes an:"))
    }

    fn calc_square_size(&self, vertical_lines: i64) -> i64 {
        println!("({} / {}) / ({} / {}) = {}", self.width,self.calc_lines(vertical_lines), self.height, vertical_lines, (self.width / self.calc_lines(vertical_lines)) / (self.height / vertical_lines));
        (self.width / self.calc_lines(vertical_lines)) / (self.height / vertical_lines)
    }

    fn calc_lines(&self, vertical_lines: i64) -> i64 {
        println!("{} / {} = {}", self.interested - 1, vertical_lines, ((self.interested - 1) / vertical_lines));
       (self.interested - 1) / vertical_lines
    }

    pub fn calc(&self) -> (i64, i64) {



        let mut last: i64 = 0;
        for i in 1..self.interested {
            let cur = self.calc_square_size(i);

            if !is_more_squared(cur, last) {
                return (i - 1, self.calc_lines(i - 1));
            }

            last = cur;
        }

        (0, 0)
    }
}



