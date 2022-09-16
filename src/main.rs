//////////////////////////
//! Braincum Compiler
//! Braincum designed & originally implemented by qexat (Clarisse) @ github.com/qexat
//! Braincum compiler made by blyxyas (Alex) @ github.com/blyxyas
//!
//! As of this day, the project is very recent, the docs are just out. So I'm going to just improvise their following until the creator updates their project to add more information (some things are a little bit vague.)

use clap::Parser;
use std::fs;

mod compiler;
mod definitions;

#[derive(Parser, Debug)]
struct Args {
    inputfile: String,
    #[clap(short)]
    outputfile: String,
}

#[derive(PartialEq)]
pub enum Token {
    Ref,
    Val,
    StartLoop,
    EndLoop,
    OpenScope,
    CloseScope,
    IncVR,
    DecVR,
    ResVR,
    OppVR,
    Deref,
    CharFn,
    StrFn,
    ShiftR,
    ShiftL,
    PrintVRChar,
    AskInt,
    ThrowCode,
    PrintVRInt,
    PrintNStr,
    PrintLastInpOrAsk,
    AskStr,
    Rand,
    SumAll,
    MulVxR,
    Comment,
}

fn main() {
    let args = Args::parse();
    // Read file
    let data = fs::read_to_string(&args.inputfile)
        .expect(&format!("Couldn't read file {}", &args.inputfile));
    let parsed: Vec<Token> = parse(data);
    let compiled = compiler::compile(parsed);
	let mut array: [u8; 256] = [0; 256];
}

fn parse<'a>(data: String) -> Vec<Token> {
    use Token::*;
    let mut TokenTree: Vec<Token> = Vec::new();
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
            '@' => Deref,
            '\'' =>CharFn,
            '"' => StrFn,
            '>' => ShiftR,
            '<' => ShiftL,
            '.' => PrintVRChar,
            ',' => AskInt,
            '!' => ThrowCode,
            '#' => PrintVRInt,
            ':' => PrintNStr,
            ';' => PrintLastInpOrAsk,
            '?' => AskStr,
            'r' => Rand,
            's' => SumAll,
            'm' => MulVxR,
            _ => Comment,
        })
    }
    return TokenTree;
}
