use std::cell::Cell;

use crate::tokens::{Keyword, Separator, Token, Location};
pub struct Lexer<'src> {
    pos: Cell<usize>,
    line: Cell<usize>,
    col: Cell<usize>,
    source: &'src str,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source, pos: 0.into(), line: 1.into(), col: 0.into() }
    }
    pub fn lex(&self) -> Option<Token<'src>> {
        let value = if self.pos.get() == 0 {
            self.peek()?
        } else {
            self.advance_filtered()?
        };
        match value {
            "\"" => self.process_literal(),
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.process_number(),
            "[" => self.process_signature(),
            "(" | ")" | "{" | "}" | ";" | "," => self.process_separator(),
            _ => self.process_word(),
        }
    }
    fn loc(&self) -> Location {
        Location { line: self.line.get(), col: self.col.get() }
    }
    fn process_signature(&self) -> Option<Token<'src>> {
        self.advance_filtered();
        let top = self.process_number();
        if !matches!(self.advance_filtered(), Some("/")) {
            return None;
        }
        self.advance_filtered();
        let bottom = self.process_number();
        if !matches!(self.advance_filtered(), Some("]")) {
            return None;
        }
        match (top, bottom) {
            (Some(Token::Number(top, _)), Some(Token::Number(bottom, _))) => Some(Token::Signature(top, bottom, self.loc())),
            _ => None
        }
    }
    fn process_number(&self) -> Option<Token<'src>> {
        let pos = self.pos.get();
        while let Some("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9") = self.advance() {
        }
        self.pos.set(self.pos.get() - 1);
        self.peek()?;
        let num: u32 = self.source.get(pos..=self.pos.get())?.parse().ok()?;
        Some(Token::Number(num, self.loc()))
    }
    fn process_literal(&self) -> Option<Token<'src>> {
        let pos = self.pos.get();
        while !matches!(self.advance(), Some("\"")) {}
        self.peek()?;
        let literal = self.source.get(pos + 1..self.pos.get())?;
        Some(Token::Literal(literal, self.loc()))
    }
    fn process_separator(&self) -> Option<Token<'src>> {
        match self.peek() {
            Some("(") => Some(Token::Separator(Separator::LParan, self.loc())),
            Some(")") => Some(Token::Separator(Separator::RParan, self.loc())),
            Some("{") => Some(Token::Separator(Separator::LCurly, self.loc())),
            Some("}") => Some(Token::Separator(Separator::RCurly, self.loc())),
            Some(";") => Some(Token::Separator(Separator::Semicolon, self.loc())),
            Some(",") => Some(Token::Separator(Separator::Comma, self.loc())),
            _ => None,
        }
    }
    fn process_word(&self) -> Option<Token<'src>> {
        let pos = self.pos.get();
        while !matches!(
            self.advance(),
            Some(" " | "\t" | "\n" | "\r" | "(" | ")" | "{" | "}" | ";" | ",")
        ) {}
        self.pos.set(self.pos.get() - 1);
        self.peek()?;
        let literal = self.source.get(pos..=self.pos.get())?;
        let token = match literal {
            "import" => Token::Keyword(Keyword::Import, self.loc()),
            "meta" => Token::Keyword(Keyword::Meta, self.loc()),
            "staff" => Token::Keyword(Keyword::Staff, self.loc()),
            "pickup" => Token::Keyword(Keyword::Pickup, self.loc()),
            "measure" => Token::Keyword(Keyword::Measure, self.loc()),
            "from" => Token::Keyword(Keyword::From, self.loc()),
            "with" => Token::Keyword(Keyword::With, self.loc()),
            "is" => Token::Keyword(Keyword::Is, self.loc()),
            "in" => Token::Keyword(Keyword::In, self.loc()),
            _ => Token::Identifier(literal, self.loc()),
        };
        Some(token)
    }
    fn advance(&self) -> Option<&'src str> {
        self.pos.set(self.pos.get() + 1);
        self.col.set(self.col.get() + 1);
        self.peek()
    }
    fn advance_filtered(&self) -> Option<&'src str> {
        loop {
            if let Some(value) = self.advance() {
                if Self::is_newline(value) {
                    self.line.set(self.line.get() + 1);
                    self.col.set(0);
                }
                if Self::is_whitespace(value) {
                    continue;
                }
                break Some(value);
            }
            return None;
        }
    }
    fn peek(&self) -> Option<&'src str> {
        self.source.get(self.pos.get()..=self.pos.get())
    }
    fn is_whitespace(value: &'src str) -> bool {
        match value {
            " " | "\t" | "\n" | "\r" => true,
            _ => false,
        }
    }
    fn is_newline(value: &'src str) -> bool {
        match value {
            "\n" => true,
            _ => false,
        }
    }
}
