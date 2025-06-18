# `?` operator

## Example

Execute the following command to run [`./examples/question.rs`]

```shell
cargo run --example question
```

## Exercises

Exercises are in [`src/lib.rs`](./src/lib.rs)

### Exercise 1

```rust
pub fn sum(nums: &[&str]) -> Result<u32, String> {
    todo!();
}
```

Parse the slice of string slices into `u32` and return their sum.

Call the `parse` function to parce a `&str` into `u32`.

Use `?` to make your code shorter.

### Test

```shell
cargo test
```
