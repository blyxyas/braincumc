// use rand::{distributions::Alphanumeric, Rng};

#![allow(non_snake_case)]

use core::panic;

use super::{buffer::BufferTrait, ResBuf, Token, TokenTree};
use crate::{
    BoilerplateEnd,
    BoilerplateStart,
    DecR,
    DecV,
    EndLoop,
    IncR,
    IncV,
    OppR,
    OppV,
    ResR,
    ResV,
    StartLoop, 
	ConvertV, 
	ConvertR, 
	CharFnV,
	CharFnR, MulRxVR, MulVxRV
};

// fn rand_string() -> String {
//     return rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(7)
//         .map(char::from)
//         .collect();
// }

// Actually not needed debug.
#[derive(Debug)]
enum Subject {
	Val,
	Ref
}

/// Compiles a `TokenTree` to return a `ResBuf`.
pub fn compile<'a>(TokenTree: TokenTree) -> ResBuf<'a> {
    use Subject::{Ref, Val};
	let mut CurrentSubject: Subject = Subject::Val;
    // let mut Scope: u8;

    // Start the buffer
    let mut Buf: ResBuf = ResBuf::new();

    Buf.write(BoilerplateStart!());

    for /*(i, */token/*)*/ in TokenTree/*.iter().enumerate() */ {
        match token {
            Token::Ref => {
                CurrentSubject = Subject::Ref;
            }
            Token::Val => {
                CurrentSubject = Subject::Val;
            }

            Token::IncVR => match CurrentSubject {
                Subject::Ref => {
                    Buf.write(IncR!());
                }
                Subject::Val => {
                    Buf.write(IncV!());
                }
            },
            Token::DecVR => match CurrentSubject {
                Subject::Ref => {
                    Buf.write(DecR!());
                }
                Subject::Val => {
                    Buf.write(DecV!());
                }
            },
            Token::ResVR => match CurrentSubject {
                Subject::Ref => Buf.write(ResR!()),
                Subject::Val => Buf.write(ResV!()),
            },

            Token::OppVR => match CurrentSubject {
                Subject::Ref => Buf.write(OppR!()),
                Subject::Val => Buf.write(OppV!()),
            },

			Token::ConvertVR => match CurrentSubject {
				Subject::Ref => Buf.write(ConvertR!()),
				Subject::Val => Buf.write(ConvertV!()),
			}
			
			Token::CharFnVR => match CurrentSubject {
				Subject::Ref => Buf.write(CharFnR!()),
				Subject::Val => Buf.write(CharFnV!())
			}

			Token::MulVxR => match CurrentSubject {
				Subject::Ref => Buf.write(MulRxVR!()),
				Subject::Val => Buf.write(MulVxRV!())
			}

            Token::StartLoop => Buf.write(StartLoop!()),
            Token::EndLoop => Buf.write(EndLoop!()),

            _ => unimplemented!(),
        }
    }

    Buf.write(BoilerplateEnd!());

    return Buf;
}
