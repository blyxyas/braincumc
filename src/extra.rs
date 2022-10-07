pub mod syntax {
    use colored::*;

    use crate::Token;
    use crate::TokenTree;

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

    fn throw_err_with_char(msg: &str, charno: usize, tt: &TokenTree) {
        // Convert TT to characters:
        let converted_tt = tt_to_char(tt);

        // Make sure that only characters that exist in the input are shown.

        let span: String;
        if charno < 8 {
            if charno + 8 > tt.len() {
                span = converted_tt[..].iter().collect();
            } else {
                span = converted_tt[..=charno + 8].iter().collect();
            }
        } else {
            if charno + 8 > tt.len() {
                span = converted_tt[charno - 8..].iter().collect();
            } else {
				span = converted_tt[charno - 8..=charno + 8].iter().collect();
			}
        }

        // This will look something like: '$[&+]#&+$-['
        // '#' being the character to highlight ^

        println!(
            "[{} @ Char. no {}] {}\n\n{}",
            "ERROR".red(),
            charno.to_string().blue(),
            msg.yellow(),
            span,
        );
    }

    fn throw_err_no_char(msg: &str) {
        println!("[{}] {}", "ERROR".red(), msg.yellow());
    }
    /// Checks that a loop starts and ends.
    pub fn loop_check(tt: &TokenTree) {
        let mut loop_count: i8 = 0;
        for token in tt {
            match token {
                Token::StartLoop => loop_count += 1,
                Token::EndLoop => loop_count -= 1,
                _ => continue,
            }
        }

        if loop_count != 0 {
            // Where's the bug?
            if loop_count.is_positive() {
                let mut loop_symbol_table: Vec<usize> = Vec::new();
                for (i, token) in tt.iter().enumerate() {
                    match token {
                        Token::StartLoop => {
                            loop_symbol_table.push(i);
                        }
                        Token::EndLoop => {
                            loop_symbol_table.pop();
                        }
                        _ => continue,
                    }
                }

                throw_err_with_char("This loop isn't closed", loop_symbol_table[0], tt);
            }
        }
    }
}
