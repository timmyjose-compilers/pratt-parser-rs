use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Div,
    Eof,
    LParen,
    Minus,
    Mod,
    Mul,
    Number,
    Plus,
    Pow,
    RParen,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                TokenType::Div => "/",
                TokenType::Eof => "eof",
                TokenType::LParen => "(",
                TokenType::Minus => "-",
                TokenType::Mod => "%",
                TokenType::Mul => "*",
                TokenType::Number => "number",
                TokenType::Plus => "+",
                TokenType::Pow => "^",
                TokenType::RParen => ")",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub spelling: String,
}

impl Token {
    pub fn new(kind: TokenType, spelling: String) -> Self {
        Token { kind, spelling }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{};{}>", self.kind, self.spelling)
    }
}
