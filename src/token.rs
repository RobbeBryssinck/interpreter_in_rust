// TODO: string for token type might not be performant
struct Token {
    token_type: String,
    literal: String,
}

const ILLEGAL: &str = "ILLEGAL";
const EOF: &str = "EOF";

// Identifiers + literals
const IDENT: &str = "IDENT"; // add, foobar, x, y, ...
const INT: &str = "INT"; // 12341

// Operators
const ASSIGN: &str = "=";
const PLUS: &str = "+";

// Delimiters
const COMMA: &str = ",";
const SEMICOLON: &str = ";";

const LPAREN: &str = "(";
const RPAREN: &str = ")";
const LBRACE: &str = "{";
const RBRACE: &str = "}";

// Keywords
const FUNCTION: &str = "FUNCTION";
const LET: &str = "LET";
