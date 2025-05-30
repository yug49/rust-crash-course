# Rust Crash Course

TODO: examples
TODO: exercises

### Course intro

- Setup
  - Folder structure
    - examples
  - How to do exercises

### Intro

- [Install cargo](./notes/install.md)
- Hello world
  - Attributes
  - Macros
  - `main`
  - `println!`
  - `cargo run` `examples`, `bin` and `main`
- Variable
  - Immutable by default
  - `let`
  - `mut`
  - Constant
  - Shadowing
  - Type placeholder `_`
  - `println!`
- Function
  - Return value
  - Implicit return
  - No return value

### Data

- Scalar types
  - `i32`, `u32`, `f32`, `bool`, `char`
  - `isize`, `usize`
  - Type conversion
  - Min and max value
  - Integer overflow
- Tuple
  - Destructure, `_`
  - Empty tuple
  - Nested
  - Return multiple outputs
- Array
  - Array - collection of elements with length known at compile time
  - Slice - collection of elements with length not known at compile time
- `String` and `&str`
  - `+`
  - `format!`
- Enum
  - `derive(Debug)`
- `Result` and `Option`
- Struct
- Vector
- Hash map

### Control flow

- If / else
- If let and let else
- Loop
- Match

### Ownership

- Stack and heap
- Ownership
- Borrow and references

### Error handling

- Error handling
- Expect, unwrap
- `?`

### Modules

- Mod

### Generic types

- Basic

### Trait

- Basic
- Lifetimes
- `derive`, `Debug`, etc...
- Iterators

### Concurrency

- Async / await

# Resources

- [Rust](https://www.rust-lang.org/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [rustlings](https://github.com/rust-lang/rustlings/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust playground](https://play.rust-lang.org/)
