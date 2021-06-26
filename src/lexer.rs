use std::{usize};

use crate::token::Token;
use super::token;

pub enum TokenParsingError {
    UnknownToken,
}

struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    character: char
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            character: '\x00'
        };

        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Result<Token, TokenParsingError> {
        let result = match self.character {
            '=' => Ok(Token::new(String::from(token::ASSIGN), String::from(self.character))),
            ';' => Ok(Token::new(String::from(token::SEMICOLON), String::from(self.character))),
            '(' => Ok(Token::new(String::from(token::LPAREN), String::from(self.character))),
            ')' => Ok(Token::new(String::from(token::RPAREN), String::from(self.character))),
            ',' => Ok(Token::new(String::from(token::COMMA), String::from(self.character))),
            '+' => Ok(Token::new(String::from(token::PLUS), String::from(self.character))),
            '{' => Ok(Token::new(String::from(token::LBRACE), String::from(self.character))),
            '}' => Ok(Token::new(String::from(token::RBRACE), String::from(self.character))),
            '\x00' => Ok(Token::new(String::from(token::EOF), String::from(""))),
            _ => Err(TokenParsingError::UnknownToken)
        };

        self.read_char();

        result
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.chars().count() as i32 {
            self.character = '\x00';
        } else {
            self.character = self.input.chars().nth(self.read_position as usize).expect("Cannot read character.");
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::token::{Token, ASSIGN, PLUS, LPAREN, RPAREN, LBRACE, RBRACE, COMMA, SEMICOLON, EOF};
    use super::*;
    
	#[test]
	fn next_token_test() {
        let input = String::from("=+(){},;");

        let test_tokens: [Token; 9] = [
            Token {
                token_type: String::from(ASSIGN),
                literal: String::from("="),
            },
            Token {
                token_type: String::from(PLUS),
                literal: String::from("+"),
            },
            Token {
                token_type: String::from(LPAREN),
                literal: String::from("("),
            },
            Token {
                token_type: String::from(RPAREN),
                literal: String::from(")"),
            },
            Token {
                token_type: String::from(LBRACE),
                literal: String::from("{"),
            },
            Token {
                token_type: String::from(RBRACE),
                literal: String::from("}"),
            },
            Token {
                token_type: String::from(COMMA),
                literal: String::from(","),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(EOF),
                literal: String::from(""),
            },
        ];

        let mut lexer = Lexer::new(input);

        for test_token in test_tokens {
            let fetched_token = match lexer.next_token() {
                Ok(fetched_token) => fetched_token,
                Err(e) => {
                    match e {
                        TokenParsingError::UnknownToken => panic!("Error parsing token: unknown token.")
                    }
                }
            };
            
            assert_eq!(fetched_token.token_type, test_token.token_type);
            assert_eq!(fetched_token.literal, test_token.literal);
        }
	}
}