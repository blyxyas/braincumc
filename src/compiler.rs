use crate::{buffer::BufferTrait, BoilerplateEnd, BoilerplateStart, IncR, IncV};

use super::{ResBuf, Token};
use rand::{distributions::Alphanumeric, Rng};

fn rand_string(seed: u8) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
}

pub fn compile<'a>(TokenTree: Vec<Token>) -> ResBuf<'a> {
    let mut CurrentSubject: &Token = &Token::Val;
    let mut Scope: u8;

    // Start the buffer
    let mut Buf: ResBuf = ResBuf::new();

    Buf.write(BoilerplateStart!());

    for (i, token) in TokenTree.iter().enumerate() {
        match token {
            Token::Ref => {
                CurrentSubject = &Token::Ref;
            }
            Token::Val => {
                CurrentSubject = &Token::Val;
            }

            Token::IncVR => match CurrentSubject {
                Token::Ref => {
                    Buf.write(IncR!());
                }
                Token::Val => {
                    Buf.write(IncV!());
                }
				_ => {
					panic!("You cannot increment a {}", CurrentSubject)
				}
            },
        }
    }

    Buf.write(BoilerplateEnd!());

    return Buf;
}
