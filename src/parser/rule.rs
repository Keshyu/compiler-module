use std::marker::Sync;
use std::rc::Rc;
// use crate::tokens::Tokens;
// use crate::nodes::Nodes;


pub enum Rule<'a, Node, TokenType> {
    // OneZero(Rc<Rule<'a, Node, TokenType>>),
    // Sequence(Rules<'a, Node, TokenType>),
    // Choice(Rules<'a, Node, TokenType>),
    Nonterm(Rc<Nonterminal<'a, Node, TokenType>>),
    Term(TokenType),
}

pub struct Nonterminal<'a, Node, TokenType>(
    Rc<Rule<'a, Node, TokenType>>,
    &'a (dyn (Fn() -> Node) + Sync)
);

pub type Rules<'a, Node, TokenType> = Vec<Rc<Rule<'a, Node, TokenType>>>;
