// Copyright 2022 Pekka Enberg and contributors
// SPDX-License-Identifier: MIT

//! ECMAScript Tokenizer
//!
//! The tokenizer transforms a stream of characters into a stream of tokens
//! that represent, for example, ECMAScript identifiers, literals, operators,
//! and other grammatical elements.
//!
//! The tokenization is implemented lazily. The tokenizer transforms the
//! character stream one token at a time.

use std::collections::vec_deque::VecDeque;
use std::io::BufRead;
use utf8_chars::Chars;

/// ECMAScript token enumeration.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    AdditionAssignment,           // +=
    Ampersand,                    // &
    Arrow,                        // =>
    Assignment,                   // =
    Asterisk,                     // *
    BitwiseAndAssignment,         // &=
    BitwiseOrAssignment,          // |=
    BitwiseXorAssignment,         // ^=
    Caret,                        // ^
    Colon,                        // :
    Comma,                        // ,
    ConstKeyword,                 // const
    Decrement,                    // --
    DivisonAssignment,            // /=
    Dot,                          // .
    Equality,                     // ==
    ExclamationMark,              // !
    Exponentation,                // **
    ExponentationAssignment,      // **=
    GreaterThanOrEqual,           // >=
    IdentifierName,               // Identifier.
    ImportKeyword,                // import
    Increment,                    // ++
    Inequality,                   // !=
    LeftAngleBracket,             // <
    LeftBrace,                    // {
    LeftParenthesis,              // (
    LeftShift,                    // <<
    LeftShiftAssignment,          // <<=
    LeftSquareBracket,            // [
    LessThanOrEqual,              // <=
    LetKeyword,                   // let
    LogicalAnd,                   // &&
    LogicalAndAssignment,         // &&=
    LogicalNullishAssignment,     // ??=
    LogicalOr,                    // ||
    LogicalOrAssignment,          // ||=
    Minus,                        // -
    MultiLineComment,             // /* [...] */
    MultiplicationAssignment,     // *=
    NullishCoalescingOperator,    // ??
    NumericLiteral,               // Numeric literal
    OptionalChaining,             // ?.
    Percent,                      // %
    Pipe,                         // |
    Plus,                         // +
    QuestionMark,                 // ?
    RemainderAssignment,          // %=
    RightAngleBracket,            // >
    RightBrace,                   // }
    RightParenthesis,             // )
    RightShift,                   // >>
    RightShiftAssignment,         // >>=
    RightSquareBracket,           // ]
    Semicolon,                    // ;
    SingleLineComment,            // // [...]
    Slash,                        // /
    Spread,                       // ...
    StrictEquality,               // ===
    StrictInequality,             // !==
    StringLiteral,                // String literal. For example, "hello, world"
    SubtractionAssignment,        // -=
    TemplateLiteral,              // Template literal. For example: `hello, world`
    Tilde,                        // ~
    UnsignedRightShift,           // >>>
    UnsignedRightShiftAssignment, // >>>=
    VarKeyword,                   // var
}

pub struct Tokenizer<'a, R: BufRead> {
    chars: Chars<'a, R>,
    lookaheads: VecDeque<char>,
    slice: String,
}

impl<'a, R: BufRead> Tokenizer<'a, R> {
    pub fn new(chars: Chars<'a, R>) -> Self {
        Self {
            chars,
            lookaheads: VecDeque::new(),
            slice: String::new(),
        }
    }

    /// Returns the next token in the token stream.
    ///
    /// The tokenizer ignores any whitespace.
    pub fn next_token(&mut self) -> Option<Token> {
        self.get_next_token()
    }

