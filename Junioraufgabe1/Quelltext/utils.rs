use std::io::Write;

// a function to read an int from the user
pub fn read_int(message: &str) -> i64 {
    println!("{} ", message);
    loop {
        // for ux (user experience)
        print!("> ");

        //clear the tests buffer so the print combined with the input above works
        std::io::stdout().flush().unwrap();

        //create a string for the input
        let mut input = String::new();

        //read the input
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        //check if it is parseable into an i64 if not ask again
        match input.trim().parse::<i64>() {
            Ok(ok) => return ok,
            Err(_e) => println!("Bitte gib eine Dezimalzahl ein:"),
        }
    }
}


pub fn int_from_str(input:&str) -> i64 {
    match input.trim().parse::<i64>() {
        Ok(ok) => ok,
        Err(_e) => panic!("File is broken couldn't parse {}.", input),
    }
}

pub fn round_up_to_even(i: i64) -> i64 {
    if i % 2 == 0 {
        i
    } else {
        i + 1
    }
}

pub fn is_more_squared(current: f64, last: f64) -> bool {
    if current == last {
        return true;
    }
    (1.0 - current).abs() < (1.0 - last).abs()
}
