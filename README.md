# Rust Learning Lab

A collection of code written while studying *Programming Rust*.

## Layout

- `programming-rust/exercises` contains small, chapter-focused examples.
- `programming-rust/projects` contains complete programs from the book.
- `programming-rust/hello` is the introductory Cargo project.
- `shad-rust` contains exercises from the SHAD Rust course.
- `practice` contains independent, application-idiom exercises guided by the
  project-local `rust-practice-coach` skill.

Each directory with a `Cargo.toml` is an independent package. Book-derived and
practice work stay separate.

## Usage

Compile every package with [just](https://github.com/casey/just):

```sh
just check
```

Run or test one package with Cargo:

```sh
cargo run --manifest-path programming-rust/projects/gcd/Cargo.toml -- 42 56
cargo test --manifest-path programming-rust/exercises/Cargo.toml
```
