use crate::lexer::position::Position;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Literals
    Integer,
    Float,
    String,
    Char,
    True,
    False,
    Null,

    // Keywords
    Fn,
    Let,
    Mut,
    Const,
    If,
    Else,
    While,
    For,
    In,
    Loop,
    Break,
    Continue,
    Return,
    Match,
    Struct,
    Class,
    Enum,
    Interface,
    Impl,
    Module,
    Import,
    Export,
    Async,
    Await,
    Thread,
    Unsafe,
    Extern,
    As,
    Try,
    Catch,
    Throw,
    Operator,
    Static,
    Ref,
    Type,
    Trait,
    Where,
    Self_,
    Self_Cap,

    // Dependency keywords
    Require,
    Add,
    Pick,

    // Identifiers
    Identifier,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Increment,
    Decrement,

    // Comparison
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    AmpAmp,
    PipePipe,
    Bang,

    // Bitwise
    Amp,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,

    // Assignment
    Equal,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    AmpAssign,
    PipeAssign,
    CaretAssign,
    LeftShiftAssign,
    RightShiftAssign,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    DotDot,
    Colon,
    Semicolon,
    Question,

    // Arrows
    Arrow,      // ->
    FatArrow,   // =>

    // Pipelines
    Pipeline,   // |>

    // Special
    At,         // @
    Hash,       // #

    // End
    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
    pub position: Position,
}

impl Token {
    pub fn new(kind: TokenKind, position: Position) -> Self {
        Token {
            kind,
            value: None,
            position,
        }
    }

    pub fn with_value(kind: TokenKind, value: String, position: Position) -> Self {
        Token {
            kind,
            value: Some(value),
            position,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::Eof
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref val) = self.value {
            write!(f, "{:?}({})", self.kind, val)
        } else {
            write!(f, "{:?}", self.kind)
        }
    }
}
