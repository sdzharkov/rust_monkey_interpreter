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

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(&self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            // @TODO: use alternative rescueing
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;
        if is_letter(&self.ch) {
            let identifier = self.read_identifier();
            token = Token::next_identifier_token(identifier);
        } else {
            token = Token::next_token(&self.ch);
        }
        self.read_char();
        token
    }
}

fn is_letter(character: &char) -> bool {
    character.is_alphabetic() || *character == '_'
}
