pub(crate) mod error;
pub(crate) mod lexer;

use lexer::token::TokenKind;
use lexer::Lexer;

pub fn compile(code: &str) {
    let mut lexer = Lexer::new(code);
    loop {
        let tok = lexer.next_token();
        if tok.kind == TokenKind::Eof {
            break;
        }
        println!("{:?}", tok);
    }
}
