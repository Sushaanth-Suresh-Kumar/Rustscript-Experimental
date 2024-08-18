// pub(crate) mod error;
pub(crate) mod error;
pub(crate) mod lexer;
pub(crate) mod token;

use error::LErrorHandler;
use lexer::Lexer;
use token::TokenKind;

pub fn compile(code: &str, file_name: &str) {
    let err_handler = LErrorHandler::new(code, file_name);
    let mut lexer = Lexer::new(code, &err_handler);
    loop {
        let tok = lexer.next_token();
        if tok.kind == TokenKind::Eof {
            break;
        }
        if tok.kind != TokenKind::Whitespace {
            println!("{:?}", tok);
        }
    }
}
