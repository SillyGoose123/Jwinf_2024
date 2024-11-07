use std::fs;
use crate::utils::{int_from_str, is_more_squared, read_int, round_up_to_even};


#[derive(Debug)]
pub struct Garden {
    interested: i64,
    height: f64,
    width: f64,
}

impl Garden {
    pub fn new(interested: i64, height: i64, width: i64) -> Garden {
        // round the number of interested persons up to an even number
        Garden {
            interested: round_up_to_even(interested),
            height: height as f64,
            width: width as f64,
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

    fn calc_square_size(&self, vertical_lines: f64) -> f64 {
        let new_horizontal_lines: f64 = self.interested as f64 / vertical_lines;

        print!("({} / {}) / ({} / {}) = ", self.width, new_horizontal_lines, self.height, vertical_lines);

        print!("{} / {} = ", self.width / new_horizontal_lines, self.height / vertical_lines);


        println!("{}", (self.width  / new_horizontal_lines) / self.height / vertical_lines);

        (self.width / new_horizontal_lines) / (self.height / vertical_lines)
    }

    pub fn calculate_area(&self) -> (i64, i64) {
        //find the most square layout
        self.calc_most_square_layout(0, 0.0)
    }


    fn calc_most_square_layout(&self, last_vertical_lines: i64, last_square_diff: f64) -> (i64, i64) {
        let new_vertical_lines = last_vertical_lines + 1;

        let new_square_diff = self.calc_square_size(new_vertical_lines as f64);

        if is_more_squared(new_square_diff, last_square_diff) {
            self.calc_most_square_layout(new_vertical_lines, new_square_diff)
        } else {
            (
                last_vertical_lines,
                self.interested / last_vertical_lines
            )
        }
    }
}



