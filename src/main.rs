mod cli;

use cli::line::{
    read_line,
};

fn main() {
    loop {
        let input = read_line();
        println!("entry: {}", input);
    }
}
