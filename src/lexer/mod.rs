mod lex_manager;
mod recorder;

pub use lex_manager::LexManager;
pub use recorder::Recorder;


pub trait Lexer {
    type Token;
    type TokenType;

    fn new(source: &str) -> Self;

    fn lex(
        &mut self,
        valid_tokens: Vec<Self::TokenType>,
        skipable_tokens: Vec<Self::TokenType>,
    )
        -> Option<Self::Token>;
}
