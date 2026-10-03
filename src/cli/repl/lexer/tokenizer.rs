

#[derive(PartialEq)]
enum StateToken {
    Normal,
    InQuote,
    InEscape,
}


#[derive(Debug)]
pub enum Token {
    Word(String),
    Pipe,
    Redirect,      // >
    Append,        // >>
    Input,         // <
}

pub struct TokenCollector {
    state           : StateToken,
    buffer_tokens   : Vec<Token>,
    ct              : String, //current token
    last_char       : Option<char>,
}

impl TokenCollector {

    pub fn new() -> Self {

        TokenCollector {
            state           : StateToken::Normal,
            buffer_tokens   : Vec::new(),
            ct              : String::new(),
            last_char       : None,
        }

    }

    pub fn iter_line(&mut self, line: String) -> std::io::Result<Vec<Token>>{

        self._reset();

        for c in line.chars() {

            self._process_char(c);
        }

        let result = self._finish();
        
        return result;
    }


    fn _process_char(&mut self, c: char) {

        match c {
            ' ' => {
                match self.state {
                    StateToken::Normal => {
                        if !self.ct.is_empty() {
                            self.buffer_tokens.push(Token::Word(std::mem::take(&mut self.ct)));
                        }
                        self.last_char = None;
                    },
                    StateToken::InQuote => {
                        self.ct.push(c);
                    },
                    StateToken::InEscape => {
                        self.ct.push(c);
                        self.state = StateToken::Normal;
                    }
                }

            },
            
            '|' => {
                match self.state {
                    StateToken::Normal => {
                        if !self.ct.is_empty() {
                            self.buffer_tokens.push(Token::Word(std::mem::take(&mut self.ct)));
                        }
                        self.buffer_tokens.push(Token::Pipe);
                        self.last_char = None;
                    },
                    StateToken::InQuote => {
                        self.ct.push(c);
                    },
                    StateToken::InEscape => {
                        self.ct.push(c);
                        self.state = StateToken::Normal;
                    }
                }
            },

            '>' => {
                match self.state {
                    StateToken::Normal => {
                        if !self.ct.is_empty() {
                            self.buffer_tokens.push(Token::Word(std::mem::take(&mut self.ct)));
                        }
                        
                        if self.last_char == Some('>') {
                            if let Some(Token::Redirect) = self.buffer_tokens.last() {
                                self.buffer_tokens.pop();
                            }
                            self.buffer_tokens.push(Token::Append);
                            self.last_char = None;
                        } else {
                            self.buffer_tokens.push(Token::Redirect);
                            self.last_char = Some('>');
                        }
                    },
                    StateToken::InQuote => {
                        self.ct.push(c);
                    },
                    StateToken::InEscape => {
                        self.ct.push(c);
                        self.state = StateToken::Normal;
                    }
                }
            },

            '<' => {
                match self.state {
                    StateToken::Normal => {
                        if !self.ct.is_empty() {
                            self.buffer_tokens.push(Token::Word(std::mem::take(&mut self.ct)));
                        }
                        self.buffer_tokens.push(Token::Input);
                        self.last_char = None;
                    },
                    StateToken::InQuote => {
                        self.ct.push(c);
                    },
                    StateToken::InEscape => {
                        self.ct.push(c);
                        self.state = StateToken::Normal;
                    }
                }
            },

            '"' => {
                match self.state {
                    StateToken::Normal => {
                        self.state = StateToken::InQuote;
                    },
                    StateToken::InQuote => {
                        self.state = StateToken::Normal;
                    },
                    StateToken::InEscape => {
                        self.ct.push(c);
                        self.state = StateToken::Normal;
                    }
                }
            },

            '\\' => {
                match self.state {
                    StateToken::Normal => {
                        self.state = StateToken::InEscape;
                    },
                    StateToken::InQuote => {
                        self.ct.push(c);
                    },
                    StateToken::InEscape => {
                        self.ct.push(c);
                        self.state = StateToken::Normal;
                    }
                }
            },

            _ => {
                self.ct.push(c)
            }
        }

    }

    fn _finish(&mut self) -> std::io::Result<Vec<Token>>  {
        
        if self.state == StateToken::InQuote {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unclosed quote sequence"
                    )
                )
        }  else if self.state == StateToken::InEscape {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unclosed escape sequence"
                    )
                )
        }

        if !self.ct.is_empty() {
            self.buffer_tokens.push(Token::Word(std::mem::take(&mut self.ct)));
        }

        Ok(std::mem::take(&mut self.buffer_tokens))

    }

    fn _reset(&mut self) {

        self.state = StateToken::Normal;
        self.buffer_tokens.clear();
        self.ct.clear();
    }
}