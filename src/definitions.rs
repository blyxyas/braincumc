#[macro_export]
macro_rules! BoilerplateStart {
    () => {
        "fn main() {
			let mut ref: u8 = 0;
			let mut cell: [u8; 256] = [0; 256];
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
        "cell[ref] += 1"
    };
}

#[macro_export]
macro_rules! IncR {
    () => {
        "ref += 1"
    };
}