//////////////////////////
//! Braincum Compiler
//! Braincum designed & originally implemented by qexat (Clarisse) @ github.com/qexat
//! Braincum compiler made by blyxyas (Alex) @ github.com/blyxyas
//!

#![allow(non_snake_case)]

use std::fs;

use colored::*;
use clap::Parser;

mod buffer;
mod compiler;
mod conversion;
use conversion::create_and_convert;

#[macro_use]
mod extra;
use extra::syntax::*;

#[macro_use]
mod definitions;

#[derive(Parser, Debug)]
struct Args {
    inputfile: String,
    #[clap(short)]
    outputfile: String,
    #[clap(short, long)]
    release: bool,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Ref,
    Val,
    StartLoop,
    EndLoop,
    IncVR,
    DecVR,
    ResVR,
    OppVR,
    ConvertVR,
    CharFnVR,
    StrFnVR,
    ShiftR,
    ShiftL,
    PrintVRChar,
    AskInt,
    ThrowCodeVR,
    PrintVRInt,
    PrintNStr,
    PrintLastInpOrAsk,
    AskStr,
    RandVR,
    SumAllVR,
    MulVxR,
}

type ResBuf<'a> = Vec<&'a str>;
type TokenTree = Vec<Token>;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    // Read file
    let data = fs::read_to_string(&args.inputfile)
        .expect(&format!("Couldn't read file {}", &args.inputfile));
    let parsed: TokenTree = parse(data);

	// Check for syntax errors:
	let mut terminate: bool = false;
	test_lints! {
		parsed, terminate,
		loop_check
		repeated_subjects
	}

	if terminate {
		println!("{}", "There were errors in the compilation process. Fix them and try again.".yellow());
		std::process::exit(1);
	}

    let compiled: ResBuf = compiler::compile(parsed);
    create_and_convert(compiled, &args.outputfile, args.release)?;

    Ok(())
}

fn parse<'a>(data: String) -> TokenTree {
    use Token::*;
    let mut inComment: bool = false;
    let mut TokenTree: TokenTree = Vec::new();
    for (i, ch) in data.chars().enumerate() {
		if ch == '(' { inComment = true; continue; }
		else if ch == ')' { inComment = false; continue; }

		if inComment || ch == ' ' || ch == '\n' || ch == '\t' {
			continue;
		}


        TokenTree.push(match ch {
            '&' => Ref,
            '$' => Val,
            '[' => StartLoop,
            ']' => EndLoop,
            '+' => IncVR,
            '-' => DecVR,
            '^' => ResVR,
            '~' => OppVR,
            '@' => ConvertVR,
            '\'' => CharFnVR,
            '"' => StrFnVR,
            '>' => ShiftR,
            '<' => ShiftL,
            '.' => PrintVRChar,
            ',' => AskInt,
            '!' => ThrowCodeVR,
            '#' => PrintVRInt,
            ':' => PrintNStr,
            ';' => PrintLastInpOrAsk,
            '?' => AskStr,
            'r' => RandVR,
            's' => SumAllVR,
            'm' => MulVxR,
            _ => panic!("Unknown character: {} @ character no. {}", ch, i),
        })
    }
    return TokenTree;
}