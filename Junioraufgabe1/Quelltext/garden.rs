use std::fs;
use std::ops::Range;
use crate::utils::read_int;


#[derive(Debug)]
pub struct Garden {
    interested: i64,
    height: i64,
    width: i64,
}

impl Garden {
    pub fn from_file(path: String) -> Garden {
        println!("{}", path);
        let file: String = match fs::read_to_string(path) {
            Ok(f) =>f,
            Err(_err) => panic!("File not found or wrong encoding."),
        };

        //split the file content into a vector of strings
        let file_content: Vec<&str> = file.trim().split("\n").collect::<Vec<&str>>();

        //create struct and parse string values to int
        Garden {
            interested: int_from_str(file_content[0]),
            height:  int_from_str(file_content[1]),
            width:  int_from_str(file_content[2])
        }
    }

    pub fn from_input() -> Garden {
        Garden {
            interested: read_int("Gib die Anzahl an Interesenten an:"),
            height: read_int("Gib die Höhe des Grundstückes an:"),
            width: read_int("Gib die Breite des Grundstückes an:")
        }
    }
    
}

fn int_from_str(input:&str) -> i64 {
    match input.trim().parse::<i64>() {
        Ok(ok) => ok,
        Err(_e) => panic!("File is broken couldn't parse {}.", input),
    }
}