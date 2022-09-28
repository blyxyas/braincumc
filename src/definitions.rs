//! Defines all necessary macros

#[macro_export]
macro_rules! CargoBoilerplateSmall {
	() => {
		"
		[package]
		name = \"braincumc\"
		version = \"0.1.0\"
		edition = \"2021\"
		
		[profile.release]
		opt-lever = \"z\"
		strip = true
		lto = true
		codegen-units = 1
		
		[dependencies]
		rand = \"0.8.5\"
		"
	};
}

#[macro_export]
macro_rules! BoilerplateStart {
    () => {
        "use std::io::stdin;
use std::collections::VecDeque;
use rand::Rng;
	
fn main() {
let mut ptr: usize = 0;
let mut scope: usize = 0;
let mut cell: [u8; 3000] = [0; 3000];
let mut input_string: String = String::new();
"
    };
}

#[macro_export]
macro_rules! BoilerplateEnd {
    () => {
        "}"
    };
}
#[macro_export]
macro_rules! IncV {
    () => {
        "cell[ptr] += 1;"
    };
}

#[macro_export]
macro_rules! IncR {
    () => {
        "ptr += 1;"
    };
}

#[macro_export]
macro_rules! DecV {
    () => {
        "cell[ptr] -= 1;"
    };
}

#[macro_export]
macro_rules! DecR {
    () => {
        "ptr -= 1;"
    };
}

#[macro_export]
macro_rules! StartLoop {
    () => {
        "while (cell[ptr] > 0) {"
    };
}

#[macro_export]
macro_rules! EndLoop {
    () => {
        "}"
    };
}

#[macro_export]
/// Sets an address (sets to 0)
macro_rules! ResR {
    () => {
        "ptr = 0;"
    };
}

#[macro_export]
/// Sets a value to 0
macro_rules! ResV {
    () => {
        "cell[ptr] = 0;"
    };
}

#[macro_export]
/// Sets an address to the opposite value.
macro_rules! OppR {
    () => {
        "ptr = 255 - ptr;"
    };
}

#[macro_export]
macro_rules! OppV {
    () => {
        "cell[ptr] = 255 -  cell[ptr];"
    };
}

#[macro_export]
macro_rules! ConvertR {
    () => {
        "ptr = cell[ptr];"
    };
}

#[macro_export]
macro_rules! ConvertV {
    () => {
        "cell[ptr]= ptr;"
    };
}

#[macro_export]
macro_rules! CharFnR {
    () => {
        "if ptr < 32 {
			ptr += 32;
		} else if ptr >= 256 {
			ptr = ptr % 255;
		};"
    };
}

#[macro_export]
macro_rules! CharFnV {
    () => {
        "if cell[ptr] < 32 {
			cell[ptr] += 32;
		} else if cell[ptr] >= 256 {
			cell[ptr] = cell[ptr] % 255
		};"
    };
}

#[macro_export]
macro_rules! MulRxVR {
    () => {
        "ptr = (ptr * cell[ptr]); % 255;"
    };
}

#[macro_export]
macro_rules! MulVxRV {
    () => {
        "cell[ptr] = (ptr * cell[ptr]) % 255;"
    };
}

// use rand::{distributions::Alphanumeric, Rng};

// pub fn rand_string() -> String {
//     return rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(7)
//         .map(char::from)
//         .collect();
// }

#[macro_export]
macro_rules! AskIntR {
	() => {
		"stdin().read_line(&mut input_string).ok.expect(\"Failed to read line\");
		
		ptr = input_string.parse::<u8>().expect(\"Couldn't parse your input to a number between 0 and 255\");"
	};
}

#[macro_export]
macro_rules! AskIntV {
	() => {
		// rand_string = rand_string();
		"stdin().read_line(&mut input_string).ok().expect(\"Failed to read line\");
		
		cell[ptr] = input_string.parse::<usize>().expect(\"Couldn't parse your input to a number between 0 and 256\");"
	};
}

#[macro_export]
macro_rules! ShiftL {
    () => {
        "cell[..].rotate_left(1);
		cell[-1] = 0;"
    };
}

#[macro_export]
macro_rules! ShiftR {
    () => {
        "cell[..].rotate_right(1);
		cell[0] = 0;"
    };
}

#[macro_export]
macro_rules! OpenScope {
    () => {
        "scope += 1 * (scope == 0);"
    };
}

#[macro_export]
macro_rules! CloseScope {
    () => {
        "scope -= 1 * (scope == 1);"
    };
}

#[macro_export]
macro_rules! ThrowCodeV {
    () => {
        "std::process::exit(cell[ptr]);"
    };
}

#[macro_export]
macro_rules! ThrowCodeR {
    () => {
        "std::process::exit(ptr);"
    };
}

#[macro_export]
macro_rules! PrintVChar {
    () => {
        "println!(\"{}\", cell[ptr] as char);"
    };
}

#[macro_export]
macro_rules! PrintRChar {
    () => {
        "println!(\"{}\", ptr as char);"
    };
}

#[macro_export]
macro_rules! SumAllV {
    () => {
        "for i in 0..ptr {
			cell[ptr] += cell[i];
		}"
    };
}

#[macro_export]
macro_rules! SumAllR {
    () => {
        "
		let mut sum = 0;
		for i in 0..ptr {
			sum += cell[i];
		}
		ptr = sum;"
    };
}

#[macro_export]
macro_rules! RandR {
    () => {
        "ptr = rand::thread_rng().gen_range(0..=255);"
    };
}

#[macro_export]
macro_rules! RandV {
    () => {
        "cell[ptr] = rand::thread_rng().gen_range(0..=255);"
    };
}

#[macro_export]
macro_rules! AskStr {
	() => {
		"stdin().read_line(&mut input_string).ok.expect(\"Failed to read line\");
		
		for i in input_string.chars().count() {
			cell[ptr + i] = input_string.chars().nth(i)
		}"
	};
}

#[macro_export]
macro_rules! PrintLastInpOrAsk {
    () => {
        "if input_string.is_empty() {
			// Take input
			stdin().read_line(&mut input_string).ok.expect(\"Failed to read line\");
		}
		println!(\"{}\", input_string())"
    };
}

#[macro_export]
macro_rules! PrintNStr {
    () => {
        "for i in ptr..cell[ptr] {
			print!(\"{}\", cell[i] as char);
		}"
    };
}

#[macro_export]
macro_rules! PrintVInt {
    () => {
        "println!(\"{}\", cell[ptr]);"
    };
}

#[macro_export]
macro_rules! PrintRInt {
    () => {
        "println!(\"{}\", ptr);"
    };
}

// TODO: Waiting for response on #13

// #[macro_export]
// macro_rules! StrFnR {
//     () => {
//         "cell[ptr]"
//     };
// }

// #[macro_export]
// macro_rules! StrFnV {
//     () => {
//         ""
//     };
// }
