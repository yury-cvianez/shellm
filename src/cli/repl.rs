pub mod input;
pub mod line;
pub mod lexer;

use input::session::{Input};
use lexer::tokenizer::{TokenCollector};

pub fn activate() -> std::io::Result<()> { 
    
    let mut input_session = Input::new().unwrap();
    let mut token_collector = TokenCollector::new();

    loop {
        
        match input_session.next_line()? {
            Some(line) => {
                println!("Line Entered: {}", line);

                let token = token_collector.iter_line(line);
                println!("Tokens: {:?}", token);
            },
            None => {
                println!("End - Eof");
                break
            }
        }
    }

    Ok(())

}