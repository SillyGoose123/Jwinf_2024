mod utils;
mod runner;

use std::env::args;
use std::{fs, panic, vec};
use std::process::exit;
use crate::runner::Runner;
use crate::utils::from_input;

fn main() {
    //custom error handling
    panic::set_hook(Box::new(|info| {
        //print the error
        eprintln!("Error: {}", &info.to_string().split("\n").collect::<Vec<&str>>()[1]);

        // kill the program with exit code 1
        exit(1);
    }));

    //get text from stdin or a file
    let args: Vec<String> = args().collect::<Vec<String>>();
    let text = if args.len() > 1 {
        match fs::read_to_string(args[1].clone()) {
            Ok(f) => f,
            Err(_err) => panic!("File not found or wrong encoding."),
        }
    } else {
        from_input()
    };

    //remove everything that is not a letter
    let characters: Vec<char> = text.chars().filter(|c| c.is_alphabetic()).collect();

    //create a runner for each player
    let mut runners: Vec<Runner> = vec![
        Runner::new(0, "Bela", &characters),
        Runner::new(1, "Amira", &characters),
    ];

    //let one hop after the other
    loop {
        for runner in &mut runners {
            // hop the winner will exit the program and print his name
            runner.hop();
        }
    }
}