    fn get_next_token(&mut self) -> Option<Token> {
        let mut ch = self.next_char();
        while let Some(c) = ch {
            if !c.is_whitespace() {
                break;
            }
            ch = self.consume_char_and_peek();
        }
        self.slice.clear();
        match ch {
            Some(ch) if ch.is_alphabetic() => self.consume_identifier(),
            Some(ch) if ch.is_numeric() => self.consume_numeric_literal(),
            Some('!') => match self.consume_char_and_peek() {
                Some('=') => match self.consume_char_and_peek() {
                    Some('=') => self.consume_char_as(Token::StrictInequality),
                    _ => Some(Token::Inequality),
                },
                _ => Some(Token::ExclamationMark),
            },
            Some('"') => self.consume_double_quote_string_literal(),
            Some('%') => match self.consume_char_and_peek() {
                Some('=') => self.consume_char_as(Token::RemainderAssignment),
                _ => Some(Token::Percent),
            },
            Some('&') => match self.consume_char_and_peek() {
                Some('&') => match self.consume_char_and_peek() {
                    Some('=') => self.consume_char_as(Token::LogicalAndAssignment),
                    _ => Some(Token::LogicalAnd),
                },
                Some('=') => self.consume_char_as(Token::BitwiseAndAssignment),
                _ => Some(Token::Ampersand),
            },
            Some('\'') => self.consume_single_quote_string_literal(),
            Some('(') => self.consume_char_as(Token::LeftParenthesis),
            Some(')') => self.consume_char_as(Token::RightParenthesis),
            Some('*') => match self.consume_char_and_peek() {
                Some('*') => match self.consume_char_and_peek() {
                    Some('=') => self.consume_char_as(Token::ExponentationAssignment),
                    _ => Some(Token::Exponentation),
                },
                Some('=') => self.consume_char_as(Token::MultiplicationAssignment),
                _ => Some(Token::Asterisk),
            },
            Some('+') => match self.consume_char_and_peek() {
                Some('+') => self.consume_char_as(Token::Increment),
                Some('=') => self.consume_char_as(Token::AdditionAssignment),
                _ => Some(Token::Plus),
            },
            Some(',') => self.consume_char_as(Token::Comma),
            Some('-') => match self.consume_char_and_peek() {
                Some('-') => self.consume_char_as(Token::Decrement),
                Some('=') => self.consume_char_as(Token::SubtractionAssignment),
                _ => Some(Token::Minus),
            },
            Some('.') => {
                if self.peek_char() == Some('.') && self.peek_char() == Some('.') {
                    self.consume_char();
                    self.consume_char();
                    self.consume_char_as(Token::Spread)
                } else {
                    self.consume_char_as(Token::Dot)
                }
            }
            Some('/') => match self.consume_char_and_peek() {
                Some('/') => {
                    self.consume_char();
                    self.consume_single_line_comment()
                }
                Some('=') => self.consume_char_as(Token::DivisonAssignment),
                _ => Some(Token::Slash),
            },
            Some(':') => self.consume_char_as(Token::Colon),
            Some(';') => self.consume_char_as(Token::Semicolon),
            Some('<') => {
                ch = self.consume_char_and_peek();
                if ch == Some('<') {
                    ch = self.consume_char_and_peek();
                    if ch == Some('=') {
                        self.consume_char_as(Token::LeftShiftAssignment)
                    } else {
                        Some(Token::LeftShift)
                    }
                } else if ch == Some('=') {
                    self.consume_char_as(Token::LessThanOrEqual)
                } else {
                    Some(Token::LeftAngleBracket)
                }
            }
            Some('=') => match self.consume_char_and_peek() {
                Some('=') => match self.consume_char_and_peek() {
                    Some('=') => self.consume_char_as(Token::StrictEquality),
                    _ => Some(Token::Equality),
                },
                Some('>') => self.consume_char_as(Token::Arrow),
                _ => Some(Token::Assignment),
            },
            Some('>') => {
                ch = self.consume_char_and_peek();
                if ch == Some('=') {
                    self.consume_char_as(Token::GreaterThanOrEqual)
                } else if ch == Some('>') {
                    ch = self.consume_char_and_peek();
                    if ch == Some('=') {
                        self.consume_char_as(Token::RightShiftAssignment)
                    } else if ch == Some('>') {
                        ch = self.consume_char_and_peek();
                        if ch == Some('=') {
                            self.consume_char_as(Token::UnsignedRightShiftAssignment)
                        } else {
                            Some(Token::UnsignedRightShift)
                        }
                    } else {
                        Some(Token::RightShift)
                    }
                } else {
                    Some(Token::RightSquareBracket)
                }
            }
            Some('}') => self.consume_char_as(Token::RightBrace),
            Some('?') => match self.consume_char_and_peek() {
                Some('?') => match self.consume_char_and_peek() {
                    Some('=') => self.consume_char_as(Token::LogicalNullishAssignment),
                    _ => Some(Token::NullishCoalescingOperator),
                },
                Some('.') => self.consume_char_as(Token::OptionalChaining),
                _ => Some(Token::QuestionMark),
            },
            Some('[') => self.consume_char_as(Token::LeftSquareBracket),
            Some(']') => self.consume_char_as(Token::RightSquareBracket),
            Some('^') => match self.consume_char_and_peek() {
                Some('=') => self.consume_char_as(Token::BitwiseXorAssignment),
                _ => Some(Token::Caret),
            },
            Some('{') => self.consume_char_as(Token::LeftBrace),
            Some('|') => match self.consume_char_and_peek() {
                Some('|') => match self.consume_char_and_peek() {
                    Some('=') => self.consume_char_as(Token::LogicalOrAssignment),
                    _ => Some(Token::LogicalOr),
                },
                Some('=') => self.consume_char_as(Token::BitwiseOrAssignment),
                _ => Some(Token::Pipe),
            },
            Some('~') => self.consume_char_as(Token::Tilde),
            Some(ch) => {
                todo!("Token starting with character `{}` is not recognized", ch)
            }
            None => None,
        }
    }

