# Prototype

**Prototype** is a creational design pattern that allows cloning objects, even complex ones, without coupling to their 
specific classes.

All prototype classes should have a common interface that makes it possible to copy objects even if their concrete 
classes are unknown. Prototype objects can produce full copies since objects of the same class can access each other’s 
private fields.


## Built-in Clone trait
Rust has a built-in `std::clone::Clone` trait with many implementations for various types (via `#[derive(Clone)]`). 
Thus, the Prototype pattern is ready to use out of the box.


```rust
let mut circle2 = circle1.clone();
```

### How to Execute

```bash
cargo run --example prototype
```

### Output

```
Circle 1: 10, 15, 10
Circle 2: 10, 15, 77
```

## Reference

[Prototype in Rust](https://refactoring.guru/design-patterns/prototype/rust/example)
