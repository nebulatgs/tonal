
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
pub enum Token<'src> {
    Separator(Separator),
    Literal(&'src str),
    Identifier(&'src str),
    Signature(u32, u32),
    Number(u32),
    Note(Note, u8),
    Keyword(Keyword),
    EOF
}

impl<'src> std::fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Separator(sep) => write!(f, "{}", sep),
            Token::Literal(lit) => write!(f, "\"{}\"", lit),
            Token::Identifier(ident) => write!(f, "{}", ident),
            Token::Signature(top, bottom) => write!(f, "[{}/{}]", top, bottom),
            Token::Number(num) => write!(f, "{}", num),
            Token::Note(note, octave) => write!(f, "{:?}_{}", note, octave),
            Token::Keyword(word) => write!(f, "{}", format!("{:?}", word).to_lowercase()),
            Token::EOF => write!(f, ""),
        }
    }
}