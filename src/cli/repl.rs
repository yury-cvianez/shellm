pub mod input;
pub mod line;


pub fn activate() -> std::io::Result<()> { 
    
    let mut input_session = input::Input::new().unwrap();

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