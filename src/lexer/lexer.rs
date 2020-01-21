use super::lex_manager::LexManager;
use super::tokenizer::Tokenizer as TokenizerTrait;
use crate::token::Token as TokenTrait;
use std::fmt::Debug;
use std::vec::IntoIter;

pub struct Lexer<Tokenizer, Token, TokenType>
where
    Tokenizer: TokenizerTrait<Token = Token, TokenType = TokenType>,
    Token: TokenTrait<Type = TokenType>,
    TokenType: Debug + Clone + Eq,
{
    manager: LexManager,
    tokenizer: Tokenizer,
    parsed_token: Option<Token>,
}

impl<Tokenizer, Token, TokenType> Lexer<Tokenizer, Token, TokenType>
where
    Tokenizer: TokenizerTrait<Token = Token, TokenType = TokenType>,
    Token: TokenTrait<Type = TokenType>,
    TokenType: Debug + Clone + Eq,
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
            .map(|token_type| self.lex_one(token_type))
            .collect()
    }

    pub fn lex_one(&mut self, token_type: TokenType) -> Token {
        let error_message = format!("Parse Error: Expected: {:?}", token_type);

        self.lex(vec![token_type]).expect(error_message.as_str());

        self.parsed_token().unwrap()
    }

    pub fn lex_either(&mut self, token_types: Vec<TokenType>) -> TokenType {
        let error_message =
            format!("Parse Error: Expected one of these: {:?}", token_types);

        self.lex(token_types).expect(error_message.as_str())
    }

    pub fn parsed_token(&mut self) -> Option<Token> {
        self.parsed_token.take()
    }

    fn lex(&mut self, valid_tokens: Vec<TokenType>) -> Option<TokenType> {
        let parsed_token = self.tokenizer.tokenize(valid_tokens.clone(), &mut self.manager);

        parsed_token.map(|token| {
            let r#type = token.r#type();

            if !valid_tokens.contains(&r#type) {
                panic!("Parse Error: Expected {:?} but found {:?}", valid_tokens, r#type);
            }

            self.save_parsed_token(token);

            r#type
        })
    }

    fn save_parsed_token(&mut self, token: Token) {
        self.parsed_token = Some(token);
    }
}
