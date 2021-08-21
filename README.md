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
  - Run: `cargo run` or `cargo run --bin ipma-cli`.
    - Compile in release mode: `cargo run --release`.
    - With an environment variable: `API_TOKEN=<token> cargo run`.
    - With arguments passed to the binary to run: `cargo run -- lisbon`.
  - Run the binary: `./target/debug/ipma-cli`.
  - By convention, `src/main.rs` is the entry point for a binary, just like `index.js` is for JavaScript.
  - Variables (`let api_token = ...`, for example) are immutable by default.
  - `let mut`: mutable variable.
  - [VS Code extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust).
  - `std::env::var("API_TOKEN")`: `std` crate, `std::env` module, and `var` function.
  - Rust doesn't have the concept of `null` or `undefined`. It has data types that can carry information about whether a function succeeded or failed (`Result` type, for example).
  - `Result` type: `Ok` and `Err` (it is an enum with these variants). `std::env::var: Result<String, VarError>`, for example.
  - `dbg!` macro: print macro for debugging.
    - `println!("{}", dbg!(2) == 2); // true`.
  - `.expect` (`std::env::var("API_TOKEN").expect("...");`, for example):
    - If `Ok`, `.expect` will unwrap the value and give just the `String`.
    - If `Err`, `.expect` will panic and crash the program, printing the error to the console.
  - Rust is primarily an Expression-based language. There is always a return value from a block. This even applies to `if` (in JavaScript, `if` is a Statement and doesn't return a value, for example).
  - The last value in an Expression is returned from that Expression.
  - Semicolons turn Expressions into Statements. They are operators that take an Expression, evaluate it, and discard the result.
  - Default return value of a function: `()` ("unit").
  - A struct is similar to an object in JavaScript.
  - `.collect`: to consume the iterator (to get all values without `.next`).
  - CLI crates:
    - ⭐ [clap](https://crates.io/crates/clap).
    - [structopt](https://crates.io/crates/structopt).
    - [docopt](https://crates.io/crates/docopt).
  - Crates:
    - [reqwest](https://crates.io/crates/reqwest) (for HTTP requests).
      - `cargo add --features "json blocking" reqwest@0.11.3`.
    - `serde` + `serde_json` (to serialize/deserialize Rust data structures).
      - `blocking` feature: alternative to async/await.
      - `cargo add serde@1.0.126`.
      - `cargo add serde_json@1.0.64`.
  - `Vec` is similar to an array in JavaScript.
  - An `array` in Rust is a fixed-size collection of similar elements. `&`: to convert an `array` into a `slice` (shared view).
  - The elements of a tuple can have different types.
  - `let x: f64 = 1.1;` (type annotation).
  - It is necessary to add type annotations when defining functions.
  - `let one_thousand = 1_000;`.
  - `f64`, `f32`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8` (unsigned integer), `u16`, `u32`, `u64`, `u128`, `char` (a Unicode validated `u32`).
  - `as`: to convert numbers (`return x as f64 / y as f64;`, for example).
  - An Expression evaluates to a value. A Statement does not evaluate to a value.
  - In a function: `return x * y;` (Statement, `x * y` Expression) or `x * y` (Expression). If a function ends with an Expression, it automatically returns that Expression.
- [direnv](https://direnv.net/): to load and unload environment variables depending on the current directory.
- [Zig](https://ziglang.org/) programming language.

### Snippets

```rust
let message = if x > 1 {
    "..."
} else if x > 1_000 {
    "..."
} else {
    "..."
};
```
