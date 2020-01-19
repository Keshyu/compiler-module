use crate::lexer::Lexer as LexerTrait;
use std::str::Chars;
use std::marker::PhantomData;

pub struct Parser<'a, Lexer, Token, TokenType>
where
    Lexer: LexerTrait<'a, Token = Token, TokenType = TokenType>,
{
    lexer: Lexer,
    _m: PhantomData<&'a ()>
}

impl<'a, Lexer, Token, TokenType> Parser<'a, Lexer, Token, TokenType>
where
    Lexer: LexerTrait<'a, Token = Token, TokenType = TokenType>,
{
    pub fn new(source: Chars<'a>) -> Self {
        Parser {
            lexer: Lexer::new(source),
            _m: PhantomData,
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
