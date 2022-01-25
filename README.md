# JavaScript and TypeScript parser for 🦀

[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE.md)

The aim for this project is to build a JavaScript and TypeScript parser for Rust. The goal is to make the parser easy to integrate into applications such as web development tooling that require the speed of Rust.
To accomplish the goal, we aim to make the parser API simple and generate an abstract syntax tree (AST), which is easy to consume.

But to set your expectations right, this project is in its infancy, so not that many JavaScript or TypeScript programs parse yet.
However, if you found a bug or have an itch to scratch, please open an issue or send a pull request to improve the parser!

## Getting Started

```console
cargo run tests/declarations.js
```

```console
Script {
    body: BlockStatement {
        stmts: [
            VariableStatement(
                VariableStatement {
                    binding_identifier: Identifier {
                        value: "y",
                    },
                },
            ),
        ],
    },
}
```

## Features and Roadmap

* [ ] Tokenizing 
  * [x] Double quote string literals
  * [ ] Multi-line comments
  * [ ] Numeric literals
  * [ ] Single quote string literals
  * [ ] Template literals
  * [x] Identifiers 
  * [x] Punctuators
  * [x] Single-line comments
* [ ] Expression parsing
  * [ ] Additive expression
  * [ ] Arguments expression
  * [ ] Array literal expression
  * [ ] Assignment expression
  * [ ] Assignment operator expression
  * [ ] Bitwise expressions
  * [ ] Cast as expression
  * [ ] Delete expression
  * [ ] Equality expression
  * [ ] Generators expression
  * [ ] Generators function expression
  * [ ] Identifier expression
  * [ ] In expression
  * [ ] Instanceof expression
  * [ ] Iterators expression
  * [ ] Literal expression
  * [ ] Logical expression
  * [ ] Multiplicative expression
  * [ ] New expression
  * [ ] Not expression
  * [ ] Object literal expression
  * [ ] Parenthesized expression
  * [ ] Post increment/decrease expressions
  * [ ] Pre increment/decrease expressions
  * [ ] Relational expression
  * [ ] Super expression
  * [ ] Template string expression
  * [ ] Ternary expression
  * [ ] This expression
  * [ ] Typeof expression
  * [ ] Unary plus/minus expression
  * [ ] Void expression
  * [ ] Yield expression
* [ ] Statement parsing
  * [ ] Array literals
  * [ ] Arrow function declaration
  * [ ] Block statement
  * [ ] Break statement
  * [ ] Class declaration
  * [ ] Continue statement
  * [ ] Export statement
  * [ ] Function declaration
  * [ ] Generator function declaration
  * [ ] If statement
  * [x] Import statement
  * [ ] Interface declaration
  * [ ] Iteration statement
  * [ ] Labeled statement
  * [ ] Namespace declaration
  * [ ] Object literals
  * [ ] Return statement
  * [ ] Switch statement
  * [ ] Throw statement
  * [ ] Try statement
  * [ ] Variable declaration
  * [ ] With statement
  * [ ] Yield statement

## References

* [ANTLR Grammar for TypeScript](https://github.com/antlr/grammars-v4/tree/master/javascript/typescript)
* [ECMAScript® 2022 Language Specification](https://tc39.es/ecma262/)
* [Official ECMAScript Conformance Test Suite](https://github.com/tc39/test262)
* [The TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html)
