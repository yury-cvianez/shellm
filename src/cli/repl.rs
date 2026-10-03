pub mod input;
pub mod line;
pub mod lexer;
pub mod parser;
pub mod executor;

use input::session::{Input};
use lexer::tokenizer::{TokenCollector};
use parser::pipeline::{Parser};
use executor::mecha::{Executor};

pub fn activate() -> std::io::Result<()> { 
    
    let mut input_session = Input::new().unwrap();
    let mut token_collector = TokenCollector::new();
    let executor = Executor::new();

    loop {
        
        match input_session.next_line()? {
            Some(line) => {
                println!("Line Entered: {}", line);

                let tokens = token_collector.iter_line(line)?;
                println!("Tokens: {:?}", tokens);

                let mut parser = Parser::new(tokens);

                match parser.parse() {
                    Ok(ast) => {
                        println!("{ast:#?}");

                        if let Err(err) = executor.execute(ast) {
                            eprintln!("Erro de execução: {err:?}");
                        }
                    }

                    Err(err) => {
                        eprintln!("Erro de sintaxe: {err:?}");
                    }
                }
            },
            None => {
                println!("End - Eof");
                break
            }
        }
    }

    Ok(())

}