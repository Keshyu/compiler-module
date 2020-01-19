use crate::lexer::Lexer as LexerTrait;
use crate::token::Token as TokenTrait;
use std::vec::IntoIter;

pub struct Parser<Lexer, Token, TokenType>
where
    Lexer: LexerTrait<Token = Token, TokenType = TokenType>,
    Token: TokenTrait<Type = TokenType>,
{
    lexer: Lexer,
}

impl<Lexer, Token, TokenType> Parser<Lexer, Token, TokenType>
where
    Lexer: LexerTrait<Token = Token, TokenType = TokenType>,
    Token: TokenTrait<Type = TokenType>,
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
            .map(|token_type| {
                if !self.lexer.are_parsed_tokens() {
                    self.lexer.lex(vec![token_type]).expect("Parse Error");
                }

                self.lexer.pop_parsed_token().expect("Parse Error")
            })
            .collect();

        parse(parsed_tokens);
    }

    pub fn parse_either(&mut self, choices: Vec<TokenType>, mut do_with_parsed: impl FnMut(Token)) {
        if !self.lexer.are_parsed_tokens() {
            self.lexer.lex(choices).expect("Parse Error");
        }

        let parsed_token = self.lexer.pop_parsed_token().expect("Parse Error");

        do_with_parsed(parsed_token);
    }

    pub fn check_for(&mut self, token_type: TokenType) -> bool {
        self.lexer.lex(vec![token_type]).is_some()
    }
}
