use crate::parser::ast::{Policy, Rule, PolicyAction, Condition};
use crate::parser::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let first = lexer.next_token();

        Self {
            lexer,
            current: first,
        }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse_policy(&mut self) -> Policy {
        let mut policy = Policy {
            name: "default".to_string(),
            rules: vec![],
        };

        while let Token::End = self.current {
            break;
        }

        policy
    }

    fn parse_rule(&mut self) -> Option<Rule> {
        None // placeholder MVP
    }
}
