# Compile Times
Rust compiles the project on each run checking if the code violates any of it's ownership and borrowing rules. This can increase compile times if the project is large or has a lot of dependencies. Hereare  some tools and tricks to increase speed of compilation.

1. `incremental compilation`
    Rust uses incremental compilation by default where on first compile run, Rust will compile the whole project and then compile only the files that change on each other subsequent run. Using `cargo clean` to compile the project again from scratch also obeys this rule. Learn more at https://blog.rust-lang.org/2016/09/08/incremental.html
2. #### Use `sccache`
    [Sccache](https://crates.io/crates/sccache) is used as a compiler wrapper and avoids compilation when possible, storing a cache in a remote storage using various cloud storage. To use sccache, add the following code to the `config.toml` file in the global `.cargo` directory as outlined in chapter on [Rust config file](config_toml_file.md). More info about sccache at https://crates.io/crates/sccache#installation
3. #### Use `cargo workspace`
    For large projects, use cargo workspaces to split the project into smaller submodules. Learn more at https://doc.rust-lang.org/stable/book/ch14-03-cargo-workspaces.html?highlight=workspace#creating-a-workspace

4. #### Use `mold` as the linker

    Mold is a linker that increases linker performance compared to the default linker `ld`. It uses parallelism to achieve higher speeds compared to Rust's default linker. More about mold at https://github.com/rui314/mold

Some articles with more information about compile times
- https://web.archive.org/web/20231126165748/https://benw.is/posts/how-i-improved-my-rust-compile-times-by-seventy-five-percent
- https://web.archive.org/web/20231126224303/https://endler.dev/2020/rust-compile-times/