# Array

## Example

Execute the following command to run [`./examples/array.rs`]

```shell
cargo run --example array
```

## Exercises

Exercises are in [`src/lib.r`](./src/lib.rs)

### Exercise 1

```rust
pub fn zeros() -> [u32; 100] {
    todo!();
}
```

Return an array with 100 elements, all equal to 0.

### Exercise 2

```rust
pub fn first_3(s: &[u32]) -> &[u32] {
    todo!();
}
```

Return a slice of the first 3 elements of `s`.

### Exercise 3

```rust
pub fn last_3(s: &[u32]) -> &[u32] {
    todo!();
}
```

Return a slice of the last 3 elements of `s`.

## Test

```shell
cargo test
```
