//! Defines the functions necessary to convert from a ResBuf to a full executable. All functions are first made for Linux use (Easier in Linux)

use std::process::{
    Command,
};

use tempfile::tempdir;

use super::ResBuf;
use super::CargoBoilerplate;
use super::CargoBoilerplateSmall;

pub fn create_and_convert(resbuf: ResBuf, output_file: &str, smallerbin: bool) -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();

    Command::new("cargo")
        .args(["init", "--name", "builder"])
        .current_dir(dir_path)
        .status()
        .expect("Couldn't create cargo project");

    // Write to file:
    std::fs::write(dir_path.join("src/main.rs"), resbuf.join("\n").as_bytes()).expect(&format!(
        "Couldn't write to {}",
        dir_path.join("src/main.rs").to_string_lossy()
    ));

    // let file_path = &dir
    //     .path()
    //     .join("src/out.rs")
    //     .as_os_str()
    //     .to_str()
    //     .unwrap()
    //     .to_string();

	// Write to Cargo.toml

	if smallerbin {
		std::fs::write(dir_path.join("Cargo.toml"), CargoBoilerplateSmall!().as_bytes()).expect("Couldn't write to Cargo.toml");
	} else {
		std::fs::write(dir_path.join("Cargo.toml"), CargoBoilerplate!().as_bytes()).expect("Couldn't write to Cargo.toml");
	}

    // Compile new Rust tempfile

    Command::new("cargo")
        .args(["build"])
        .current_dir(dir_path)
        .status()
        .expect("Couldn't build project");

    Command::new("mv")
        .args([
            format!(
                "{}",
                dir_path.join("target/debug/braincumc").to_string_lossy()
            ),
            output_file.to_string(),
        ])
        .status()
        .expect("Couldn't move built file");

    Ok(())
}
