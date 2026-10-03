use crate::cli::repl::parser::ast::{Command, Pipeline, Redirection, RedirectKind};
use crate::cli::repl::lexer::tokenizer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    ExpectedCommand,
    ExpectedRedirectionTarget,
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Pipeline, ParseError> {
        
        let mut commands = Vec::new();

        if self.tokens.peek().is_none() {
            return Err(ParseError::EmptyInput);
        }

        loop {
            let command = self.parse_command()?;
            commands.push(command);

            match self.tokens.peek() {
                Some(Token::Pipe) => {
                    self.tokens.next();
                }
                None => break,
                _ => unreachable!("parse_command deve consumir o comando inteiro"),
            }
        }

        Ok(Pipeline { commands })
    }

    fn parse_command(&mut self) -> Result<Command, ParseError> {

        let program = match self.tokens.next() {
            Some(Token::Word(word)) => word,
            _ => return Err(ParseError::ExpectedCommand),
        };

        let mut args = Vec::new();
        let mut redirections = Vec::new();

        loop {
            match self.tokens.peek() {
                Some(Token::Word(_)) => {
                    if let Some(Token::Word(word)) = self.tokens.next() {
                        args.push(word);
                    }
                }

                Some(Token::Redirect)
                | Some(Token::Append)
                | Some(Token::Input) => {
                    let kind = match self.tokens.next() {
                        Some(Token::Redirect) => RedirectKind::Stdout,
                        Some(Token::Append) => RedirectKind::Append,
                        Some(Token::Input) => RedirectKind::Stdin,
                        _ => unreachable!(),
                    };

                    let target = match self.tokens.next() {
                        Some(Token::Word(word)) => word,
                        _ => {
                            return Err(
                                ParseError::ExpectedRedirectionTarget
                            );
                        }
                    };

                    redirections.push(Redirection {
                        kind,
                        target,
                    });
                }

                Some(Token::Pipe) | None => break,
            }
        }

        Ok(Command {
            program,
            args,
            redirections,
        })
    }
}