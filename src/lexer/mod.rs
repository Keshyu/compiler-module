use std::vec::IntoIter;
use crate::token::Token as TokenTrait;

mod lex_manager;
mod recorder;

pub use lex_manager::LexManager;
pub use recorder::Recorder;

pub trait Lexer {
    type Token: TokenTrait<Type = Self::TokenType>;
    type TokenType;

    fn new(source: IntoIter<char>) -> Self;

    fn lex(&mut self, valid_tokens: Vec<Self::TokenType>) -> Option<Self::TokenType> {
        let parsed_token = self.tokenize(valid_tokens);

        match parsed_token {
            Some(token) => {
                let r#type = token.r#type();

                self.push_parsed_token(token);

                Some(r#type)
            }
            None => None,
        }
    }

    fn tokenize(&mut self, valid_tokens: Vec<Self::TokenType>) -> Option<Self::Token>;

    fn push_parsed_token(&mut self, token: Self::Token);

    fn pop_parsed_token(&mut self) -> Option<Self::Token>;

    fn are_parsed_tokens(&self) -> bool;
}
