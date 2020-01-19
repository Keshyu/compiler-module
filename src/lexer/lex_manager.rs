use std::vec::IntoIter;
use super::recorder::Recorder;


pub struct LexManager {
    pub source: IntoIter<char>,
    pub lookahead: Option<char>,
}

impl LexManager {
    pub fn record(
        &mut self,
        rec: impl Fn(&mut LexManager, &mut Recorder) -> ()
    ) -> String {
        let mut recording = String::new();
        let mut recorder = Recorder::new(&mut recording);

        rec(self, &mut recorder);

        recording
    }

    pub fn check_for(&self, checkings: &[char]) -> bool
    { checkings.iter().any(|c| { self.check(*c) }) }

    pub fn check(&self, checking: char) -> bool
    { self.check_if( |c| c == checking ) }

    pub fn check_if(&self, predicate: impl Fn(char) -> bool) -> bool
    { self.lookahead.map_or(false, predicate) }

    pub fn advance(&mut self)
    { self.lookahead = self.source.next() }

    pub fn at_end(&self) -> bool
    { self.lookahead.is_none() }
}
