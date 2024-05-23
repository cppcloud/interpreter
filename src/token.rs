use std::fmt::Display;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Debug, Default, Clone, Hash, Eq)]
pub enum TokenKind {
    #[default]
    Illegal,
    Eof,

    Ident,
    Int,
    String,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,

    Comma,
    Semicolon,
    Colon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        match self {
            Illegal => write!(f, "Illegal"),
            Eof => write!(f, "Eof"),
            Ident => write!(f, "Ident"),
            Int => write!(f, "Int"),
            String => write!(f, "String"),
            Assign => write!(f, "="),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Bang => write!(f, "!"),
            Asterisk => write!(f, "*"),
            Slash => write!(f, "/"),
            Lt => write!(f, "<"),
            Gt => write!(f, ">"),
            Eq => write!(f, "=="),
            NotEq => write!(f, "!="),
            Comma => write!(f, ","),
            Semicolon => write!(f, ";"),
            Colon => write!(f, ":"),
            Lparen => write!(f, "("),
            Rparen => write!(f, ")"),
            Lbrace => write!(f, "{{"),
            Rbrace => write!(f, "}}"),
            Lbracket => write!(f, "["),
            Rbracket => write!(f, "]"),
            Function => write!(f, "fn"),
            Let => write!(f, "let"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            Return => write!(f, "return"),
        }
    }
}

pub fn lookup_ident(identifier: &str) -> TokenKind {
    match identifier {
        "fn" => TokenKind::Function,
        "let" => TokenKind::Let,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        _ => TokenKind::Ident,
    }
}
