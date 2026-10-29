# Rust Learning Notes

Main notes file. Topic-wise notes are in separate files, linked below.

## Topic Notes

- [Guessing Game](guessing-game.md)
- [Variables, Constants, Literals, and Number Overflow](variables-constants-numbers.md)
- [Common Programming Concepts](common-programming-concepts.md)

## Rust Tools

1. **Cargo** - Tool to create projects, build code, and manage libraries (dependencies).
2. **rustfmt** - Tool to format Rust code in the same style across the team.
3. **Rust Language Server** - IDE extension. It gives errors, suggestions, and Go to Definition in the editor.

## Cargo

### Creating a Project

- Create a new project:

  ```bash
  cargo new PROJECT_NAME
  ```

- This creates a `src` folder with `main.rs`, and a `Cargo.toml` file. `Cargo.toml` stores project settings and the list of libraries.
- To start Cargo in an existing folder: move code into `src` and run `cargo init`. This creates the `Cargo.toml` file.
- Create a project with Git:

  ```bash
  cargo new PROJECT_NAME --vcs=git
  ```

- See all options: `cargo new --help`

### Building and Running

- `cargo build` - Builds the project. You can run the program file yourself, for example: `./target/debug/hello_cargo`
- `cargo run` - Checks for changes, builds, and runs the program.
- `cargo check` - Checks if the code can compile. It does not create a program file. It is faster than `cargo build`.
- `cargo build --release` - Builds the final version of the project for release. This version runs faster.

### Libraries and Updates

- `crates.io` is the website where Rust libraries are available.
- `cargo update` - Updates all libraries and updates the `Cargo.lock` file.