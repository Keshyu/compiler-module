use super::parse_path::ParsePath;

pub struct Parser<'a, TokenType> {
   parse_path: ParsePath<'a, TokenType>,
}

impl<'a, TokenType> Parser<'a, TokenType> {
    pub fn new(parse_path: ParsePath<'a, TokenType>) -> Self {
        Self {
            parse_path: parse_path,
        }
    }
}
