// Copyright 2022 Pekka Enberg and contributors
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use tsparser_parser::parser::Parser;
use tsparser_parser::tokenizer::Tokenizer;
use utf8_chars::BufReadCharsExt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tsparser")]
struct Opt {
    /// Input JavaScript file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Tokenize and print out tokens, but don't parse.
    #[structopt(short, long)]
    tokenize_only: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let input = File::open(opt.input)?;
    let mut input = BufReader::new(input);
    let mut tokenizer = Tokenizer::new(input.chars());
    if opt.tokenize_only {
        while let Some(token) = tokenizer.next_token() {
            println!("{:?} => `{}`", token, tokenizer.slice());
        }
        return Ok(());
    }
    let mut parser = Parser::new(tokenizer);
    match parser.parse_script() {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("Parse error: {}", e.reason)
        }
    }
    Ok(())
}
