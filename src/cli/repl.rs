pub mod input;
pub mod line;
pub mod lexer;

use input::session::{Input};

pub fn activate() -> std::io::Result<()> { 
    
    let mut input_session = Input::new().unwrap();

    loop {
        
        match input_session.next_line()? {
            Some(line) => {
                println!("Line Entered: {}", line);
            },
            None => {
                println!("End - Eof");
                break
            }
        }
    }

    Ok(())

}