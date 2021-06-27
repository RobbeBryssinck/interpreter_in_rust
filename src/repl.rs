use std::io::{self, Write};

use super::token;
use super::token::{Token};
use super::lexer::{Lexer, TokenParsingError};

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = std::io::stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            return;
        }

        let mut lexer = Lexer::new(input);

        let mut fetched_token = match lexer.next_token() {
            Ok(fetched_token) => fetched_token,
            Err(e) => {
                match e {
                    TokenParsingError::UnknownToken => {
                        println!("Error parsing token: unknown token.");
                        continue;
                    }
                }
            }
        };

        while fetched_token.token_type != token::EOF && fetched_token.token_type != token::ILLEGAL {
            println!("{:?}", fetched_token);

            fetched_token = match lexer.next_token() {
                Ok(fetched_token) => fetched_token,
                Err(e) => {
                    match e {
                        TokenParsingError::UnknownToken => {
                            println!("Error parsing token: unknown token.");
                            Token::new(String::from(token::ILLEGAL), String::from("\x00"))
                        }
                    }
                }
            };
        }
    }
}