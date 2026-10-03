pub mod input;
pub mod line;
pub mod lexer;
pub mod parser;
pub mod executor;
pub mod frame;

use input::session::{Input};
use lexer::tokenizer::{TokenCollector};
use parser::pipeline::{Parser};
use executor::mecha::{Executor};
use frame::{
    mold,//::{Frame},
    trace,//::{completion, tokens, parsed, execute, error, eof}
};

// pub fn activate() -> std::io::Result<()> { 
    
//     let mut input_session = Input::new().unwrap();
//     let mut token_collector = TokenCollector::new();
//     let executor = Executor::new();

//     loop {
        
//         match input_session.next_line()? {
//             Some(line) => {
//                 println!("Line Entered: {}", line);

//                 let tokens = token_collector.iter_line(line)?;
//                 println!("Tokens: {:?}", tokens);

//                 let mut parser = Parser::new(tokens);

//                 match parser.parse() {
//                     Ok(ast) => {
//                         println!("{ast:#?}");

//                         if let Err(err) = executor.execute(ast) {
//                             eprintln!("Erro de execução: {err:?}");
//                         }
//                     }

//                     Err(err) => {
//                         eprintln!("Erro de sintaxe: {err:?}");
//                     }
//                 }
//             },
//             None => {
//                 println!("End - Eof");
//                 break
//             }
//         }
//     }

//     Ok(())

// }


pub fn activate() -> std::io::Result<()> {

    let mut input_session = Input::new().unwrap();
    let mut token_collector = TokenCollector::new();
    let executor = Executor::new();

    mold::banner();

    loop {

        let Some(line) = input_session.next_line()? else {
            trace::eof();
            break;
        };

        let mut frame = mold::Frame::new(format!("$ {line}"));
        for (
            typed, 
            done
        ) in input_session.take_completions() {
            trace::completion(
                &mut frame, 
                &typed, 
                &done
            );
        }

        let tokens = token_collector.iter_line(line)?;
        trace::tokens(&mut frame, &tokens);

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(ast) => {
                trace::parsed(&mut frame, &ast);
                trace::execute(&mut frame);
                match executor.execute(ast) {
                    Ok(output) => frame.text(&output),
                    Err(err) => trace::error(&mut frame, "execution error", &err),
                }
            }
            Err(err) => trace::error(&mut frame, "syntax error", &err),
        }

        frame.print();
    }

    Ok(())
}
