use super::Token;
use rand::{distributions::Alphanumeric, Rng};

use super::definitions;

fn rand_string(seed: u8) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
}

pub fn compile<'a>(TokenTree: Vec<Token>) {
    let mut CurrentSubject: &Token;
	let mut Scope: u8;
    for (i, token) in TokenTree.iter().enumerate() {
        match token {
            Ref => {
				CurrentSubject = Ref
				
			},
            Val => {
				CurrentSubject = Val
				// Write to buffer
			},
        }
    }
}
