use std::{usize};

use crate::token::Token;
use super::token;

pub enum TokenParsingError {
    UnknownToken,
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
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

    // TODO: Error or illegal token?
    pub fn next_token(&mut self) -> Result<Token, TokenParsingError> {
        self.skip_whitespace();

        let result = match self.character {
            '=' if self.peek_char() == '=' => {
                self.read_char();
                Ok(Token::new(String::from(token::EQ), String::from("==")))
            },
            '=' => Ok(Token::new(String::from(token::ASSIGN), String::from(self.character))),
            '+' => Ok(Token::new(String::from(token::PLUS), String::from(self.character))),
            '-' => Ok(Token::new(String::from(token::MINUS), String::from(self.character))),
            '!' if self.peek_char() == '=' => {
                self.read_char();
                Ok(Token::new(String::from(token::NOT_EQ), String::from("!=")))
            },
            '!' => Ok(Token::new(String::from(token::BANG), String::from(self.character))),
            '/' => Ok(Token::new(String::from(token::SLASH), String::from(self.character))),
            '*' => Ok(Token::new(String::from(token::ASTERISK), String::from(self.character))),
            '<' => Ok(Token::new(String::from(token::LT), String::from(self.character))),
            '>' => Ok(Token::new(String::from(token::GT), String::from(self.character))),
            ';' => Ok(Token::new(String::from(token::SEMICOLON), String::from(self.character))),
            '(' => Ok(Token::new(String::from(token::LPAREN), String::from(self.character))),
            ')' => Ok(Token::new(String::from(token::RPAREN), String::from(self.character))),
            ',' => Ok(Token::new(String::from(token::COMMA), String::from(self.character))),
            '{' => Ok(Token::new(String::from(token::LBRACE), String::from(self.character))),
            '}' => Ok(Token::new(String::from(token::RBRACE), String::from(self.character))),
            '\x00' => Ok(Token::new(String::from(token::EOF), String::from(""))),
            _ if self.is_letter() => {
                let identifier = self.read_identifier();
                Ok(Token::new(String::from(Token::lookup_identifier(identifier)), String::from(identifier)))
            },
            _ if self.character.is_digit(10) => {
                Ok(Token::new(String::from(token::INT), String::from(self.read_number())))
            },
            _ => Err(TokenParsingError::UnknownToken),
        };

        self.read_char();

        result
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.chars().count() {
            self.character = '\x00';
        } else {
            self.character = self.input.chars().nth(self.read_position).expect("Cannot read character.");
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn revert_position(&mut self) {
        self.position -= 1;
        self.read_position -= 1;
        self.character = self.input.chars().nth(self.read_position).expect("Cannot read character.");
    }

    fn read_identifier(&mut self) -> &str {
        let lower_position = self.position;

        while self.is_letter() {
            self.read_char();
        }

        let upper_position = self.position;
        self.revert_position();

        &self.input[lower_position..upper_position]
    }

    fn read_number(&mut self) -> &str {
        let lower_position = self.position;

        while self.character.is_digit(10) {
            self.read_char();
        }

        let upper_position = self.position;
        self.revert_position();

        &self.input[lower_position..upper_position]
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.chars().count() {
            return '\x00';
        } else {
            return self.input.chars().nth(self.read_position).expect("Cannot read character.");
        }
    }

    fn is_letter(&self) -> bool {
        self.character.is_alphabetic() || self.character == '_'
    }

    fn skip_whitespace(&mut self) {
        while self.character == ' ' || self.character == '\t' || self.character == '\n' || self.character == '\r' {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{ASTERISK, BANG, ELSE, EQ, FALSE, FUNCTION, GT, IDENT, IF, INT, LET, LT, MINUS, NOT_EQ, RETURN, SLASH, TRUE};

    use super::token::{Token, ASSIGN, PLUS, LPAREN, RPAREN, LBRACE, RBRACE, COMMA, SEMICOLON, EOF};
    use super::*;
    
	#[test]
	fn next_token_test() {
        let input = String::from("let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
");

        let test_tokens = [
            Token {
                token_type: String::from(LET),
                literal: String::from("let"),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("five"),
            },
            Token {
                token_type: String::from(ASSIGN),
                literal: String::from("="),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("5"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(LET),
                literal: String::from("let"),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("ten"),
            },
            Token {
                token_type: String::from(ASSIGN),
                literal: String::from("="),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("10"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(LET),
                literal: String::from("let"),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("add"),
            },
            Token {
                token_type: String::from(ASSIGN),
                literal: String::from("="),
            },
            Token {
                token_type: String::from(FUNCTION),
                literal: String::from("fn"),
            },
            Token {
                token_type: String::from(LPAREN),
                literal: String::from("("),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("x"),
            },
            Token {
                token_type: String::from(COMMA),
                literal: String::from(","),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("y"),
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
                token_type: String::from(IDENT),
                literal: String::from("x"),
            },
            Token {
                token_type: String::from(PLUS),
                literal: String::from("+"),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("y"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(RBRACE),
                literal: String::from("}"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(LET),
                literal: String::from("let"),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("result"),
            },
            Token {
                token_type: String::from(ASSIGN),
                literal: String::from("="),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("add"),
            },
            Token {
                token_type: String::from(LPAREN),
                literal: String::from("("),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("five"),
            },
            Token {
                token_type: String::from(COMMA),
                literal: String::from(","),
            },
            Token {
                token_type: String::from(IDENT),
                literal: String::from("ten"),
            },
            Token {
                token_type: String::from(RPAREN),
                literal: String::from(")"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(BANG),
                literal: String::from("!"),
            },
            Token {
                token_type: String::from(MINUS),
                literal: String::from("-"),
            },
            Token {
                token_type: String::from(SLASH),
                literal: String::from("/"),
            },
            Token {
                token_type: String::from(ASTERISK),
                literal: String::from("*"),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("5"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("5"),
            },
            Token {
                token_type: String::from(LT),
                literal: String::from("<"),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("10"),
            },
            Token {
                token_type: String::from(GT),
                literal: String::from(">"),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("5"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(IF),
                literal: String::from("if"),
            },
            Token {
                token_type: String::from(LPAREN),
                literal: String::from("("),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("5"),
            },
            Token {
                token_type: String::from(LT),
                literal: String::from("<"),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("10"),
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
                token_type: String::from(RETURN),
                literal: String::from("return"),
            },
            Token {
                token_type: String::from(TRUE),
                literal: String::from("true"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(RBRACE),
                literal: String::from("}"),
            },
            Token {
                token_type: String::from(ELSE),
                literal: String::from("else"),
            },
            Token {
                token_type: String::from(LBRACE),
                literal: String::from("{"),
            },
            Token {
                token_type: String::from(RETURN),
                literal: String::from("return"),
            },
            Token {
                token_type: String::from(FALSE),
                literal: String::from("false"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(RBRACE),
                literal: String::from("}"),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("10"),
            },
            Token {
                token_type: String::from(EQ),
                literal: String::from("=="),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("10"),
            },
            Token {
                token_type: String::from(SEMICOLON),
                literal: String::from(";"),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("10"),
            },
            Token {
                token_type: String::from(NOT_EQ),
                literal: String::from("!="),
            },
            Token {
                token_type: String::from(INT),
                literal: String::from("9"),
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

            println!("TYPE: {}, LITERAL: {}, EXPECTED TYPE: {}, EXPECTED LITERAL: {}", fetched_token.token_type, fetched_token.literal, test_token.token_type, test_token.literal);
            
            assert_eq!(fetched_token.token_type, test_token.token_type);
            assert_eq!(fetched_token.literal, test_token.literal);
        }
	}
}