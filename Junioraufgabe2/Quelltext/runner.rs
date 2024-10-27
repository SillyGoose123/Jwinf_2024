use std::process::exit;
use crate::utils::jump_width;

#[derive(Clone)]
pub struct Runner {
    player_name: String,
    characters: Vec<char>,
    pos: usize,
}

impl Runner {
    pub fn new(start_position: usize, player_name: &str, characters: &Vec<char>) -> Runner {
            Runner {
            player_name: player_name.to_string(),
            characters: characters.clone(),
            pos: start_position,
        }
    }
    
    pub fn hop(&mut self) {
        let new_pos = match &self.characters.get(self.pos) {
            Some(c) => jump_width(c),
            None => {
                println!("The Winner is: {}", &self.player_name);
                exit(0);
            }
        };

        self.pos += new_pos;
    }
}