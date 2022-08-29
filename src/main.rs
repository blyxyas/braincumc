//////////////////////////
//! Braincum Compiler
//! Braincum designed & originally implemented by qexat (Clarisse) @ github.com/qexat
//! Braincum compiler made by blyxyas (Alex) @ github.com/blyxyas

use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
	inputfile: String,
	#[clap(short)]
	outputfile: String,
}

enum Tokens {
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
	Comment
}

fn main() {
	let args = Args::parse();
    // Read file
	let data = fs::read_to_string(&args.inputfile).expect(&format!("Couldn't read file {}", &args.inputfile));
	let parsed: Vec<Tokens> = parse(data);
	let compiled = compile(parsed);
}

fn parse(data: String) -> Vec<Tokens> {
	use Tokens::*;
	let mut TokenTree: Vec<Tokens> = Vec::new();
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
			'\'' => CharFn,
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

			_ => Comment
		})
	}
	return TokenTree;
}

fn compile(TokenTree: Vec<Tokens>) {
	for token in TokenTree {
		// Token subject is maintained until some of the following operators is used:
		// 
	}
}
