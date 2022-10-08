use crate::{Token, TokenTree};
use crate::lint_helper::*;

/// Checks that a loop starts and ends.
pub fn loop_check(tt: &TokenTree) -> bool {
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
		let mut loop_symbol_table: Vec<usize> = Vec::new();
		if loop_count.is_positive() {
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

			throw_err!("This loop isn't closed", loop_symbol_table[0], tt);
		} else {
			for (i, token) in tt.iter().enumerate() {
				match token {
					Token::EndLoop => {
						loop_symbol_table.push(i);
					}
					Token::StartLoop => {
						loop_symbol_table.pop();
					}
					_ => continue,
				}
			}

			throw_err!(
				"This closing bracket is unmatched",
				loop_symbol_table[loop_symbol_table.len() - 1],
				tt
			);
		}
	}
	true
}

pub fn repeated_subjects(tt: &TokenTree) -> bool {

	for i in 1..tt.len() - 1 {
		match tt[i] {
			Token::Ref => {
				if tt[i + 1] == Token::Ref {
					throw_err_sugg!("This '&' is useless", i, tt, "Try removing this '&'");
				} else if tt[i + 1] == Token::Val {
					throw_err_sugg!("This '&' is useless, because it is later changed without any operations.", i , tt, "Try removing this '&'");
				}
			}
			Token::Val => {
				if tt[i + 1] == Token::Val {
					throw_err_sugg!("This '$' is useless", i, tt, "Try removing this '$'");
				} else if tt[i + 1] == Token::Ref {
					throw_err_sugg!("This '$' is useless, because it is later changed without any operations.", i , tt, "Try removing this '$'");
				}
			}
			_ => continue,
		}
	}
	return true;
}

