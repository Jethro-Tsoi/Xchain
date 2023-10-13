use std::{process, io};
use crate::menu::Menu;

pub struct GetInput{
}

impl GetInput {

    pub fn get_input_line (&self) -> String {
        let mut input = String::new();
        if input == "exit" {
            process::exit(1);
        } else if input == "menu" {
            Menu.execute();
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
}    