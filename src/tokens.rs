
#[derive(Debug, Clone, Copy)]
pub enum Separator {
    LParan,
    RParan,
    Semicolon,
    LCurly,
    RCurly,
    Comma,
}

impl std::fmt::Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Separator::LParan => write!(f, "{}", "("),
            Separator::RParan => write!(f, "{}", ")"),
            Separator::Semicolon => write!(f, "{}", ";"),
            Separator::Comma => write!(f, "{}", ","),
            Separator::LCurly => write!(f, "{}", "{"),
            Separator::RCurly => write!(f, "{}", "}"),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Note {
    C,
    Cs,
    Db,
    D,
    Ds,
    Eb,
    E,
    F,
    Fs,
    Gb,
    G,
    Gs,
    Ab,
    A,
    As,
    Bb,
    B,
}
#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Import,
    Meta,
    Staff,
    Pickup,
    Measure,
    From,
    With,
    Is,
    In
}
#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}
#[derive(Debug, Clone, Copy)]
pub enum Token<'src> {
    Separator(Separator, Location),
    Literal(&'src str, Location),
    Identifier(&'src str, Location),
    Signature(u32, u32, Location),
    Number(u32, Location),
    Note(Note, u8, Location),
    Keyword(Keyword, Location),
    EOF(Location)
}
#[derive(Debug)]
pub enum TokenType {
    Separator,
    Literal,
    Identifier,
    Signature,
    Number,
    Note,
    Keyword,
    EOF
}

impl<'src> std::fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Separator(sep, _) => write!(f, "{}", sep),
            Token::Literal(lit, _) => write!(f, "\"{}\"", lit),
            Token::Identifier(ident, _) => write!(f, "{}", ident),
            Token::Signature(top, bottom, _) => write!(f, "[{}/{}]", top, bottom),
            Token::Number(num, _) => write!(f, "{}", num),
            Token::Note(note, octave, _) => write!(f, "{:?}_{}", note, octave),
            Token::Keyword(word, _) => write!(f, "{}", format!("{:?}", word).to_lowercase()),
            Token::EOF(_) => write!(f, "EOF"),
        }
    }
}