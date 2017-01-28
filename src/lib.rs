pub type token_type i8;

pub const token_done: token_type = -1
pub const token_error: token_type = -2

pub struct token {
    type: token_type,
    data: String,
}

pub type token_fn fn(&mut parser) token;

pub trait tokeniser {
    fn next(&mut self) -> char;
    fn backup(&mut self) -> char;
    fn get(&mut self) -> String;
    fn len(&mut self) -> usize;
}

struct parser<T: tokeniser> {
    tokeniser: T,
    state: token_fn,
    err: String,
}

impl<T: tokeniser> parser {
    pub fn get(&mut self) -> token {
            self.state(self)
    }
    pub fn set_state(&mut self, func: token_fn) {
        self.state = func;
    }
    pub fn peek(&mut self) -> char {
        let c = self.tokeniser.next();
        self.tokeniser.backup();
        c
    }
    pub fn len(&mut self) -> usize {
        self.tokeniser.len()
    }
    pub fn accept(&mut self, chars: &str) -> bool {
        true
    }
    pub fn accept_run(&mut self, chars: &str) -> char {

    }
    pub fn except(&mut self, chars: &str) -> bool {
        true
    }
    pub fn except_run(&mut self, chars: &str) -> char {

    }
    pub fn done() -> token {
        self.set_state(self.done_state);
        self.done_state()
    }
    fn done_state() -> token {
        token {
            type: token_done,
            data: "",
        }
    }
    pub fn error(&mut self, err: &str) -> token {
        self.set_state(self.error_state);
        self.err = err;
        self.error_state()
    }
    fn error_state(&mut self) -> token {
        token {
            type: token_error,
            data: self.err,
        }
    }
}

pub type phrase_type i8;

pub const phrase_done: phrase_type = -1
pub const phrase_error: phrase_type = -2

pub type phrase_fn fn(&mut phraser) phrase;

struct phraser<T: parser> {
     parser: T,
     state phrase_fn,
}
