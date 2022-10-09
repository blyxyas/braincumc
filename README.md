# Braincum Compiler

The compiler for the Braincum esolang, interpreter (as well as documentation) is [here](https://github.com/qexat/braincum)

Keep in mind that I'm not the original creator of the language, and this compiler isn't maintained 100% in sync with the original interpreter (some things may change.)

## Installation

You can either download it in the *Releases* section or build it from source

### Downloading the compiler

Head up to [*Releases*](https://github.com/blyxyas/braincumc/releases)

### Building from source

Requirements:

* Cargo
* Rust
* Git

Right now you can install Braincum Compiler by cloning the repo:

```bash
git clone https://github.com/blyxyas/braincumc
```

And building the project with Cargo:
```bash
cargo build --release
```

## Usage

First, you need to write the Braincum source to any `.bc`

```js
// source.bc
++#
```

Now, run the executable, either using `target/release/braincumc` or `cargo run`

```bash
target/release/braincumc source.bc -o out --release
```

Use the `--release` flag at the end of the command to indicate that the final output will be totally optimized (even in size!), else it will be a debug build, and it will not be as fast (though it will build faster.)

Now you can execute the new executable!

```
./out
```

(This will output `2`)

Keep in mind that this process will create a new directory in the current directory (called `target_bc`) to host the dependencies and cache the build. If you decide to delete this directory braincumc will need to create it again for the next build.

## License

The license is kept in the LICENSE file, this software uses the MIT License.