pub mod lex_manager;
pub mod recorder;


pub trait Lexer {
    type Token;
    type TokenType;

    fn lex(&mut self, valid_tokens: Vec<Self::TokenType>)
        -> Option<Self::Token>;
}
