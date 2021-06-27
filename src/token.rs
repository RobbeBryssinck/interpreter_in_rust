use phf::phf_map;

// TODO: string for token type might not be performant
pub struct Token {
    pub token_type: String,
    pub literal: String,
    pub is_identifier: bool,
}

impl Token {
    pub fn new(token_type: String, literal: String, is_identifier: bool) -> Self {
        Token { token_type, literal, is_identifier }
    }

    pub fn lookup_identifier(identifier: &str) -> &str {
        match KEYWORDS.get(identifier) {
            Some(keyword) => keyword,
            _ => IDENT
        }
    }
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &str = "INT"; // 12341

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub const LT: &str = "<";
pub const GT: &str = ">";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";

pub static KEYWORDS: phf::Map<&'static str, &str> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
};