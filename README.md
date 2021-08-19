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
  - Run: `cargo run` or `cargo run --bin ipma-cli`. Compile in release mode: `cargo run --release`. With environment variable: `API_TOKEN=<token> cargo run`.
  - Run the binary: `./target/debug/ipma-cli`.
  - By convention, `src/main.rs` is the entry point for a binary, just like `index.js` is for JavaScript.
  - Variables (`let api_token = ...`, for example) are immutable by default.
  - [VS Code extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust).
  - `std::env::var("API_TOKEN")`: `std` crate, `std::env` module, and `var` function.
  - Rust doesn't have the concept of `null` or `undefined`. It has data types that can carry information about whether a function succeeded or failed (`Result` type, for example).
  - `Result` type: `Ok` and `Err` (it is an enum with these variants). `std::env::var: Result<String, VarError>`, for example.
  - `dbg!` macro: print macro for debugging.
  - `.expect` (`std::env::var("API_TOKEN").expect("...");`, for example):
    - If `Ok`, `.expect` will unwrap the value and give just the `String`.
    - If `Err`, `.expect` will panic and crash the program, printing the error to the console.
  - Rust is primarily an Expression-based language. There is always a return value from a block. This even applies to `if` (in JavaScript, `if` is a Statement and doesn't return a value, for example).
  - The last value in an Expression is returned from that Expression.
  - Semicolons turn Expressions into Statements. They are operators that take an Expression, evaluate it, and discard the result.
  - Default return value of a function: `()` ("unit").
- [direnv](https://direnv.net/): to load and unload environment variables depending on the current directory.
