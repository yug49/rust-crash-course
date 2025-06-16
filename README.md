# Rust Crash Course

### Course intro

- [Course intro](./notes/course_intro.md)
- [Setup](./notes/course_setup.md)

### Intro

- [Install cargo](./notes/install.md)
- [Hello world](./topics/hello)
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
  - `derive(Debug, PartialEq)`
  - `Result` and `Option`
- Struct
  - update
- Vector
- Hash map

### Control flow

- If / else
- Loop
- Match
- If let and let else

### Ownership

- Stack and heap
- Ownership
- Borrowing rules

### Error handling

- Error handling
  - `panic`
  - `Option`
  - `Result`
- `unwrap` and `expect`
- `?`

### Modules

- Mod
  - `mod`
  - `pub`
  - `use`
  - `super`
  - `crate`

### Generic types and Traits

- Generic types
- Methods
- Trait
- Generic trait
- Trait bound
  - `where`
  - `+`
- Lifetimes
- Iterators
  - `map`
  - `filter`
  - `collect`

### Concurrency

- Async / await
  - runtime
  - Future
  - lazy
  - concurrent
  - join!
  - async block
  - spawn
- `select!`

# Resources

- [Rust](https://www.rust-lang.org/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [rustlings](https://github.com/rust-lang/rustlings/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust playground](https://play.rust-lang.org/)
