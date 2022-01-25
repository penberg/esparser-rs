// Copyright 2022 Pekka Enberg and contributors
// SPDX-License-Identifier: MIT

//! ECMAScript Abstract Syntax Tree (AST)

/// A script.
#[derive(Debug)]
pub struct Script {
    pub body: BlockStatement,
}

impl Script {
    pub fn new(body: BlockStatement) -> Self {
        Self { body }
    }
}

// An identifier.
#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

// An expression.
#[derive(Debug)]
pub enum Expression {
    /// A binary expression.
    BinaryExpression(Box<BinaryExpression>),
}

/// A binary expression.
///
/// For example, in ECMAScript, `x + y` is a binary expression where left side
/// of the expression is the identifier `x`, the binary operator is `+`, and
/// the right side fo the expression is `y`.
#[derive(Debug)]
pub struct BinaryExpression {
    /// Left side of this binary expression.
    pub left: Expression,
    /// Binary operator of this expression.
    pub op: BinaryOp,
    /// Right side of this binary expression.
    pub right: Expression,
}

/// A binary operator.
#[derive(Debug)]
pub enum BinaryOp {
    Addition,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Division,
    Equality,
    Exponentiation,
    GreaterThan,
    GreaterThanOrEqual,
    Inequality,
    LeftShift,
    LessThan,
    LessThanOrEqual,
    LogicalAnd,
    LogicalOr,
    Multiplication,
    Remainder,
    RightSift,
    StrictEquality,
    StrictInequality,
    Subtraction,
    UnsignedRightShift,
}

/// A statement or a declaration.
#[derive(Debug)]
pub enum Statement {
    BlockStatement(BlockStatement),       // Block statement
    BreakStatement,                       // `break` statement
    Comment,                              // Comment.
    ContinueStatement,                    // `continue` statement
    Expressiontatement,                   // Expression statement
    ForStatement,                         // `for` statement
    FunctionDeclaration,                  // Function declaration
    IfStatement,                          // `if` statement
    ImportDeclaration(ImportDeclaration), // `import` declaration
    VariableStatement(VariableStatement), // Variable statement
}

/// A block statement.
#[derive(Debug)]
pub struct BlockStatement {
    pub stmts: Vec<Statement>,
}

/// An import declaration.
#[derive(Debug)]
pub struct ImportDeclaration {
    pub import_clause: Option<ImportClause>,
    pub module_specifier: ModuleSpecifier,
}

/// A from clause.
#[derive(Debug)]
pub struct ModuleSpecifier {
    pub value: String,
}

/// An import clause.
#[derive(Debug)]
pub enum ImportClause {
    NamedImports(Vec<String>),
}

/// A variable statement.
#[derive(Debug)]
pub struct VariableStatement {
    pub binding_identifier: Identifier,
    pub initializer: Option<Expression>,
}
