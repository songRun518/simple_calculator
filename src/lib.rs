mod calculator;

pub use calculator::Calculator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LParen,
    RParen,
    Invalid(char),
}
