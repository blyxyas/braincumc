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
        "while (i < cell[ref]) {"
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
        "cell[ref] = 255 - cell[ref];"
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
			cell[rerf] = cell[ref] % 255
		};"
    };
}

#[macro_export]
macro_rules! MulRxVR {
	() => {
		"ref = (ref * cell[ref]) % 255;"	
	};
}

#[macro_export]
macro_rules! MulVxRV {
	() => {
		"cell[ref] = (ref * cell[ref]) % 255;"
	};
}


