use std::fmt::{Display, Formatter};
use std::fs;
use crate::utils::{int_from_str, is_more_squared, read_int, round_up_to_even};


#[derive(Debug)]
pub struct Garden {
    pub interested: i64,
    height: i64,
    width: i64,
}

impl Garden {
    pub fn new(interested: i64, height: i64, width: i64) -> Garden {
        //print exercise
        println!("{} Interessenten mit Grundstück: {}m (Breite) x {}m (Höhe)", interested, width, height);
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
        Garden::new(read_int("Gib die Anzahl an Interessenten an:"),
                    read_int("Gib die Höhe des Grundstückes an:"),
                    read_int("Gib die Breite des Grundstückes an:"))
    }

    fn get_horizontal_lines(&self, vertical_lines: i64) -> i64 {
        (self.interested / (vertical_lines + 1)) - 1
    }

    fn calc_square_size(&self, vertical_lines: i64, horizontal_lines: i64) -> f64 {
        if horizontal_lines == 0 {
            return 0.0;
        }

        (self.width as f64 / vertical_lines as f64) / (self.height as f64 / horizontal_lines as f64)
    }

    fn find_best_division(&self) -> (i64, i64){
        let mut best_square: f64 = 0.0;
        let mut best_lines = (0, 0);

        for i in 1..self.interested {
            let horizontal_lines = self.get_horizontal_lines(i);
            let square_size = self.calc_square_size(i, horizontal_lines);

            if is_more_squared(square_size, best_square){
                best_square = square_size;
                best_lines = (i, horizontal_lines);
            }
        }

        best_lines
    }

    pub fn calc(&self) -> (f64, f64){
        let best_lines = self.find_best_division();
        (self.width as f64 / (best_lines.0 + 1) as f64, self.height as f64 / (best_lines.1 + 1) as f64)
    }


}
