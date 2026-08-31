mod cli;

use cli::input::{
    read_line,
};

fn main() {
    loop {
        let input = read_line();
        println!("entry: {}", input);
    }
}
