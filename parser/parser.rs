// Copyright 2022 Pekka Enberg and contributors
// SPDX-License-Identifier: MIT

//! ECMAScript Parser

use crate::ast::{
    BlockStatement, Expression, Identifier, ImportClause, ImportDeclaration, ModuleSpecifier,
    Script, Statement, VariableStatement,
};
use crate::tokenizer::{Token, Tokenizer};
use std::cell::RefCell;
use std::io::BufRead;

#[derive(Debug)]
pub struct ParseError {
    pub reason: String,
}

pub struct Parser<'a, R: BufRead> {
    preserve_comments: bool,
    tokenizer: Tokenizer<'a, R>,
    lookahead: RefCell<Option<Token>>,
}

impl<'a, R: BufRead> Parser<'a, R> {
    pub fn new(tokenizer: Tokenizer<'a, R>) -> Self {
        Self {
            preserve_comments: false,
            tokenizer,
            lookahead: RefCell::new(None),
        }
    }

    /// Parse a script.
    pub fn parse_script(&mut self) -> Result<Script, ParseError> {
        let mut stmts = Vec::new();
        loop {
            match self.parse_statement()? {
                Some(stmt) => stmts.push(stmt),
                None => break,
            }
        }
        Ok(Script::new(BlockStatement { stmts }))
    }

    /// Parse a statement or a declaration.
    fn parse_statement(&mut self) -> Result<Option<Statement>, ParseError> {
        while let Some(token) = self.next_token() {
            match token {
                Token::SingleLineComment => {
                    if self.preserve_comments {
                        return self.parse_comment();
                    }
                }
                Token::ImportKeyword => return self.parse_import_declaration(),
                Token::ConstKeyword | Token::LetKeyword | Token::VarKeyword => {
                    return self.parse_variable_declaration()
                }
                _ => {
                    return Err(ParseError {
                        reason: format!("Unexpected token: {}", self.tokenizer.slice()),
                    })
                }
            }
        }
        Ok(None) // EOF
    }

    fn parse_comment(&mut self) -> Result<Option<Statement>, ParseError> {
        Ok(Some(Statement::Comment {}))
    }

    // Parse an import declaration.
    fn parse_import_declaration(&mut self) -> Result<Option<Statement>, ParseError> {
        if let Some(import_clause) = self.parse_import_clause()? {
            if let Some(from_clause) = self.parse_from_clause()? {
                return Ok(Some(Statement::ImportDeclaration(ImportDeclaration {
                    import_clause: Some(import_clause),
                    module_specifier: from_clause,
                })));
            } else {
                return Err(ParseError {
                    reason: "Expression expected.".to_string(),
                });
            }
        }
        if let Some(module_specifier) = self.parse_module_specifier()? {
            return Ok(Some(Statement::ImportDeclaration(ImportDeclaration {
                import_clause: None,
                module_specifier,
            })));
        }
        Err(ParseError {
            reason: "Declaration or statement expected.".to_string(),
        })
    }

    fn parse_import_clause(&mut self) -> Result<Option<ImportClause>, ParseError> {
        match self.peek_token() {
            Some(Token::LeftBrace) => self.parse_named_imports(),
            _ => Ok(None),
        }
    }

    fn parse_named_imports(&mut self) -> Result<Option<ImportClause>, ParseError> {
        self.expect_token(Token::LeftBrace)?;
        let mut import_specifiers = vec![];
        loop {
            match self.next_token() {
                Some(Token::IdentifierName) => {
                    let import_specifier = self.tokenizer.slice().to_string();
                    import_specifiers.push(import_specifier);
                }
                Some(Token::Comma) => continue,
                Some(Token::RightBrace) => break,
                _ => {
                    return Err(ParseError {
                        reason: "Identifier expected".to_string(),
                    })
                }
            }
        }
        Ok(Some(ImportClause::NamedImports(import_specifiers)))
    }

    fn parse_from_clause(&mut self) -> Result<Option<ModuleSpecifier>, ParseError> {
        if let Some(_) = self.next_token() {
            // FIXME: The underlying tokenizer could have advanced to a different token.
            // Let's fix this by storing the slice as part of the lookahead.
            if self.tokenizer.slice() != "from" {
                return Err(ParseError {
                    reason: "`from` expected".to_string(),
                });
            }
            self.parse_module_specifier()
        } else {
            Err(ParseError {
                reason: "`from` expected".to_string(),
            })
        }
    }

    fn parse_module_specifier(&mut self) -> Result<Option<ModuleSpecifier>, ParseError> {
        if let Some(Token::StringLiteral) = self.peek_token() {
            self.expect_token(Token::StringLiteral)?;
            Ok(Some(ModuleSpecifier {
                value: self.tokenizer.slice().to_string(),
            }))
        } else {
            Ok(None)
        }
    }

    // Parse a variable declaration.
    fn parse_variable_declaration(&mut self) -> Result<Option<Statement>, ParseError> {
        if let Some(binding_identifier) = self.parser_binding_identifier()? {
            let initializer = self.parse_initializer()?;
            return Ok(Some(Statement::VariableStatement(VariableStatement {
                binding_identifier,
                initializer,
            })));
        }
        todo!("Unexpected token: `{:?}`", self.next_token());
    }

    fn parser_binding_identifier(&mut self) -> Result<Option<Identifier>, ParseError> {
        match self.peek_token() {
            Some(Token::IdentifierName) => {
                self.next_token();
                Ok(Some(Identifier {
                    value: self.tokenizer.slice().to_string(),
                }))
            }
            _ => Ok(None),
        }
    }

    fn parse_initializer(&mut self) -> Result<Option<Expression>, ParseError> {
        match self.peek_token() {
            Some(Token::Assignment) => {
                self.next_token();
                self.parse_assignment_expression()
            }
            _ => Ok(None),
        }
    }

    fn parse_assignment_expression(&mut self) -> Result<Option<Expression>, ParseError> {
        todo!();
    }

    fn peek_token(&mut self) -> Option<Token> {
        if let Ok(lookahead) = self.lookahead.try_borrow() {
            let lookahead = (*lookahead).clone();
            if let Some(lookahead) = lookahead {
                return Some(lookahead);
            }
        }
        let token = self.tokenizer.next_token();
        self.lookahead.replace(token.clone());
        token
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.next_token() {
            Some(actual) if actual == expected => Ok(()),
            actual => Err(ParseError {
                reason: format!("Expected token `{:?}`, but was `{:?}`", expected, actual),
            }),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if let Some(token) = self.lookahead.take() {
            return Some(token);
        }
        self.tokenizer.next_token()
    }
}
