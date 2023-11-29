# Tips, Tricks and Tools

1. #### Use `cargo-deny` to check for license violations. 
   Sometimes you want to use dependencies that have different licenses despite those licenses being incompatible. Use cargo-deny to list all the licenses in dependencies and throw an error if use of a license is not granted - https://embarkstudios.github.io/cargo-deny/ 
2. #### Use cargo-fmt in workflows
   In workflows like Github and Gitlab actions, run `cargo fmt` to enforce code formatting standards. Run the command
   ```sh
    cargo fmt --all -- --config format_code_in_doc_comments=true --check
   ```
3. #### Use cargo-clippy in workflows
   Use cargo clippy to check for common Rust mistakes when running workflows. Run the command
   ```sh
   cargo clippy --all-targets --all-features -- -D warnings
   ```
4. Use `Cow<str>` instead of `String` so that you only clone memory when necessary.
5. Use `cargo-make` tp run build scripts like when you want to perform multiple commands like run cargo and then copy the resulting binary to a certain folder. Learn more at - https://sagiegurari.github.io/cargo-make/
6. Use `Rust Analyzer` on your favourite code editor if it is supported. Check if your code editor is supported at https://rust-analyzer.github.io/
7. Run `cargo clippy` on save in your favourite code editor
8. Use `dbg!()` instead of `println!()` macro when debugging code since it shows more information like line numbers.
9. Use `thiserror` and `anyhow` (recommended only for binaries on when you don't care about who is consuming you crate) for error handling. https://crates.io/crates/thiserror and https://crates.io/crates/anyhow
10. Use `include_str!()` and `include_bytes!()` inbuilt rust macros  to read the contents of a file at compile time. The str or bytes can be `const` :) . https://doc.rust-lang.org/std/macro.include_str.html and https://doc.rust-lang.org/std/macro.include_bytes.html
11. Modularize your tests
12. Use _ to make large numbers legible eg `1_000_000` instead of `1000000`
13. Use the `swap` function in rustlang std to directly swap two variables without needing to create a temporary variable. https://doc.rust-lang.org/std/mem/fn.swap.html
14. Use `!#[forbid(lint_here)]` to ensure a certain requirement is met for the code to compile. eg `!#[forbid(unsafe_code)]` to ensure that unsafe code is not used in the crate.
15. Use `!#[deny(lint_here)]` to get warnings when a certain requirement is not met eg `!#[deny(missing_docs)]` to emit warnings when public types are not documented. `!#[forbid(missing_docs)]` can also be used to error on compile time if public types are not documented.
16. Use `std::env!()` macro to read the environment of the project at compile time eg. get the current crate version using `env!("PKG_VERSION")` - https://doc.rust-lang.org/std/macro.env.html
17. Use `todo!()` or `panic!()` macros where you have yet to write code to perform a certain task eg 
    ```rust
    fn read_file() {
      todo!()
    }
    ```
    