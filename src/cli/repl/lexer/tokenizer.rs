

enum StateToken {
    Normal,
    InQuote,
    InEscape,
}

enum Token {
    Word(String),
    Pipe,
}

pub struct TokenCollector {
    state   : StateToken,
    tokens  : Vec<Token>,
    t       : String, //current token
}

impl TokenCollector {

    pub fn new() -> Self {

        TokenCollector {
            state   : StateToken::Normal,
            tokens  : Vec::new(),
            t       : String::new(),
        }

    }

    pub fn start_line(&mut self, line: String) {

        self._reset();

        for c in line.chars() {

            self.process_char(c);
        }
    }


    pub fn process_char(&mut self, c: char) {
        
    }

    fn _reset(&mut self) {

        self.state = StateToken::Normal;
        self.tokens.clear();
        self.t.clear();
    }
}