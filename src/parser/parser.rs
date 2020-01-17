use std::rc::Rc;
use crate::lexer::Lexer as LexerTrait;
use super::rule::{Rule::{self, *}, Nonterminal};


pub struct Parser<'a, Node, TokenType> {
    pub grammar: Rc<Nonterminal<'a, Node, TokenType>>,
}

impl<'a, Node, TokenType> Parser<'a, Node, TokenType>
{
    pub fn parse<Lexer>(&mut self, source_string: Box<str>) -> Node
    where
        Lexer: LexerTrait,
    {
        let mut lexer = Lexer::new(source_string);
        let grammar = Rc::clone(&self.grammar);
        let rule = Rc::new(Nonterm(grammar));

        self.parse_rule(rule, &mut lexer)
    }

    pub fn parse_rule<'b, Lexer: LexerTrait>(
        &mut self,
        rule: Rc<Rule<'b, Node, TokenType>>,
        lexer: &mut Lexer,
    ) -> Node {
        unimplemented!();

        match &*rule {
            Nonterm(_) => {
                unimplemented!()
            },
            Term(token_type) => {
                unimplemented!()
            },
        }
    }
}
