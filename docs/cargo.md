# Cargo
This is the package manager for Rust, it handles dependencies and project initialization and management.

## Usage
### Basic usage
I ran `cargo init hello_cargo` and it made a Cargo.toml with initial configurations, a src dir and a main file already, very nice
It also should create a git repo, this can be overridden with `--vcs=git`

### Building with Cargo
There is also the option to build the executable with cargo, running `cargo build` in a directory with a Cargo.toml.
By default, the target is assumed to be debug, so the executable is created in a dir `target/debug/{filename}`.
Also creates a classic lockfile to keep track of dependency versions

Similar to Go, we can compile and execute in one command: `cargo run`

There is also the feature to use `cargo check` which checks the source code to make sure it compiles but does not generate a binary file.

### Building for release
Using `cargo build --release` compiles the code with optimizations, this makes the compile time considerably longer, and the output goes to the release dir in the target.
When running benchmarks is the obvious choice to use the release command
