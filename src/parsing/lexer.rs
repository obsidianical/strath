use super::preprocessor::preprocess;
use super::token::Token;
use crate::error::{IllegalCharacterError, Position};

const DIGITS: &str = "0123456789";
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET_C: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Lexer {
    code: String,
    pos: Position,
    current: char,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            code: code.clone(),
            pos: Position { line: 0, character: 0 },
            current: code.chars().nth(0),
        }
    }

    pub fn next(&mut self) {
        self.pos.character += 1;
        self.current = switch self.code.chars().nth(self.pos.character) {
            Some(c) => {
                let chara = ;
                if chara == '\n' { self.pos.line += 1; };
                chara
            },
            None => {}
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        self.code = preprocess(self.code.clone());
        
        loop {
            let mut jump_next: bool = false;
            let token: Option<Token> = match self.current {
                // SKIP OR BREAK
                Some(' ') => None,
                Some('\n') => None,
                None => break,

                // OPERATORS
                Some('+') => Some(Token::ADD),
                Some('-') => Some(Token::SUBTRACT),
                Some('*') => Some(Token::MULTIPLY),
                Some('/') => Some(Token::DIVIDE),
                Some('(') => Some(Token::LBRACK),
                Some(')') => Some(Token::RBRACK),
                
                // REST
                Some(c) => {
                    if DIGITS.contains(c) {
                        jump_next = true;
                        Some(self.make_nr_token())
                    } else {
                        IllegalCharacterError::new()
                    }
                },
            };

            match token {
                Some(token) => {
                    tokens.push(token);
                    if !jump_next {
                        self.next();
                    }
                },
                None => self.next(),
            }
        }

        println!("{:?}", tokens);
        tokens
    }

    fn make_nr_token(&mut self) -> Token {
        let mut nr: String = String::new();
        let mut dot_amount: u8 = 0;

        while (self.current != None) && (DIGITS.contains(self.current.unwrap()) || self.current.unwrap() == '.') {
            if self.current.unwrap() == '.' {
                if dot_amount == 1 {
                    panic!("Unexpected additional '.' in Number.");
                }
                dot_amount += 1;
            }
            nr.push(self.current.unwrap());
            self.next();
        }

        if dot_amount == 1 {
            return Token::FLOAT(nr.parse::<f32>().unwrap());
        } else {
            return Token::INT(nr.parse::<i32>().unwrap());
        }
    }
}

#[test]
fn test_operators() {
    let mut lexer = Lexer::new("+-*/".to_string());
    assert_eq!(lexer.tokenize(), vec![Token::ADD, Token::SUBTRACT, Token::MULTIPLY, Token::DIVIDE]);
}
