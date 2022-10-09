//! Defines the functions necessary to convert from a ResBuf to a full executable. All functions are first made for Linux use (Easier in Linux)

use std::fs;
use std::path::Path;
use std::process::Command;

use super::CargoBoilerplateSmall;
use super::ResBuf;

pub fn create_and_convert(resbuf: ResBuf, output_file: &str, release: bool) -> std::io::Result<()> {
    if !Path::new("target_bc").is_dir() {
        fs::create_dir("target_bc").unwrap();
        Command::new("cargo")
            .args(["init", "--name", "builder"])
            .current_dir("target_bc")
            .status()
            .expect("Couldn't create cargo project");

        // let file_path = &dir
        //     .path()
        //     .join("src/out.rs")
        //     .as_os_str()
        //     .to_str()
        //     .unwrap()
        //     .to_string();

        // Write to Cargo.toml

        std::fs::write("target_bc/Cargo.toml", CargoBoilerplateSmall!().as_bytes())
            .expect("Couldn't write to Cargo.toml");
    }

    // Write ResBuf to main.rs

    std::fs::write("target_bc/src/main.rs", resbuf.join("\n").as_bytes())
        .expect("Couldn't write to target_bc/src/main.rs");

	println!("Compiling the source, this could take a few seconds...");

    // build and move
    if release {
        Command::new("cargo")
            .args(["build", "--quiet", "--release"])
            .current_dir("target_bc")
            .status()
            .expect("Couldn't build project");
		Command::new("mv")
        .args(["target_bc/target/release/braincumc", output_file])
        .status()
        .expect("Couldn't move built file");
    } else {
        Command::new("cargo")
            .args(["build", "--quiet"])
            .current_dir("target_bc")
            .status()
            .expect("Couldn't build project");
		Command::new("mv")
		.args(["target_bc/target/debug/braincumc", output_file])
		.status()
		.expect("Couldn't move built file");
		}

    Ok(())
}
