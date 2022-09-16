macro_rules! BoilerplateStart {
	() => {
		"fn main() {
			let mut pointer: u8 = 0;
			let mut array: [u8; 256] = [0; 256];
		}"
	};
}

macro_rules! BoilerplateEnd {
	() => {
		"}"
	};
}

macro_rules! IncV {
	($pointer: expr) => {
		format!("{} += 1", $pointer);
	};
}

macro_rules! IncR {
	($pointer: expr) => {
		format!("pointer += 1", $pointer)
	};
}