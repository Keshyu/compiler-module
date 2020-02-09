use std::rc::Rc;
use super::parse::Parse;


#[derive(Debug)]
pub enum Grammar<'a, TokenType> {
    Sequence(Vec<Rc<Self>>),
    Choice(Vec<Rc<Self>>),
    Terminal(TokenType),
    Nonterminal(&'a Parse, Rc<Self>),
}
