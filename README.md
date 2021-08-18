# ipma-cli

A Rust CLI to get weather forecasts in Portugal from IPMA.

## References

- Chris Biscardi's [Rust Adventure](https://www.rustadventure.dev/) email course.

## Notes

- [IPMA API](http://api.ipma.pt/):
  - _Previsão Meteorológica Diária até 5 dias agregada por Local_.
  - _Lista de identificadores para as capitais distrito e ilhas_.
- [Rust](https://www.rust-lang.org/):
  - [rustup](https://rustup.rs/): version manager.
  - Crate: package.
  - `Cargo.toml` file: equivalent to the `package.json` file.
  - Cargo: package manager. It works like npm, but doesn't bring commands to manage (add, for example) crates/dependencies in the `Cargo.toml` file. Install the [cargo-edit](https://crates.io/crates/cargo-edit) crate (`cargo install cargo-edit`) to do this.
  - Init: `cargo new ipma-cli`.
  - Run: `cargo run` or `cargo run --bin ipma-cli`. Compile in release mode: `cargo run --release`.
  - Run the binary: `./target/debug/ipma-cli`.
  - By convention, `src/main.rs` is the entry point for a binary, just like `index.js` is for JavaScript.
