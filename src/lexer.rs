use std::cell::Cell;

use crate::tokens::{Keyword, Separator, Token};
pub struct Lexer<'src> {
    pos: Cell<usize>,
    source: &'src str,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source, pos: 0.into() }
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
            (Some(Token::Number(top)), Some(Token::Number(bottom))) => Some(Token::Signature(top, bottom)),
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
        Some(Token::Number(num))
    }
    fn process_literal(&self) -> Option<Token<'src>> {
        let pos = self.pos.get();
        while !matches!(self.advance(), Some("\"")) {}
        self.peek()?;
        let literal = self.source.get(pos + 1..self.pos.get())?;
        Some(Token::Literal(literal))
    }
    fn process_separator(&self) -> Option<Token<'src>> {
        match self.peek() {
            Some("(") => Some(Token::Separator(Separator::LParan)),
            Some(")") => Some(Token::Separator(Separator::RParan)),
            Some("{") => Some(Token::Separator(Separator::LCurly)),
            Some("}") => Some(Token::Separator(Separator::RCurly)),
            Some(";") => Some(Token::Separator(Separator::Semicolon)),
            Some(",") => Some(Token::Separator(Separator::Comma)),
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
            "import" => Token::Keyword(Keyword::Import),
            "meta" => Token::Keyword(Keyword::Meta),
            "staff" => Token::Keyword(Keyword::Staff),
            "pickup" => Token::Keyword(Keyword::Pickup),
            "measure" => Token::Keyword(Keyword::Measure),
            "from" => Token::Keyword(Keyword::From),
            "with" => Token::Keyword(Keyword::With),
            "is" => Token::Keyword(Keyword::Is),
            "in" => Token::Keyword(Keyword::In),
            _ => Token::Identifier(literal),
        };
        Some(token)
    }
    fn advance(&self) -> Option<&'src str> {
        self.pos.set(self.pos.get() + 1);
        self.peek()
    }
    fn advance_filtered(&self) -> Option<&'src str> {
        loop {
            if let Some(value) = self.advance() {
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
}
