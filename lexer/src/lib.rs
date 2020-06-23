use token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            ch: input.chars().next().unwrap(),
            input,
            position: 0,
            read_position: 0,
        };

        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            // @TODO: use alternative rescueing 
            self.ch = self.input.chars().nth(self.position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // @TODO: is it possible for this function to not be mutable?
    pub fn next_token(&mut self) -> Token {
        self.read_char();
        Token::next_token(&self.ch)
    } 
}
