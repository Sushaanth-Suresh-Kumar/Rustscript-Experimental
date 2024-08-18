use crate::error;
use crate::token;

use error::LErrorHandler;
use std::str::Chars;
use token::{LiteralKind, Span, Token, TokenKind};
use unicode_properties::UnicodeEmoji;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Chars<'a>,
    // length_remaining: usize,
    begin: usize,
    end: usize,
    error_handler: &'a LErrorHandler<'a>,
}
// for the struct can have the iter().peekable()
// later check all the pub type access specifier and change accordingly
// --> checked now for now make it them public later change them to private and
// --> have a compiler.rs / lib.rs to handle all the Lexer->Parser->Execution pipeline and
// --> then give the compiler as the entry pub point

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str, error_handler: &'a LErrorHandler) -> Lexer<'a> {
        Lexer {
            input: code.chars(),
            // length_remaining: code.chars().count(), // code.len(),
            begin: 0,
            end: 0,
            error_handler,
        }
    }

    // TODO: maybe add this function in some util
    fn is_whitespace(ch: char) -> bool {
        matches!(
            ch,
            '\u{0009}'   // \t
            | '\u{000A}' // \n
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space

            // NEXT LINE from latin1
            | '\u{0085}'

            // Bidi markers
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK

            // Dedicated whitespace characters from Unicode
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}' // PARAGRAPH SEPARATOR
        )
    }

    fn is_eof(&self) -> bool {
        self.input.as_str().is_empty()
    }

    pub(crate) fn reset(&mut self) {
        self.begin = self.end;
    }

    pub(crate) fn create_span(&self) -> Span {
        Span::set(self.begin, self.end)
    }

    // pub(crate) fn unicode_position_span(&self) -> Span {
    //     let span = Span::set(self.input.clone().count(), self.length_remaining);
    //     // self.reset_unicode_position();
    //     span
    // }

    // pub(crate) fn reset_unicode_position(&mut self) {
    //     self.length_remaining = self.input.clone().count();
    // }

    // pub(crate) fn reset_position(&mut self) {
    //     self.length_remaining = self.input.as_str().len()
    // }

    // pub(crate) fn position(&self) -> usize {
    //     self.length_remaining - self.input.as_str().len()
    // }

    // Not sure if this is needed
    fn read_char(&mut self) {
        let _ = self.next();
    }

    fn next(&mut self) -> Option<char> {
        self.end += 1;
        self.input.next()
    }

    fn first(&self) -> char {
        self.input.clone().next().unwrap_or('\0')
    }

    fn handle_slash(&mut self) -> TokenKind {
        match self.first() {
            '/' => self.line_comment(),
            '*' => self.block_comment(),
            '=' => {
                self.read_char();
                TokenKind::SlashEq
            }
            _ => TokenKind::Slash,
        }
    }

    fn handle_colon(&mut self) -> TokenKind {
        match self.first() {
            ':' => {
                self.read_char();
                TokenKind::Scope
            }
            _ => TokenKind::Colon,
        }
    }

    fn handle_minus(&mut self) -> TokenKind {
        match self.first() {
            '>' => {
                self.read_char();
                TokenKind::Arrow
            }
            '=' => {
                self.read_char();
                TokenKind::MinusEq
            }
            _ => TokenKind::Minus,
        }
    }

    fn handle_equal(&mut self) -> TokenKind {
        match self.first() {
            '=' => {
                self.read_char();
                TokenKind::EqEq
            }
            _ => TokenKind::Eq,
        }
    }
    fn handle_bang(&mut self) -> TokenKind {
        match self.first() {
            '=' => {
                self.read_char();
                TokenKind::NotEq
            }
            _ => TokenKind::Bang,
        }
    }

    fn handle_lt(&mut self) -> TokenKind {
        match self.first() {
            '=' => {
                self.read_char();
                TokenKind::LtEq
            }
            _ => TokenKind::Lt,
        }
    }

    fn handle_gt(&mut self) -> TokenKind {
        match self.first() {
            '=' => {
                self.read_char();
                TokenKind::GtEq
            }
            _ => TokenKind::Gt,
        }
    }

    fn handle_ampersand(&mut self) -> TokenKind {
        match self.first() {
            '&' => {
                self.read_char();
                TokenKind::And
            }
            _ => TokenKind::Ampersand,
        }
    }

    fn handle_pipe(&mut self) -> TokenKind {
        match self.first() {
            '|' => {
                self.read_char();
                TokenKind::Or
            }
            _ => TokenKind::Pipe,
        }
    }

    fn handle_plus(&mut self) -> TokenKind {
        match self.first() {
            '=' => {
                self.read_char();
                TokenKind::PlusEq
            }
            _ => TokenKind::Plus,
        }
    }

    fn handle_asterisk(&mut self) -> TokenKind {
        match self.first() {
            '=' => {
                self.read_char();
                TokenKind::AsteriskEq
            }
            _ => TokenKind::Asterisk,
        }
    }

    fn handle_caret(&mut self) -> TokenKind {
        match self.first() {
            '=' => {
                self.read_char();
                TokenKind::CaretEq
            }
            _ => TokenKind::Caret,
        }
    }

    fn handle_percent(&mut self) -> TokenKind {
        match self.first() {
            '=' => {
                self.read_char();
                TokenKind::PercentEq
            }
            _ => TokenKind::Percent,
        }
    }

    fn line_comment(&mut self) -> TokenKind {
        self.eat_while(|ch| ch != '\n');
        TokenKind::LineComment
    }

    fn block_comment(&mut self) -> TokenKind {
        self.next();
        let mut depth = 1usize;
        while let Some(c) = self.next() {
            match c {
                '/' if self.first() == '*' => {
                    self.next();
                    depth += 1;
                }
                '*' if self.first() == '/' => {
                    self.next();
                    depth -= 1;
                    if depth == 0 {
                        // This block comment is closed, so for a construction like "/* */ */"
                        // there will be a successfully parsed block comment "/* */"
                        // and " */" will be processed separately.
                        break;
                    }
                }
                _ => (),
            }
        }
        TokenKind::BlockComment {
            terminated: depth == 0,
        }
    }

    fn is_id_start(c: char) -> bool {
        // This is XID_Start OR '_' (which formally is not a XID_Start).
        c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
    }

    fn is_id_continue(c: char) -> bool {
        unicode_xid::UnicodeXID::is_xid_continue(c)
    }

    // fn get_str_slice_from_chars_direct(
    //     &self,
    //     start_char_idx: usize,
    //     end_char_idx: usize,
    // ) -> String {
    //     self.input
    //         .clone()
    //         .by_ref()
    //         .enumerate()
    //         .filter_map(|(i, c)| {
    //             if i >= start_char_idx && i < end_char_idx {
    //                 Some(c)
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect::<String>()
    // }

    fn valid_or_invalid_identifier(&mut self, first_char: char) -> TokenKind {
        let literal = self.eat_while_get_literal(Self::is_id_continue, Some(first_char));
        // Known prefixes must have been handled earlier. So if
        // we see a prefix here, it is definitely an unknown prefix.
        match self.first() {
            c if !c.is_ascii() && c.is_emoji_char() => self.invalid_ident(),
            _ => {
                match Token::literal_to_keyword(&literal) {
                    Some(keyword) => TokenKind::Keyword { kind: keyword },
                    None => TokenKind::Ident {
                        name: literal, // self.get_str_slice_from_chars_direct(self.begin, self.end),
                    },
                }
            }
        }
    }

    fn invalid_ident(&mut self) -> TokenKind {
        // Start is already eaten, eat the rest of identifier.
        self.eat_while(|c| {
            unicode_xid::UnicodeXID::is_xid_continue(c)
                || (!c.is_ascii() && c.is_emoji_char())
                || c == '\u{200d}'
        });
        TokenKind::InvalidIdent
    }

    fn integer_or_float(&mut self, ch: char) -> TokenKind {
        let mut literal =
            self.eat_while_get_literal(|character| matches!(character, '0'..='9'), Some(ch));
        // handle floats here

        match self.first() {
            '.' => {
                literal.push(self.next().unwrap());
                self.handle_float(literal)
            }
            'e' | 'E' => {
                literal.push(self.next().unwrap());
                self.handle_exponent(literal)
            }
            _ => TokenKind::Literal {
                kind: LiteralKind::Int {
                    value: literal.parse::<isize>().unwrap(),
                },
            },
        }
    }

    fn handle_float(&mut self, mut number_prefix: String) -> TokenKind {
        match self.first() {
            '0'..='9' => {
                // number_prefix.push(self.next().unwrap());
                let decimals = self.eat_while_get_literal(|ch| matches!(ch, '0'..='9'), None);
                number_prefix.push_str(&decimals);
                self.handle_float_inner(number_prefix)
            }
            _ => {
                // Some Lexical Errors
                // atleast one digit must be present after decimal point
                TokenKind::InvalidDecimal
            }
        }
    }

    fn handle_float_inner(&mut self, mut number_prefix: String) -> TokenKind {
        match self.first() {
            'e' | 'E' => {
                number_prefix.push(self.next().unwrap());
                self.handle_exponent(number_prefix)
            }
            _ => TokenKind::Literal {
                kind: LiteralKind::Float {
                    value: number_prefix.parse::<f64>().unwrap(),
                },
            },
        }
    }

    fn handle_exponent(&mut self, mut number_prefix: String) -> TokenKind {
        match self.first() {
            '+' | '-' => {
                number_prefix.push(self.next().unwrap());
                self.handle_exponent_value(number_prefix)
            }
            '0'..='9' => self.handle_exponent_value(number_prefix),
            _ => {
                // should return Lexical Error
                // exponent number must be present after 'e' or 'E'
                TokenKind::InvalidExponent
            }
        }
    }

    fn handle_exponent_value(&mut self, mut number_exponent_prefix: String) -> TokenKind {
        match self.first() {
            '0'..='9' => {
                let exponent = self.eat_while_get_literal(|ch| matches!(ch, '0'..='9'), None);
                number_exponent_prefix.push_str(&exponent);
                TokenKind::Literal {
                    kind: LiteralKind::Float {
                        value: number_exponent_prefix.parse::<f64>().unwrap(),
                    },
                }
            }
            _ => {
                // should return Lexical Error
                // exponent number must be present after 'e' or 'E'
                TokenKind::InvalidExponent
            }
        }
    }

    fn quoted_string(&mut self, ch: char) -> TokenKind {
        let mut str_literal = String::new();
        while let Some(c) = self.next() {
            match c {
                character if character == ch => {
                    return TokenKind::Literal {
                        kind: LiteralKind::Str {
                            terminated: true,
                            value: str_literal,
                        },
                    };
                }
                '\\' if self.first() == '\\' || self.first() == '"' || self.first() == '\'' => {
                    // Bump again to skip escaped character.
                    str_literal.push(c);
                    str_literal.push(self.first());
                    self.read_char();
                }
                _ => str_literal.push(c),
            }
        }
        // End of file reached.
        TokenKind::Literal {
            kind: LiteralKind::Str {
                terminated: false,
                value: str_literal,
            },
        }
    }

    pub fn whitespace(&mut self) -> TokenKind {
        self.eat_while(Self::is_whitespace);
        TokenKind::Whitespace
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.read_char();
        }
    }

    fn eat_while_get_literal(
        &mut self,
        mut predicate: impl FnMut(char) -> bool,
        first_char: Option<char>,
    ) -> String {
        let mut literal = String::new();
        match first_char {
            Some(value) => literal.push(value),
            None => {}
        }
        while predicate(self.first()) && !self.is_eof() {
            literal.push(self.next().unwrap());
            // self.read_char();
        }
        literal
    }

    fn error_report(&self, err_token: &Token) {
        self.error_handler.error_report(err_token);
    }

    pub fn next_token(&mut self) -> Token {
        // first character of the token
        let first_char = match self.next() {
            Some(character) => character,
            None => return Token::new(TokenKind::Eof, Span::set(0, 0)), //self.unicode_position_span()),
        };
        let token_kind = match first_char {
            // slash or comment or block comment
            '/' => self.handle_slash(),

            // whitespace sequence
            ch if Self::is_whitespace(ch) => self.whitespace(),

            // identifier and keyword
            ch if Self::is_id_start(ch) => self.valid_or_invalid_identifier(first_char),

            // emoji and non-ascii starting characters
            ch if !ch.is_ascii() && ch.is_emoji_char() => self.invalid_ident(),

            ch @ '0'..='9' => self.integer_or_float(ch), // [1-9](\d+) | \d unsigned_integer

            ch if matches!(ch, '\'' | '\"') => self.quoted_string(ch),

            // One-symbol tokens.
            ';' => TokenKind::Semi,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '(' => TokenKind::OpenPara,
            ')' => TokenKind::ClosePara,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            '@' => TokenKind::At,
            '#' => TokenKind::Sharp,
            '~' => TokenKind::Tilde,
            '?' => TokenKind::Question,
            ':' => self.handle_colon(),
            '$' => TokenKind::Dollar,
            '=' => self.handle_equal(),
            '!' => self.handle_bang(),
            '<' => self.handle_lt(),
            '>' => self.handle_gt(),
            '-' => self.handle_minus(),
            '&' => self.handle_ampersand(),
            '|' => self.handle_pipe(),
            '+' => self.handle_plus(),
            '*' => self.handle_asterisk(),
            '^' => self.handle_caret(),
            '%' => self.handle_percent(),

            _ => TokenKind::Unknown,
        };

        //let curr_length = self.input.clone().count();
        // let res = Token::new(token_kind, self.unicode_position_span()); //self.position(),
        // self.reset_position();
        // self.reset_unicode_position();
        let res = Token::new(token_kind, self.create_span());
        //self.reset_unicode_position();
        self.reset();
        self.error_report(&res);
        res
    }
}
