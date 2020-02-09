use std::fmt::{Debug, Formatter, Result};


pub struct Parse();

impl Debug for Parse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<nonterminal parse function>")
    }
}
