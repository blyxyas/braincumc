//////////////////////////
//! Braincum Compiler
//! Braincum designed & originally implemented by qexat (Clarisse) @ github.com/qexat
//! Braincum compiler made by blyxyas (Alex) @ github.com/blyxyas
//!
//! As of this day, the project is very recent, the docs are just out. So I'm going to just improvise their following until the creator updates their project to add more information (some things are a little bit vague.)

#![allow(non_snake_case)]

use std::fs;

use clap::Parser;
mod buffer;
mod compiler;

#[macro_use]
mod definitions;

#[derive(Parser, Debug)]
struct Args {
    inputfile: String,
    #[clap(short)]
    outputfile: String,
    #[clap(short)]
    cargo: bool,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Ref, // -
    Val, // -
    StartLoop, // -
    EndLoop, // -
    OpenScope,
    CloseScope,
    IncVR,
    DecVR,
    ResVR,
    OppVR,
    ConvertVR,
    CharFnVR,
    StrFn,
    ShiftR,
    ShiftL,
    PrintVRChar,
    AskInt,
    ThrowCodeVR,
    PrintVRInt,
    PrintNStr,
    PrintLastInpOrAsk,
    AskStr,
    Rand,
    SumAll,
    MulVxR,
    Comment,
}

type ResBuf<'a> = Vec<&'a str>;
type TokenTree = Vec<Token>;

fn main() {
    let args = Args::parse();
    // Read file
    let data = fs::read_to_string(&args.inputfile)
        .expect(&format!("Couldn't read file {}", &args.inputfile));
    let parsed: TokenTree = parse(data);
    let compiled = compiler::compile(parsed);
    println!("{}", compiled.join("\n"));
}

fn parse<'a>(data: String) -> TokenTree {
    use Token::*;
    let mut TokenTree: TokenTree = Vec::new();
    for ch in data.chars() {
        // Check if char is valid, else, it's a comment.
        TokenTree.push(match ch {
            '&' => Ref,
            '$' => Val,
            '[' => StartLoop,
            ']' => EndLoop,
            '{' => OpenScope,
            '}' => CloseScope,
            '+' => IncVR,
            '-' => DecVR,
            '^' => ResVR,
            '~' => OppVR,
            '@' => ConvertVR,
            '\'' => CharFnVR,
            '"' => StrFn,
            '>' => ShiftR,
            '<' => ShiftL,
            '.' => PrintVRChar,
            ',' => AskInt,
            '!' => ThrowCodeVR,
            '#' => PrintVRInt,
            ':' => PrintNStr,
            ';' => PrintLastInpOrAsk,
            '?' => AskStr,
            'r' => Rand,
            's' => SumAll,
            'm' => MulVxR,
            _ => panic!(""),
        })
    }
    return TokenTree;
}
