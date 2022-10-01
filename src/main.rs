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
mod conversion;
use conversion::create_and_convert;

#[macro_use]
mod definitions;

#[derive(Parser, Debug)]
struct Args {
    inputfile: String,
    #[clap(short)]
    outputfile: String,
	#[clap(short, long)]
	release: bool
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

fn main() -> std::io::Result<()>{
    let args = Args::parse();
    // Read file
    let data = fs::read_to_string(&args.inputfile)
        .expect(&format!("Couldn't read file {}", &args.inputfile));
    let parsed: TokenTree = parse(data);
    let compiled: ResBuf = compiler::compile(parsed);
	create_and_convert(compiled, &args.outputfile, args.release)?;
	
	Ok(())
}

fn parse<'a>(data: String) -> TokenTree {
    use Token::*;
	let mut inComment: bool = false;
    let mut TokenTree: TokenTree = Vec::new();
    for (i, ch) in data.chars().enumerate() {
        if ch == ')' {
			inComment = false;
			continue;
		}

		else if inComment {
			continue;
		}

		else if ch == ' ' {
			continue
		}

		else if ch == '(' {
			inComment = true;
			continue
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
