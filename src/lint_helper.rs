
use colored::*;

use crate::Token;
use crate::TokenTree;

#[macro_export]
macro_rules! test_lints {
		($tt: ident, $term: expr,
			$($lint: ident)
		*) => {
			$(
				if !$lint(&$tt) {
					$term = true;
				}
			)
			*
		};
	}

/// Converts a token tree into it's original character tree.
fn tt_to_char<'a>(tt: &TokenTree) -> Vec<char> {
    let mut charvec: Vec<char> = Vec::new();
    for token in tt {
        charvec.push(match token {
            Token::Ref => '&',
            Token::Val => '$',
            Token::StartLoop => '[',
            Token::EndLoop => ']',
            Token::IncVR => '+',
            Token::DecVR => '-',
            Token::ResVR => '^',
            Token::OppVR => '~',
            Token::ConvertVR => '@',
            Token::CharFnVR => '\'',
            Token::StrFnVR => '"',
            Token::ShiftR => '>',
            Token::ShiftL => '<',
            Token::PrintVRChar => '.',
            Token::AskInt => ',',
            Token::ThrowCodeVR => '!',
            Token::PrintVRInt => '#',
            Token::PrintNStr => ':',
            Token::PrintLastInpOrAsk => ';',
            Token::AskStr => '?',
            Token::RandVR => 'r',
            Token::SumAllVR => 's',
            Token::MulVxR => 'm',
        })
    }
    charvec
}

fn __throw_err(msg: &str, charno: usize, tt: &TokenTree) {
    // Convert TT to characters:
    let converted_tt = tt_to_char(tt);

    // Make sure that only characters that exist in the input are shown.

    let span: String;
    let highlight: usize;
    if charno < 8 {
        if charno + 8 > tt.len() {
            span = converted_tt[..].iter().collect();
            highlight = charno;
        } else {
            span = converted_tt[..charno + 8].iter().collect();
            highlight = charno;
        }
    } else {
        if charno + 8 > tt.len() {
            span = converted_tt[charno - 8..].iter().collect();
            highlight = 8;
        } else {
            span = converted_tt[charno - 8..charno + 8].iter().collect();
            highlight = 8;
        }
    }

    let mut arrow: String = String::new();
    for _ in 0..highlight {
        arrow.push(' ');
    }
    arrow.push('^');

    // This will look something like: '$[&+]#&+$-['
    // '#' being the character to highlight ^

    println!(
        "{}\n[{} @ Char. no {}] {}\n\n{}\n{}",
        "-------------------------------------------------"
            .red()
            .bold(),
        "ERROR".red().bold(),
        charno.to_string().blue(),
        msg.bright_red().bold(),
        span,
        arrow.red().bold()
    );
}

fn __throw_err_sugg(msg: &str, charno: usize, tt: &TokenTree, sugg: &str) {
    // Convert TT to characters:
    let converted_tt = tt_to_char(tt);

    // Make sure that only characters that exist in the input are shown.

    let span: String;
    let highlight: usize;
    if charno < 8 {
        if charno + 8 > tt.len() {
            span = converted_tt[..].iter().collect();
            highlight = charno;
        } else {
            span = converted_tt[..charno + 8].iter().collect();
            highlight = charno;
        }
    } else {
        if charno + 8 > tt.len() {
            span = converted_tt[charno - 8..].iter().collect();
            highlight = 8;
        } else {
            span = converted_tt[charno - 8..charno + 8].iter().collect();
            highlight = 8;
        }
    }

    let mut arrow: String = String::new();
    for _ in 0..highlight {
        arrow.push(' ');
    }
    arrow.push('^');

    // This will look something like: '$[&+]#&+$-['
    // '#' being the character to highlight ^

    println!(
        "{}\n[{} @ Char. no {}] {}\n\n{}\n{}\n{}{}",
        "-------------------------------------------------"
            .red()
            .bold(),
        "ERROR".red().bold(),
        charno.to_string().blue(),
        msg.bright_red().bold(),
        span,
        arrow.red().bold(),
        "Suggestion: ".green().bold(),
        sugg
    );
}
macro_rules! throw_err {
    ($msg: expr, $charno: expr, $tt: ident) => {
        __throw_err($msg, $charno, $tt);
        return false;
    };
}
macro_rules! throw_err_sugg {
		($msg: expr, $charno: expr, $tt: ident, $($sugg: expr), *) => {
			__throw_err_sugg($msg, $charno, $tt, &format!($($sugg), *));
			return false;
		};
	}
