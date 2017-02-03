use std::ops;

pub type TokenType = i8;

pub const TOKEN_DONE: TokenType = -1;
pub const TOKEN_ERROR: TokenType = -2;

pub struct Token {
    pub typ: TokenType,
    pub data: String,
}

pub type TokenFn<T: Tokeniser> = fn(s: &mut Parser<T>) -> Token;

pub trait Tokeniser {
    fn next(&mut self) -> char;
    fn backup(&mut self) -> char;
    fn peek(&mut self) -> char {
        let c = self.next();
        self.backup();
        c
    }
    fn get(&mut self) -> String;
    fn len(&mut self) -> usize;
}

pub struct Parser<T: Tokeniser> {
    tokeniser: T,
    state: TokenFn<T>,
    err: String,
}

impl<T: Tokeniser> Parser<T> {
    pub fn new(t: T, state: TokenFn<T>) -> Parser<T> {
        Parser {
            tokeniser: t,
            state: state,
            err: String::new(),
        }
    }
    pub fn get(&mut self) -> Token {
        (self.state)(self)
    }
    pub fn set_state(&mut self, func: TokenFn<T>) {
        self.state = func;
    }
    pub fn peek(&mut self) -> char {
        self.tokeniser.peek()
    }
    pub fn len(&mut self) -> usize {
        self.tokeniser.len()
    }
    pub fn accept(&mut self, chars: &str) -> bool {
        chars.contains(self.tokeniser.next())
    }
    pub fn accept_run(&mut self, chars: &str) -> char {
        loop {
            let c = self.tokeniser.next();
            if !chars.contains(c) {
                return c;
            }
        }
    }
    pub fn except(&mut self, chars: &str) -> bool {
        !chars.contains(self.tokeniser.next())
    }
    pub fn except_run(&mut self, chars: &str) -> char {
        loop {
            let c = self.tokeniser.next();
            if chars.contains(c) {
                return c;
            }
        }
    }
    pub fn done(&mut self) -> Token {
        self.set_state(done_state);
        done_state(self)
    }
    pub fn error(&mut self, err: String) -> Token {
        self.err = err;
        self.set_state(error_state);
        error_state(self)
    }
}

fn done_state<T: Tokeniser>(_: &mut Parser<T>) -> Token {
    Token {
        typ: TOKEN_DONE,
        data: String::new(),
    }
}

fn error_state<T: Tokeniser>(s: &mut Parser<T>) -> Token {
    Token {
        typ: TOKEN_ERROR,
        data: s.err.clone(),
    }
}

pub type PhraseType = i8;

pub const PHRASE_DONE: PhraseType = -1;
pub const PHRASE_ERROR: PhraseType = -2;

pub struct Phrase {
    pub typ: PhraseType,
    pub data: Vec<Token>,
}

pub type PhraseFn<T: Tokeniser> = fn(&mut Phraser<T>) -> Phrase;

pub struct Phraser<T: Tokeniser> {
    parser: Parser<T>,
    state: PhraseFn<T>,
}

impl<T: Tokeniser> Phraser<T> {
    pub fn new(p: Parser<T>, s: PhraseFn<T>) -> Phraser<T> {
        Phraser {
            parser: p,
            state: s,
        }
    }
}

impl<T: Tokeniser> ops::Deref for Phraser<T> {
    type Target = Parser<T>;

    /// The deref function allows access to the wrapped parser.
    fn deref(&self) -> &Parser<T> {
        &self.parser
    }
}
