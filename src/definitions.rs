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
macro_rules! CargoBoilerplate {
	() => {
		"
		[package]
		name = \"braincumc\"
		version = \"0.1.0\"
		edition = \"2021\"

		[profile.release]
		lto = \"fat\"
		codegen-units = 1
		
		"
	};
}

#[macro_export]
macro_rules! BoilerplateStart {
    () => {
        "use std::io::stdin;
		use std::collections::VecDeque
		use rand::Rng
	
		fn main() {
			let mut ref: u8 = 0;
			let mut cell: [u8; 30000] = [0; 30000]
			let mut input_string: String = String::new();
		}"
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
        "cell[ref] += 1;"
    };
}

#[macro_export]
macro_rules! IncR {
    () => {
        "ref += 1;"
    };
}

#[macro_export]
macro_rules! DecV {
    () => {
        "cell[ref] -= 1;"
    };
}

#[macro_export]
macro_rules! DecR {
    () => {
        "ref -= 1;"
    };
}

#[macro_export]
macro_rules! StartLoop {
    () => {
        "while (cell[ref] > 0) {"
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
        "ref = 0;"
    };
}

#[macro_export]
/// Sets a value to 0
macro_rules! ResV {
    () => {
        "cell[ref] = 0;"
    };
}

#[macro_export]
/// Sets an address to the opposite value.
macro_rules! OppR {
    () => {
        "ref = 255 - ref;"
    };
}

#[macro_export]
macro_rules! OppV {
    () => {
        "cell[ref] = 255 -  cell[ref];"
    };
}

#[macro_export]
macro_rules! ConvertR {
    () => {
        "ref = cell[ref];"
    };
}

#[macro_export]
macro_rules! ConvertV {
    () => {
        "cell[ref]= ref;"
    };
}

#[macro_export]
macro_rules! CharFnR {
    () => {
        "if ref < 32 {
			ref += 32;
		} else if ref >= 256 {
			ref = ref % 255;
		};"
    };
}

#[macro_export]
macro_rules! CharFnV {
    () => {
        "if cell[ref] < 32 {
			cell[ref] += 32;
		} else if cell[ref] >= 256 {
			cell[ref] = cell[ref] % 255
		};"
    };
}

#[macro_export]
macro_rules! MulRxVR {
    () => {
        "ref = (ref * cell[ref]); % 255;"
    };
}

#[macro_export]
macro_rules! MulVxRV {
    () => {
        "cell[ref] = (ref * cell[ref]) % 255;"
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
		
		ref = input_string.parse::<u8>().expect(\"Couldn't parse your input to a number between 0 and 255\");"
	};
}

#[macro_export]
macro_rules! AskIntV {
	() => {
		// rand_string = rand_string();
		"stdin().read_line(&mut input_string).ok().expect(\"Failed to read line\");
		
		cell[ref] = input_string.parse::<usize>().expect(\"Couldn't parse your input to a number between 0 and 256\");"
	};
}

#[macro_export]
macro_rules! ShiftL {
    () => {
        "cell[..]cell[]"
    };
}

#[macro_export]
macro_rules! ShiftR {
    () => {
        "cell[..].rotate_right(1);
		cell[]"
    };
}

#[macro_export]
macro_rules! OpenScope {
    () => {
        ""
    };
}

#[macro_export]
macro_rules! CloseScope {
    () => {
        ""
    };
}

#[macro_export]
macro_rules! ThrowCodeV {
    () => {
        "std::process::exit(cell[ref]);"
    };
}

#[macro_export]
macro_rules! ThrowCodeR {
    () => {
        "std::process::exit(ref);"
    };
}

#[macro_export]
macro_rules! PrintVChar {
    () => {
        "println!(\"{}\", cell[ref] as char);"
    };
}

#[macro_export]
macro_rules! PrintRChar {
    () => {
        "println!(\"{}\", ref as char);"
    };
}

#[macro_export]
macro_rules! SumAllV {
    () => {
        "for i in 0..ref {
			cell[ref] += cell[i];
		}"
    };
}

#[macro_export]
macro_rules! SumAllR {
    () => {
        "
		let mut sum = 0;
		for i in 0..ref {
			sum += cell[i];
		}
		ref = sum;"
    };
}

#[macro_export]
macro_rules! RandR {
    () => {
        "ref = rand::thread_rng().gen_range(0..256)"
    };
}

#[macro_export]
macro_rules! RandV {
    () => {
        "cell[ref] = rand::thread_rng().gen_range(0..256)"
    };
}

#[macro_export]
macro_rules! AskStr {
	() => {
		"stdin().read_line(&mut input_string).ok.expect(\"Failed to read line\");
		
		for i in input_string.chars().count() {
			cell[ref + i] = input_string.chars().nth(i)
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
		"for i in ref..cell[ref] {
			print!(\"{}\", cell[i] as char);
		}"
	};
}

#[macro_export]
macro_rules! PrintVInt {
	() => {
		"println!(\"{}\", cell[ref]);"
	};
}

#[macro_export]
macro_rules! PrintRInt {
	() => {
		"println!(\"{}\", ref);"
	};
}

// TODO: Waiting for response on #13

// #[macro_export]
// macro_rules! StrFnR {
//     () => {
//         "cell[ref]"
//     };
// }

// #[macro_export]
// macro_rules! StrFnV {
//     () => {
//         ""
//     };
// }
