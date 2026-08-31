use std::io::{stdin};

pub fn read_line() -> String {
    // read input the user typed in the shell

    let mut input = String::new();
    
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.to_string();
}