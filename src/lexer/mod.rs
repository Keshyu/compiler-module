use std::str::Chars;

mod lex_manager;
mod recorder;

pub use lex_manager::LexManager;
pub use recorder::Recorder;

pub trait Lexer {
    type Token;
    type TokenType;

    fn new(source: Chars) -> Self;

    fn lex(&mut self, valid_tokens: Vec<Self::TokenType>) -> Option<Self::TokenType>;

    fn pop_parsed_token(&mut self) -> Option<Self::Token>;
}
