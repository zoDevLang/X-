pub mod token;
pub mod position;

use token::{Token, TokenKind};
use position::Position;
use anyhow::{anyhow, Result};
use std::str::Chars;
use std::iter::Peekable;

pub fn lex(source: &str) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

struct Lexer {
    source: String,
    chars: Peekable<Chars<'static>>,
    position: Position,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(source: &str) -> Self {
        let source_owned = source.to_string();
        let chars = unsafe {
            // SAFETY: source_owned is never moved while chars lives
            std::mem::transmute::<Chars<'_>, Chars<'static>>(source_owned.chars())
        };
        
        Lexer {
            source: source_owned,
            chars: chars.peekable(),
            position: Position::new(),
            tokens: Vec::new(),
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>> {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.position.newline();
                    self.advance();
                }
                '/' if self.peek_ahead(1) == Some('/') => {
                    self.skip_line_comment();
                }
                '/' if self.peek_ahead(1) == Some('*') => {
                    self.skip_block_comment()?;
                }
                '"' => {
                    self.tokens.push(self.string_literal()?);
                }
                '\''. => {
                    self.tokens.push(self.char_literal()?);
                }
                '0'..='9' => {
                    self.tokens.push(self.number()?);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.tokens.push(self.identifier_or_keyword());
                }
                '+' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::PlusAssign, pos));
                    } else if self.chars.peek() == Some(&'+') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::Increment, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Plus, pos));
                    }
                }
                '-' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::MinusAssign, pos));
                    } else if self.chars.peek() == Some(&'-') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::Decrement, pos));
                    } else if self.chars.peek() == Some(&'>') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::Arrow, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Minus, pos));
                    }
                }
                '*' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::StarAssign, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Star, pos));
                    }
                }
                '/' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::SlashAssign, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Slash, pos));
                    }
                }
                '%' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::PercentAssign, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Percent, pos));
                    }
                }
                '=' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::EqualEqual, pos));
                    } else if self.chars.peek() == Some(&'>') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::FatArrow, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Equal, pos));
                    }
                }
                '!' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::BangEqual, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Bang, pos));
                    }
                }
                '<' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::LessEqual, pos));
                    } else if self.chars.peek() == Some(&'<') {
                        self.advance();
                        if self.chars.peek() == Some(&'=') {
                            self.advance();
                            self.tokens.push(Token::new(TokenKind::LeftShiftAssign, pos));
                        } else {
                            self.tokens.push(Token::new(TokenKind::LeftShift, pos));
                        }
                    } else {
                        self.tokens.push(Token::new(TokenKind::Less, pos));
                    }
                }
                '>' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::GreaterEqual, pos));
                    } else if self.chars.peek() == Some(&'>') {
                        self.advance();
                        if self.chars.peek() == Some(&'=') {
                            self.advance();
                            self.tokens.push(Token::new(TokenKind::RightShiftAssign, pos));
                        } else {
                            self.tokens.push(Token::new(TokenKind::RightShift, pos));
                        }
                    } else {
                        self.tokens.push(Token::new(TokenKind::Greater, pos));
                    }
                }
                '&' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'&') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::AmpAmp, pos));
                    } else if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::AmpAssign, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Amp, pos));
                    }
                }
                '|' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'|') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::PipePipe, pos));
                    } else if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::PipeAssign, pos));
                    } else if self.chars.peek() == Some(&'>') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::Pipeline, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Pipe, pos));
                    }
                }
                '^' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'=') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::CaretAssign, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Caret, pos));
                    }
                }
                '~' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::Tilde, pos));
                }
                '?' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::Question, pos));
                }
                ':' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::Colon, pos));
                }
                ';' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::Semicolon, pos));
                }
                ',' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::Comma, pos));
                }
                '.' => {
                    let pos = self.position.clone();
                    self.advance();
                    if self.chars.peek() == Some(&'.') {
                        self.advance();
                        self.tokens.push(Token::new(TokenKind::DotDot, pos));
                    } else {
                        self.tokens.push(Token::new(TokenKind::Dot, pos));
                    }
                }
                '(' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::LParen, pos));
                }
                ')' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::RParen, pos));
                }
                '{' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::LBrace, pos));
                }
                '}' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::RBrace, pos));
                }
                '[' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::LBracket, pos));
                }
                ']' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::RBracket, pos));
                }
                '@' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::At, pos));
                }
                '#' => {
                    let pos = self.position.clone();
                    self.advance();
                    self.tokens.push(Token::new(TokenKind::Hash, pos));
                }
                _ => {
                    return Err(anyhow!("Unexpected character '{}' at {}", ch, self.position));
                }
            }
        }

        self.tokens.push(Token::new(TokenKind::Eof, self.position.clone()));
        Ok(self.tokens.clone())
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.position.advance(ch);
            Some(ch)
        } else {
            None
        }
    }

    fn peek_ahead(&mut self, n: usize) -> Option<char> {
        let mut chars = self.chars.clone();
        for _ in 0..n {
            chars.next()?;
        }
        chars.next()
    }

    fn skip_line_comment(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) -> Result<()> {
        self.advance(); // /
        self.advance(); // *
        let start_pos = self.position.clone();

        while let Some(ch) = self.advance() {
            if ch == '*' && self.chars.peek() == Some(&'/') {
                self.advance();
                return Ok(());
            }
        }

        Err(anyhow!("Unterminated block comment starting at {}", start_pos))
    }

    fn string_literal(&mut self) -> Result<Token> {
        let pos = self.position.clone();
        self.advance(); // "
        let mut value = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch == '"' {
                self.advance();
                return Ok(Token::with_value(TokenKind::String, value, pos));
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.advance() {
                    match escaped {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        _ => {
                            value.push('\\');
                            value.push(escaped);
                        }
                    }
                }
            } else {
                value.push(self.advance().unwrap());
            }
        }

        Err(anyhow!("Unterminated string starting at {}", pos))
    }

    fn char_literal(&mut self) -> Result<Token> {
        let pos = self.position.clone();
        self.advance(); // '
        let mut value = String::new();

        if self.chars.peek() == Some(&'\\') {
            self.advance();
            if let Some(escaped) = self.advance() {
                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '\'' => value.push('\''),
                    _ => {
                        value.push('\\');
                        value.push(escaped);
                    }
                }
            }
        } else if let Some(ch) = self.advance() {
            value.push(ch);
        }

        if self.chars.peek() == Some(&'\'') {
            self.advance();
            Ok(Token::with_value(TokenKind::Char, value, pos))
        } else {
            Err(anyhow!("Unterminated character literal starting at {}", pos))
        }
    }

    fn number(&mut self) -> Result<Token> {
        let pos = self.position.clone();
        let mut value = String::new();
        let mut is_float = false;

        while let Some(&ch) = self.chars.peek() {
            match ch {
                '0'..='9' => {
                    value.push(self.advance().unwrap());
                }
                '.' if !is_float && self.peek_ahead(1).map_or(false, |c| c.is_ascii_digit()) => {
                    is_float = true;
                    value.push(self.advance().unwrap());
                }
                _ => break,
            }
        }

        Ok(Token::with_value(
            if is_float {
                TokenKind::Float
            } else {
                TokenKind::Integer
            },
            value,
            pos,
        ))
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let pos = self.position.clone();
        let mut value = String::new();

        while let Some(&ch) = self.chars.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    value.push(self.advance().unwrap());
                }
                _ => break,
            }
        }

        let kind = match value.as_str() {
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "const" => TokenKind::Const,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "loop" => TokenKind::Loop,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            "match" => TokenKind::Match,
            "struct" => TokenKind::Struct,
            "class" => TokenKind::Class,
            "enum" => TokenKind::Enum,
            "interface" => TokenKind::Interface,
            "impl" => TokenKind::Impl,
            "module" => TokenKind::Module,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "thread" => TokenKind::Thread,
            "unsafe" => TokenKind::Unsafe,
            "extern" => TokenKind::Extern,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "require" => TokenKind::Require,
            "add" => TokenKind::Add,
            "pick" => TokenKind::Pick,
            "as" => TokenKind::As,
            "try" => TokenKind::Try,
            "catch" => TokenKind::Catch,
            "throw" => TokenKind::Throw,
            "operator" => TokenKind::Operator,
            "static" => TokenKind::Static,
            "mut" => TokenKind::Mut,
            "ref" => TokenKind::Ref,
            "type" => TokenKind::Type,
            "trait" => TokenKind::Trait,
            "where" => TokenKind::Where,
            "self" => TokenKind::Self_,
            "Self" => TokenKind::Self_Cap,
            _ => TokenKind::Identifier,
        };

        Token::with_value(kind, value, pos)
    }
}
