use super::lex_manager::LexManager;


pub struct Recorder<'a> {
    recording: &'a mut String,
}

impl<'a> Recorder<'a> {
    pub fn new(recording: &'a mut String) -> Recorder<'a>
    { Recorder { recording: recording } }

    pub fn advance(&mut self, lexer: &mut LexManager) {
        if let Some(c) = lexer.lookahead {
            self.push(c);
        }

        lexer.advance();
    }

    pub fn push(&mut self, c: char)
    { self.recording.push(c); }

    pub fn recording(&self) -> &String
    { self.recording }
}