    pub fn slice(&self) -> &str {
        &self.slice
    }

    fn consume_identifier(&mut self) -> Option<Token> {
        let mut ch = self.peek_char();
        while let Some(c) = ch {
            if !c.is_alphanumeric() && c != '_' {
                self.consume_char();
                break;
            }
            self.consume_char();
            ch = self.peek_char();
        }
        match self.slice() {
            "const" => Some(Token::ConstKeyword),
            "import" => Some(Token::ImportKeyword),
            "let" => Some(Token::LetKeyword),
            "var" => Some(Token::VarKeyword),
            _ => Some(Token::IdentifierName),
        }
    }

    fn consume_numeric_literal(&mut self) -> Option<Token> {
        // FIXME: decimals and other fancy numeric literals are not supported.
        let mut ch = self.peek_char();
        while let Some(c) = ch {
            if !c.is_numeric() && c != '_' {
                self.consume_char();
                break;
            }
            self.consume_char();
            ch = self.peek_char();
        }
        Some(Token::NumericLiteral)
    }

    fn consume_double_quote_string_literal(&mut self) -> Option<Token> {
        let mut prev = self.consume_next_char();
        while let Some(ch) = self.consume_next_char() {
            if prev != Some('\\') && ch == '\"' {
                break;
            }
            prev = Some(ch);
        }
        Some(Token::StringLiteral)
    }

    fn consume_single_quote_string_literal(&mut self) -> Option<Token> {
        let mut prev = self.consume_next_char();
        while let Some(ch) = self.consume_next_char() {
            if prev != Some('\\') && ch == '\'' {
                break;
            }
            prev = Some(ch);
        }
        Some(Token::StringLiteral)
    }

    fn consume_single_line_comment(&mut self) -> Option<Token> {
        let mut ch = self.peek_char();
        while let Some(c) = ch {
            if c == '\n' {
                break;
            }
            self.consume_char();
            ch = self.peek_char();
        }
        Some(Token::SingleLineComment)
    }

    fn consume_char_as(&mut self, token: Token) -> Option<Token> {
        self.consume_char();
        Some(token)
    }

    fn consume_char_and_peek(&mut self) -> Option<char> {
        self.consume_char();
        self.peek_char()
    }

    fn consume_char(&mut self) {
        // If we have a lookahead, consume it; otherwise consume from the
        // character stream.
        if let Some(ch) = self.lookaheads.pop_front() {
            self.slice.push(ch);
        } else if let Some(Ok(ch)) = self.chars.next() {
            self.slice.push(ch);
        }
    }

    fn consume_next_char(&mut self) -> Option<char> {
        let ch = self.next_char();
        self.consume_char();
        ch
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(ch) = self.lookaheads.front() {
            Some(*ch)
        } else {
            self.peek_char()
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        // Read from the character stream and insert to the vector of
        // lookaheads if we read something.
        match self.chars.next() {
            Some(Ok(lookahead)) => {
                self.lookaheads.push_back(lookahead);
                Some(lookahead)
            }
            _ => None,
        }
    }
}
