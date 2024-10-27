use std::io::Write;

pub fn from_input() -> String {
    print!("Enter the text\n>");

    //clear the tests buffer so the print combined with the input above works
    std::io::stdout().flush().unwrap();

    //create a string for the input
    let mut input = String::new();

    //read the input
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");

    input
}

pub fn calc_jump_width(c: &char) -> usize {
    // umlauts are not in the ascii table so we have to handle them separately
    match c {
        'ä' => 27,
        'ö' => 28,
        'ü' => 29,
        'ß' => 30,
        _=> c.to_ascii_lowercase() as usize - 96
    }
}