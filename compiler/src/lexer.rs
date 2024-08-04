pub mod token;

use crate::error::Error;
use std::{iter::Peekable, str::Chars};
use token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line_number: usize,
}
// for the struct can have the iter().peekable()
// later check all the pub type access specifier and change accordingly
// --> checked now for now make it them public later change them to private and
// --> have a compiler.rs / lib.rs to handle all the Lexer->Parser->Execution pipeline and
// --> then give the compiler as the entry pub point

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Lexer {
        Lexer {
            input: code.chars().peekable(),
            line_number: 1,
        }
    }

    fn get_line_number(&self) -> usize {
        self.line_number
    }

    // Not sure if this is needed
    fn read_char(&mut self) {
        let _ = self.input.next();
    }

    fn next(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn skip_whitespace(&mut self) {
        // if peek character is space | \t
        // try handling new line here or in separate function
        while matches!(self.peek(), Some(&' ') | Some(&'\t')) {
            self.read_char();
        }
    }

    fn next_line(&mut self) {
        while self.peek() == Some(&'\n') {
            self.line_number += 1;
            self.read_char();
        }
    }

    fn consume_identifier(&mut self) -> Token {
        let mut literal = String::new();
        loop {
            literal.push(self.next().unwrap());
            if !matches!(self.peek(), Some('a'..='z') | Some('A'..='Z') | Some('_')) {
                break;
            }
        }
        literal.shrink_to_fit();

        // Keyword table for now
        let kind = match literal.as_ref() {
            "fn" => TokenKind::Func,
            "let" => TokenKind::Let,
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            _ => TokenKind::Ident(literal),
        };
        Token::new(kind, self.get_line_number())
    }

    fn consume_string(&mut self) -> Token {
        let mut literal = String::new();
        let start = *self.peek().unwrap();
        self.read_char();
        while let Some(&ch) = self.peek() {
            if ch == start {
                self.read_char();
                literal.shrink_to_fit();
                return Token::new(TokenKind::String(literal), self.get_line_number());
            }
            literal.push(*self.peek().unwrap());
            self.read_char();
        }
        literal.shrink_to_fit();
        let err = Error::new(self.get_line_number(), "", "unterminated string literal");
        err.report();
        Token::new(TokenKind::InvalidString, self.get_line_number())
    }

    fn consume_number(&mut self) -> Token {
        let mut literal = String::new();
        while matches!(self.peek(), Some('0'..='9')) {
            literal.push(*self.peek().unwrap());
            self.read_char();
        }
        literal.shrink_to_fit();
        return Token::new(
            TokenKind::Int(
                literal
                    .parse::<isize>()
                    .expect("failed to parse into Integer"),
            ),
            self.get_line_number(),
        );
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.next_line();

        let token = match self.peek() {
            Some('=') => {
                self.read_char();
                if self.peek() == Some(&'=') {
                    self.read_char();
                    Token::new(TokenKind::Equal, self.get_line_number())
                } else {
                    Token::new(TokenKind::Assign, self.get_line_number())
                }
            }
            Some('+') => {
                self.read_char();
                Token::new(TokenKind::Plus, self.get_line_number())
            }
            Some('-') => {
                self.read_char();
                Token::new(TokenKind::Minus, self.get_line_number())
            }
            Some('!') => {
                self.read_char();
                if self.peek() == Some(&'=') {
                    self.read_char();
                    Token::new(TokenKind::NotEqual, self.get_line_number())
                } else {
                    Token::new(TokenKind::Bang, self.get_line_number())
                }
            }
            Some('/') => {
                self.read_char();
                Token::new(TokenKind::Slash, self.get_line_number())
            }
            Some('*') => {
                self.read_char();
                Token::new(TokenKind::Asterisk, self.get_line_number())
            }
            Some('<') => {
                self.read_char();
                if self.peek() == Some(&'=') {
                    self.read_char();
                    Token::new(TokenKind::LessThanEqual, self.get_line_number())
                } else {
                    Token::new(TokenKind::LessThan, self.get_line_number())
                }
            }
            Some('>') => {
                self.read_char();
                if self.peek() == Some(&'=') {
                    self.read_char();
                    Token::new(TokenKind::GreaterThanEqual, self.get_line_number())
                } else {
                    Token::new(TokenKind::GreaterThan, self.get_line_number())
                }
            }
            Some('(') => {
                self.read_char();
                Token::new(TokenKind::Lparen, self.get_line_number())
            }
            Some(')') => {
                self.read_char();
                Token::new(TokenKind::Rparen, self.get_line_number())
            }
            Some('{') => {
                self.read_char();
                Token::new(TokenKind::Lbrace, self.get_line_number())
            }
            Some('}') => {
                self.read_char();
                Token::new(TokenKind::Rbrace, self.get_line_number())
            }
            Some('[') => {
                self.read_char();
                Token::new(TokenKind::Lbracket, self.get_line_number())
            }
            Some(']') => {
                self.read_char();
                Token::new(TokenKind::Rbracket, self.get_line_number())
            }
            Some(',') => {
                self.read_char();
                Token::new(TokenKind::Comma, self.get_line_number())
            }
            Some(';') => {
                self.read_char();
                Token::new(TokenKind::Semicolon, self.get_line_number())
            }
            Some(':') => {
                self.read_char();
                Token::new(TokenKind::Colon, self.get_line_number())
            }
            // Identifier
            Some('a'..='z') | Some('A'..='Z') | Some('_') => self.consume_identifier(),
            // Integer
            Some('0'..='9') => self.consume_number(),
            // Try adding integer, floating and scientific notation

            // String
            Some('"') | Some('\'') => self.consume_string(),
            Some(_) => {
                let message: &str = &format!("unexpected character '{}'", self.peek().unwrap());
                let err = Error::new(self.get_line_number(), "", message);
                err.report();
                self.read_char();
                Token::new(TokenKind::IllegalCharacter, self.get_line_number())
            }
            None => {
                Token::new(TokenKind::Eof, self.get_line_number()) // return EOF token
            }
        };
        token
    }
}
// MesloLGM NF
