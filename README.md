# ipma-cli

A [Rust](https://www.rust-lang.org/) CLI to get weather forecasts in Portugal from [IPMA](https://www.ipma.pt/en/).

## References

- Chris Biscardi's [Rust Adventure](https://www.rustadventure.dev/) email course.
- Chris Biscardi's [weather-cli](https://github.com/rust-adventure/weather-cli) crate.
- Richard Feldman's [The Rust Programming Language](https://frontendmasters.com/courses/rust/) course ([Frontend Masters](https://frontendmasters.com/)).

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
  - Default return value of a function: `()` ("unit"). Similar to `void`.
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
  - Macro: it can be seen as a kind of syntactic sugar.
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) (VS Code extension).
  - Collections:
    - Tuples:
      - `let point: (i64, i64, i64) = (0, 0, 0);`.
      - `point.0`/`point.1`/`point.2` for tuple indexing, for example.
      - Destructuring: `let (x, y, z) = point;` or `let (x, y, _) = point;`.
    - Structs:
      - Destructuring: `let Point { x, y, z } = point;` or `let Point { x, y: _, z } = point;` or `let Point { x, z, .. } = point;`.
      - Naming convention: `UpperCamelCase`.
    - Arrays:
      - Stack-allocated.
      - Only one type and fixed size: `let mut years: [i32; 3] = [1995, 2000, 2005];`.
      - `let first_year = years[0];`.
      - Destructuring: `let [_, second_year, third_year] = years;`.
      - It is possible to iterate over them, unlike tuples and structs (`for year in years.iter() { ... }`).
    - In memory, they (tuples, structs, and arrays) are all represented as adjacent bytes with no extra metadata/overhead.
    - Pattern matching: similar to a switch statement.
    - Use `impl` to add methods to enums. You can think of these as functions defined in a specific namespace. `self` ([keyword](https://doc.rust-lang.org/std/keyword.self.html)) is similar to Python (`color.rgb()` vs. `Color::rgb(color)`).
    - `Option` enum ([documentation](https://doc.rust-lang.org/std/option/enum.Option.html)). Type `Option` represents an optional value: every `Option` is either `Some` and contains a value, or `None`, and does not. `let email: Option<String> = Some(email_str);`. `<...>` for type parameters.
    - Vectors:
      - Heap-allocated (dynamic length).
      - `let mut years: Vec<i32> = vec![1995, 2000, 2005];`.
      - `years.push(2010);`.
      - `let length: usize = years.len();`. On a 64-bit (32-bit) system, `usize` is equivalent to `u64` (`u32`).
      - It is possible to iterate over them.
      - `let nums = Vec::with_capacity(5);`: free memory slots for the vector, even if the vector does not occupy them, thus accommodating future pushes more efficiently, for example.
    - Memory management (automatic):
      - Rust's scope-based heuristics (no GC).
      - **Ownership**:
        - When Rust allocates, it assigns an owner to that allocation (the owner is basically the scope).
        - Ownership essentially refers to the responsibility to deallocate.
        - Ownership is transferable (when exiting a scope/returning).
        - A _move_ refers to the transfer of ownership from one scope to another.
        - Use-after-move compiler error (add a `return` to the relevant function to trigger a move or use the `.clone()` method on a vector, for example).
      - **Borrowing**:
- [direnv](https://direnv.net/): to load and unload environment variables depending on the current directory.
- [Zig](https://ziglang.org/) programming language.
- [TextMate](https://macromates.com/):
  - Text editor for macOS.
  - [EditorConfig](https://github.com/Mr0grog/editorconfig-textmate) plugin.
  - Show hidden files/dotfiles in file browser (when the focus is on it): `⌥⌘i` ([source](https://stackoverflow.com/a/19737418)).
  - Install the Rust bundle: `TextMate` > `Preferences...` > `Bundles`.
  - Change font size: `View` > `Font` > `Show Fonts`.
  - `.tm_properties` file:
    - Local/project settings.
    - Reference: `~/Downloads/TextMate.app/Contents/Resources/Default.tmProperties` file (assuming the application is in the `~/Downloads/` folder).
- [EME](https://github.com/egoist/eme) (Markdown editor).
- [Victor Mono](https://rubjo.github.io/victor-mono/):
  - Open-source monospaced font.
  - [Repo](https://github.com/rubjo/victor-mono).
  - [Package](https://www.npmjs.com/package/victormono).
  - [Example](https://www.programmingfonts.org/#victor-mono).

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

---

```rust
// Enum

// type
enum Color {
    Green,  // variant
    Yellow, // variant
    Red,    // variant
    // It is possible to have structs or tuples (payload) as well.
}

let go: Color = Color::Green;

// Pattern Matching

let current_color = Color::Yellow;

// It can be used similarly to an `if` to set the value of a variable.
match current_color {
    Color::Green => {
        println!("");
    }
    Color::Yellow => {
        println!("");
    }
    _ => {
        // Catchall pattern.
        println!("");
    }
}


let last_city = match city_names.pop() {
    Some(inner_value) => inner_value,
    None => "",
};
```
