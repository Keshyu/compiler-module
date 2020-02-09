use super::grammar::Grammar;
use super::parse_command::ParseCommand;

pub struct ParsePath<'a, TokenType>(Vec<ParseCommand<'a, TokenType>>);

impl<'a, TokenType: Clone> ParsePath<'a, TokenType> {
    pub fn new() -> Self {
        ParsePath(Vec::new())
    }

    pub fn from_grammar(grammar: &Grammar<'a, TokenType>, path_id: Option<u32>) -> Self {
        use Grammar::*;

        let mut parse_path = ParsePath::new();

        match grammar {
            Sequence(rules) => {
                rules.into_iter().for_each(|rule| {
                    parse_path.append(&mut Self::from_grammar(&*rule, None));
                });
            }
            Choice(_rules) => {
                // TODO: Implement rule choosing
            }
            Terminal(ttype) => {
                parse_path.push(ParseCommand::Consume(ttype.clone()));
            }
            Nonterminal(parse, rule) => {
                parse_path.push(ParseCommand::NonterminalStart {
                    id: path_id,
                    parse: parse,
                });

                parse_path
                    .append(&mut Self::from_grammar(&*rule, path_id));

                parse_path
                    .push(ParseCommand::NonterminalEnd { id: path_id });
            }
        }

        parse_path
    }

    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0)
    }

    pub fn push(&mut self, command: ParseCommand<'a, TokenType>) {
        self.0.push(command);
    }
}

impl<'a, TokenType: Clone> From<&Grammar<'a, TokenType>> for ParsePath<'a, TokenType> {
    fn from(grammar: &Grammar<'a, TokenType>) -> Self {
        ParsePath::from_grammar(grammar, None)
    }
}
