use crate::lint_helper::*;
use crate::{
    Token,
    TokenTree,
};

/// Checks that a loop starts and ends. (All brackets are matched)
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
        } else {
            for (i, token) in tt.iter().enumerate() {
                match token {
                    Token::EndLoop => {
                        if !loop_symbol_table.is_empty() {
                            throw_err!("This closing bracket is unmatched", i, tt);
                        } else {
                            loop_symbol_table.push(i);
                        }
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
                    throw_err_sugg!(@warn "This '&' is useless", i, tt, "Try removing this '&'");
                } else if tt[i + 1] == Token::Val {
                    throw_err_sugg!(@warn "This '&' is useless, because it is later changed without any operations.", i , tt, "Try removing this '&'");
                }
            }
            Token::Val => {
                if tt[i + 1] == Token::Val {
                    throw_err_sugg!(@warn "This '$' is useless", i, tt, "Try removing this '$'");
                } else if tt[i + 1] == Token::Ref {
                    throw_err_sugg!(@warn "This '$' is useless, because it is later changed without any operations.", i , tt, "Try removing this '$'");
                }
            }
            _ => continue,
        }
    }
    return true;
}

pub fn empty_loop(tt: &TokenTree) -> bool {
    for i in 0..tt.len() - 1 {
        match tt[i] {
            Token::StartLoop => {
                if tt[i + 1] == Token::EndLoop {
                    throw_err_sugg!(@warn "This loop is empty, and therefore useless", i, tt, "Try removing this whole loop.");
                };
            }
            _ => continue,
        }
    }
    return true;
}
