# Rust config files

1. #### `config.toml` file
    To specify global custom settings for `cargo` , add a `config.toml` file inside the `.cargo` folder where `cargo` Rust package manager was installed when Rust was installed.


    ```sh
    # Directory structure
    |- user_home_folder
        |-- .rustup
        |-- .cargo
                |-- bin
                |-- config.toml // Create this file for custom settings for cargo package manager to use when compiling projects globally
    ```

    An example is when using `sccache` to cache project files as seen in the chapter on [Compile Times](compile-times.md)

    To specify local configuration file for cargo to use to build a local project only, create a `config.toml` file inside `.cargo` directory in your project.


    ```sh
    # Directory structure
    |- my_project
        |-- src
        |-- Cargo.toml
        |-- .cargo
                |-- config.toml // Create this file for custom settings for cargo package manager to use when compiling project locally
    ```


More details can be found at https://doc.rust-lang.org/cargo/reference/config.html

2. #### `rust-toolchain` file
   This file is used to tell cargo which Rust toolchain to use when compiling the current project directory or `cargo workspace`. The toolchains can be `stable`, `beta` or `nightly` where the version can also be specified eg `nightly-2020-03-15` where the format is `toolchain-yyyy-mm-dd`
3. #### `Cargo.toml`
    This file is the manifest file for the current project. It can be a workspace or not.
    More information at https://doc.rust-lang.org/cargo/reference/manifest.html
4. #### `rustfmt.toml` file
    Rust has a default formatter called `rustfmt` which is used to format code. This can be useful when enforcing formatting on the project directory especially when the project has multiple contributors. The config file can be named as `rustfmt.toml` or `.rustfmt.toml`
    More info at https://rust-lang.github.io/rustfmt
5. #### `clippy.toml` file
    [Clippy](https://doc.rust-lang.org/clippy/index.html) is a collection of lints to catch common mistakes and improve your Rust code. The `clippy.toml` or `.clippy.toml` file is used to configure clippy.
    More information at https://doc.rust-lang.org/clippy/configuration.html



#### `rustdoc`
Rust can produce documentation of code using `rustdoc` tool.
More info at https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
