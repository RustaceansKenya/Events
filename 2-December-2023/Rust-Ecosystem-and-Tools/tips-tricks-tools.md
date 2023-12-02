# Tips, Tricks and Tools

1. Use `cargo-binstall`
   The binary crate `cargo-binstall` is used to to install crate binaries from github if the maintainers of the crate have releases binaries. This improves productivity because rather than compiling and installing the binaries, it will download the already released binaries of the crate.
2. #### Use `cargo-deny` to check for license violations. 
   Sometimes you want to use dependencies that have different licenses despite those licenses being incompatible. Use cargo-deny to list all the licenses in dependencies and throw an error if use of a license is not granted - https://embarkstudios.github.io/cargo-deny/ 
3. #### Use cargo-fmt in workflows
   In workflows like Github and Gitlab actions, run `cargo fmt` to enforce code formatting standards. Run the command
   ```sh
    cargo fmt --all -- --config format_code_in_doc_comments=true --check
   ```
4. #### Use cargo-clippy in workflows
   Use cargo clippy to check for common Rust mistakes when running workflows. Run the command
   ```sh
   cargo clippy --all-targets --all-features -- -D warnings
   ```
5. Use `Cow<str>` instead of `String` so that you only clone memory when necessary.
6. Use `cargo-make` tp run build scripts like when you want to perform multiple commands like run cargo and then copy the resulting binary to a certain folder. Learn more at - https://sagiegurari.github.io/cargo-make/
7. Use `Rust Analyzer` on your favourite code editor if it is supported. Check if your code editor is supported at https://rust-analyzer.github.io/
8. Run `cargo clippy` on save in your favourite code editor
9.  Use `dbg!()` instead of `println!()` macro when debugging code since it shows more information like line numbers.
10. Use `thiserror` and `anyhow` (recommended only for binaries on when you don't care about who is consuming you crate) for error handling. https://crates.io/crates/thiserror and https://crates.io/crates/anyhow
11. Use `include_str!()` and `include_bytes!()` inbuilt rust macros  to read the contents of a file at compile time. The str or bytes can be `const` :) . https://doc.rust-lang.org/std/macro.include_str.html and https://doc.rust-lang.org/std/macro.include_bytes.html
12. Modularize your tests
13. Use _ to make large numbers legible eg `1_000_000` instead of `1000000`
14. Use the `swap` function in rustlang std to directly swap two variables without needing to create a temporary variable. https://doc.rust-lang.org/std/mem/fn.swap.html
15. Use `!#[forbid(lint_here)]` to ensure a certain requirement is met for the code to compile. eg `!#[forbid(unsafe_code)]` to ensure that unsafe code is not used in the crate.
16. Use `!#[deny(lint_here)]` to get warnings when a certain requirement is not met eg `!#[deny(missing_docs)]` to emit warnings when public types are not documented. `!#[forbid(missing_docs)]` can also be used to error on compile time if public types are not documented.
17. Use `std::env!()` macro to read the environment of the project at compile time eg. get the current crate version using `env!("PKG_VERSION")` - https://doc.rust-lang.org/std/macro.env.html
18. Use `todo!()` or `panic!()` macros where you have yet to write code to perform a certain task eg 
    ```rust
    fn read_file() {
      todo!()
    }
    ```
    
19. Use `jemallocator` as the allocator
    The system allocator used to allocate memory can be slow to allocate memory. Switching the alloactor to a custom allocator can increase the speed of the running Rust program. A common custom allocator used in Rust is `jemallocator`. Add this to your `main.rs` file
    ```toml
    [dependencies]
    jemallocator = "*"
    ```

    ```rust
    #[cfg(not(target_env = "msvc"))]
      use jemallocator::Jemalloc;

      #[cfg(not(target_env = "msvc"))]
      #[global_allocator]
      static GLOBAL: Jemalloc = Jemalloc;
    ```

    NOTE: Not all Rust programs are bottlenecked on allocations, and those which are can use #[global_allocator] to opt-in to a jemalloc-based global allocator (through the jemallocator or any other allocator crate).

    More info on `jemallocator` 
      - Github - https://github.com/gnzlbg/jemallocator
      - Article - https://web.archive.org/web/20220327090938/https://christianfscott.com/making-rust-as-fast-as-go/
      - Article on when `jemallocator` was removed - https://github.com/rust-lang/rust/issues/36963