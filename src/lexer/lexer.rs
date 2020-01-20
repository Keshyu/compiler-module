use super::lex_manager::LexManager;
use super::tokenizer::Tokenizer as TokenizerTrait;
use crate::token::Token as TokenTrait;
use std::vec::IntoIter;

pub struct Lexer<Tokenizer, Token, TokenType>
where
    Tokenizer: TokenizerTrait<Token = Token, TokenType = TokenType>,
    Token: TokenTrait<Type = TokenType>,
{
    manager: LexManager,
    tokenizer: Tokenizer,
    parsed_token: Option<Token>,
}

impl<Tokenizer, Token, TokenType> Lexer<Tokenizer, Token, TokenType>
where
    Tokenizer: TokenizerTrait<Token = Token, TokenType = TokenType>,
    Token: TokenTrait<Type = TokenType>,
{
    pub fn new(mut source: IntoIter<char>) -> Self {
        Lexer {
            manager: LexManager {
                lookahead: source.next(),
                source: source,
            },
            tokenizer: Tokenizer::new(),
            parsed_token: None,
        }
    }

    pub fn lex_all(&mut self, token_types: Vec<TokenType>) -> Vec<Token> {
        token_types
            .into_iter()
            .map(|token_type| {
                self.lex_one(token_type)
            })
            .collect()
    }

    pub fn lex_one(&mut self, token_type: TokenType) -> Token {
        self.lex(vec![token_type]).expect("Parse Error");
        self.parsed_token().unwrap()
    }

    pub fn lex_either(&mut self, token_types: Vec<TokenType>) -> TokenType {
        self.lex(token_types).expect("Parse Error")
    }

    pub fn parsed_token(&mut self) -> Option<Token> {
        self.parsed_token.take()
    }

    fn lex(&mut self, valid_tokens: Vec<TokenType>) -> Option<TokenType> {
        let parsed_token = self.tokenizer.tokenize(valid_tokens, &mut self.manager);

        parsed_token.map(|token| {
            let r#type = token.r#type();

            self.save_parsed_token(token);

            r#type
        })
    }

    fn save_parsed_token(&mut self, token: Token) {
        self.parsed_token = Some(token);
    }
}
