use crate::lexer::Lexer as LexerTrait;
use std::str::Chars;

pub struct Parser<Lexer, Token, TokenType>
where
    for<'a> Lexer: LexerTrait<'a, Token = Token, TokenType = TokenType>,
{
    lexer: Lexer,
}

impl<Lexer, Token, TokenType> Parser<Lexer, Token, TokenType>
where
    for<'a> Lexer: LexerTrait<'a, Token = Token, TokenType = TokenType>,
{
    pub fn new(source: Chars) -> Self {
        Parser {
            lexer: Lexer::new(source),
        }
    }

    pub fn sequence_skip(&mut self, token_types: Vec<TokenType>) {
        self.sequence(token_types, |_| {});
    }

    pub fn sequence(&mut self, token_types: Vec<TokenType>, mut parse: impl FnMut(Vec<Token>)) {
        let parsed_tokens = token_types
            .into_iter()
            .map(|token_type| {
                self.lexer.lex(vec![token_type]).expect("Parse Error");

                self.lexer.pop_parsed_token().expect("Parse Error")
            })
            .collect();

        parse(parsed_tokens);
    }

    pub fn choice(&mut self, token_types: Vec<TokenType>, mut parse: impl FnMut(Token)) {
        self.lexer.lex(token_types).expect("Parse Error");

        let parsed_token = self.lexer.pop_parsed_token().expect("Parse Error");

        parse(parsed_token);
    }
}
