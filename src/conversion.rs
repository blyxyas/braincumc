//! Defines the functions necessary to convert from a ResBuf to a full executable. All functions are first made for Linux use (Easier in Linux)

use std::process::{
    Command,
};

use tempfile::tempdir;

use super::ResBuf;

pub fn create_and_convert(resbuf: ResBuf, output_file: &str) -> std::io::Result<()> {
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

    // Create cargo project;

    // Compile new Rust tempfile

    Command::new("cargo")
        .args(["build --release"])
        .current_dir(dir_path)
        .status()
        .expect("Couldn't build project");

    Command::new("mv")
        .args([
            format!(
                "{}",
                dir_path.join("target/release/builder").to_string_lossy()
            ),
            format!("{}", output_file),
        ])
        .status()
        .expect("Couldn't move built file");

    Ok(())
}
