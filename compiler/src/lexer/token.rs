#[derive(Debug)]
pub struct Token {
    // literal: &'a str,
    pub kind: TokenKind, // temporary pub
    line: usize,
    // might have to add file path/name later on
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Token {
        Token {
            // literal,
            kind,
            line,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    IllegalCharacter,
    InvalidString,
    // Blank,
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(isize),
    String(String),
    Bool(bool),

    // Ident,  //Ident(String),
    // Int,    //Int(i64),
    // String, //String(String),
    // Bool,   //Bool(bool),
    // Might have to add Floating point later on not sure

    // Statements
    Assign,
    If,
    Else,

    // Operators
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    // Delimiters
    Comma,
    Colon,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    // Reseved keywords
    Func,
    Let,
    Return,
}
