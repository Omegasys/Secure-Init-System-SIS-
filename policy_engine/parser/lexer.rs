#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Action(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Arrow,
    Eq,
    String(String),
    End,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\n' | '\t' => {
                    self.advance();
                }

                '{' => {
                    self.advance();
                    return Token::LBrace;
                }

                '}' => {
                    self.advance();
                    return Token::RBrace;
                }

                '(' => {
                    self.advance();
                    return Token::LParen;
                }

                ')' => {
                    self.advance();
                    return Token::RParen;
                }

                '=' => {
                    self.advance();
                    return Token::Eq;
                }

                '"' => {
                    return self.read_string();
                }

                _ => {
                    if c.is_alphabetic() {
                        return self.read_identifier();
                    }

                    self.advance();
                }
            }
        }

        Token::End
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text: String = self.input[start..self.pos].iter().collect();
        Token::Identifier(text)
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip "

        let start = self.pos;

        while let Some(c) = self.peek() {
            if c != '"' {
                self.advance();
            } else {
                break;
            }
        }

        let text: String = self.input[start..self.pos].iter().collect();
        self.advance(); // skip closing "

        Token::String(text)
    }
}
