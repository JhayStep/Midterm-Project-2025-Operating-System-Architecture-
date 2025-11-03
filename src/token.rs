#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    Number(i64),
}
