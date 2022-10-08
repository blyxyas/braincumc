//////////////////////////
//! Braincum Compiler
//! Braincum designed & originally implemented by qexat (Clarisse) @ github.com/qexat
//! Braincum compiler made by blyxyas (Alex) @ github.com/blyxyas
//!

#![allow(non_snake_case)]

use std::{fs, io::empty};

use clap::Parser;
use colored::*;

mod buffer;
mod compiler;
mod conversion;
use conversion::create_and_convert;

#[macro_use]
mod lint_helper;
use lint_helper::*;
mod lints;
use lints::*;

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

pub static mut TERMINATE: bool = false;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    // Read file
    let data = fs::read_to_string(&args.inputfile)
        .expect(&format!("Couldn't read file {}", &args.inputfile));

    let parsed: TokenTree = parse(data);

    // Check for syntax errors:

    unsafe {
        test_tt_lints! {
            parsed;

            loop_check
            repeated_subjects
			empty_loop
        }
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
        if ch == '(' {
            inComment = true;
            continue;
        } else if ch == ')' {
            inComment = false;
            continue;
        }

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
            _ => {
                throw_err_src!(format!("Unknown Character: {}", ch), i, &data);
                unsafe {
                    TERMINATE = true;
                }
                continue;
            }
        })
    }
    return TokenTree;
}

#[cfg(test)]
mod tests {
	use std::fs::{File, self};
	use difference::{self, Changeset, Difference};
	use term;

	fn get_diff(text1: &str, text2: &str) -> bool {
		let mut t = term::stdout().unwrap();

    let Changeset { diffs, .. } = Changeset::new(text1, text2, "");

	if diffs.is_empty() {
		return true;
	}

    for c in &diffs {
        match *c {
            Difference::Same(ref z) => {
                t.fg(term::color::RED).unwrap();
                write!(t, "{}", z);
            }
            Difference::Rem(ref z) => {
                t.fg(term::color::WHITE).unwrap();
                t.bg(term::color::RED).unwrap();
                write!(t, "{}", z);
                t.reset().unwrap();
            }
            _ => (),
        }
    }
    t.reset().unwrap();

    writeln!(t, "");

    for c in &diffs {
        match *c {
            Difference::Same(ref z) => {
                t.fg(term::color::GREEN).unwrap();
                write!(t, "{}", z);
            }
            Difference::Add(ref z) => {
                t.fg(term::color::WHITE).unwrap();
                t.bg(term::color::GREEN).unwrap();
                write!(t, "{}", z);
                t.reset().unwrap();
            }
            _ => (),
        }
    }
    t.reset().unwrap();
    t.flush().unwrap();
	false
	}

	#[test]
	pub fn test_all() {
		let mut exstderr = fs::read_to_string("../expected-stderr.txt").expect("Couldn't read file expected-stderr.txt");
		
		let parsed: TokenTree = parse(data);

		if get_diff(text1, text2) {

		}
	}
}