#![allow(non_snake_case)]

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
	CharFnR,
	MulRxVR,
	MulVxRV,
	AskIntV,
	AskIntR,
	ShiftL,
	ShiftR,
	OpenScope,
	CloseScope,
	ThrowCodeR,
	ThrowCodeV,
	PrintVChar,
	PrintRChar,
	SumAllR,
	SumAllV, RandV, RandR,
};

// Actually not needed debug.
#[derive(Debug)]
enum Subject {
	Val,
	Ref
}

/// Compiles a `TokenTree` to return a `ResBuf`.
pub fn compile<'a>(TokenTree: TokenTree) -> ResBuf<'a> {
	let mut CurrentSubject: Subject = Subject::Val;
    // let mut Scope: u8;

    // Start the buffer
    let mut Buf: ResBuf = ResBuf::new();
    Buf.write(BoilerplateStart!());

    for /*(i, */token/*)*/ in TokenTree/*.iter().enumerate() */ {
        match token {
            Token::Ref => CurrentSubject = Subject::Ref,
            Token::Val => CurrentSubject = Subject::Val,
            Token::IncVR => match CurrentSubject {
                Subject::Ref => Buf.write(IncR!()),
                Subject::Val => Buf.write(IncV!()),
            },
            Token::DecVR => match CurrentSubject {
                Subject::Ref => Buf.write(DecR!()),
                Subject::Val => Buf.write(DecV!()),
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

			Token::AskInt => match CurrentSubject {
				Subject::Ref => Buf.write(AskIntR!()),
				Subject::Val => Buf.write(AskIntV!())
			}

			Token::MulVxR => match CurrentSubject {
				Subject::Ref => Buf.write(MulRxVR!()),
				Subject::Val => Buf.write(MulVxRV!())
			}

			Token::ShiftL => Buf.write(ShiftL!()),
			Token::ShiftR => Buf.write(ShiftR!()),

            Token::StartLoop => Buf.write(StartLoop!()),
            Token::EndLoop => Buf.write(EndLoop!()),
			
			Token::OpenScope => Buf.write(OpenScope!()),
			Token::CloseScope => Buf.write(CloseScope!()),
			
			Token::StrFn => unimplemented!(),
			Token::PrintVRChar => match CurrentSubject {
				Subject::Ref => Buf.write(PrintRChar!()),
				Subject::Val => Buf.write(PrintVChar!())
			}
			Token::PrintVRInt => unimplemented!(),
			Token::ThrowCodeVR => match CurrentSubject {
				Subject::Ref => Buf.write(ThrowCodeR!()),
				Subject::Val => Buf.write(ThrowCodeV!())
			}
			Token::PrintNStr => unimplemented!(),
			Token::PrintLastInpOrAsk => unimplemented!(),
			Token::AskStr => unimplemented!(),
			Token::RandVR => match CurrentSubject {
				Subject::Ref => Buf.write(RandR!()),
				Subject::Val => Buf.write(RandV!())
			}
			Token::SumAllVR => match CurrentSubject {
				Subject::Ref => Buf.write(SumAllR!()),
				Subject::Val => Buf.write(SumAllV!())
			}
			Token::Comment => continue,
        }
    }

    Buf.write(BoilerplateEnd!());

    return Buf;
}
