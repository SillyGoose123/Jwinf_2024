use std::env::args;
use std::panic;
use std::process::exit;
use crate::garden::Garden;

mod garden;
mod utils;

fn main() {
    //custom error handling
    panic::set_hook(Box::new(|info| {
        //print the error
        eprintln!("Error: {}", &info.to_string().split("\n").collect::<Vec<&str>>()[1]);

        // kill the program with exit code 1
        exit(1);
    }));

    let args: Vec<String> = args().collect::<Vec<String>>();

    let garden = if args.len() > 1 {
        Garden::from_file(args[1].clone())
    } else {
        Garden::from_input()
    };

    let field = garden.calc();
    println!("{} Felder mit den Abmessungen: {}m (Breite) x {}m (Höhe)", garden.interested, field.0, field.1);
}
