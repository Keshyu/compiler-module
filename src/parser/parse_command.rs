use super::parse::Parse;

pub enum ParseCommand<'a, TokenType> {
    Consume(TokenType),
    Choose(Vec<TokenType>),

    EnterChooseMode,
    ExitChooseMode,

    NonterminalStart { id: Option<u32>, parse: &'a Parse },
    NonterminalEnd { id: Option<u32> },
}
