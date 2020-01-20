use super::lex_manager::LexManager;
use crate::token::Token as TokenTrait;

pub trait Tokenizer {
    type Token: TokenTrait<Type = Self::TokenType>;
    type TokenType;

    fn new() -> Self;

    fn tokenize(
        &mut self,
        valid_tokens: Vec<Self::TokenType>,
        manager: &mut LexManager,
    ) -> Option<Self::Token>;
}
