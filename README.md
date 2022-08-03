# tkinter-rs

## System dependencies

### Build dependencies

#### System libraries

bzip2 and snapper libraries are expected to be installed with bzip2 development package.
#### bindgen
bindgen leverages libclang to preprocess, parse, and type check C and C++ header files.

It is required to use Clang 5.0 or greater.

See requirements at https://rust-lang.github.io/rust-bindgen/requirements.html

### Install required Rust components

```sh
rustup update
rustup component add clippy
rustup component add rustfmt
```

### Install required Cargo plugins

```sh
cargo install cargo-cmd
```
