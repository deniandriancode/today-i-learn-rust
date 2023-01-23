check the `cargo` version

``` sh
cargo --version
```

### Creating a Project with Cargo

``` sh
cargo new hello_cargo
cd hello_cargo
```

open `Cargo.toml` you will see

``` toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

### Building and Running a Cargo Project

compile the program

``` sh
cargo build
```

run the executable

``` sh
./target/debug/hello_cargo
```

or 

``` sh
cargo run
```

to check if program will compile without actually compiling use

``` sh
cargo check
```

### Building for Release

to build for release add `--release` flag when compiling

``` sh
cargo build --release
```

