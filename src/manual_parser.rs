use crate::lexer::{Lexer, Tokenizer as TokenizerTrait};
use crate::token::Token as TokenTrait;
use std::fmt::Debug;
use std::vec::IntoIter;

pub struct Parser<Tokenzier, Token, TokenType>
where
    Tokenzier: TokenizerTrait<Token = Token, TokenType = TokenType>,
    Token: TokenTrait<Type = TokenType>,
    TokenType: Debug + Clone + Eq,
{
    lexer: Lexer<Tokenzier, Token, TokenType>,
}

impl<Tokenzier, Token, TokenType> Parser<Tokenzier, Token, TokenType>
where
    Tokenzier: TokenizerTrait<Token = Token, TokenType = TokenType>,
    Token: TokenTrait<Type = TokenType>,
    TokenType: Debug + Clone + Eq,
{
    pub fn new(source: IntoIter<char>) -> Self {
        Parser {
            lexer: Lexer::new(source),
        }
    }

    pub fn skip_all(&mut self, token_types: Vec<TokenType>) {
        self.parse_all(token_types, |_| {});
    }

    pub fn parse_all(&mut self, token_types: Vec<TokenType>, mut parse: impl FnMut(Vec<Token>)) {
        let parsed_tokens = token_types
            .into_iter()
            .map(|token_type| self.lexer.lex(vec![token_type]).expect("Parse Error"))
            .collect();

        parse(parsed_tokens);
    }

    pub fn parse_either(&mut self, choices: Vec<TokenType>, mut do_with_parsed: impl FnMut(Token)) {
        let parsed_token = self.lexer.lex(choices).expect("Parse Error");

        do_with_parsed(parsed_token);
    }

    pub fn check_for(&mut self, token_type: TokenType) -> bool {
        self.lexer.lex(vec![token_type]).is_some()
    }
}
