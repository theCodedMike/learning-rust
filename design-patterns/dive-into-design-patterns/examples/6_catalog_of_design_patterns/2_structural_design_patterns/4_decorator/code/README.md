# Decorator

**Decorator** is a structural pattern that allows adding new behaviors to objects dynamically by placing them inside 
special wrapper objects, called *decorators*.

Using decorators you can wrap objects countless number of times since both target objects and decorators follow the 
same interface. The resulting object will get a stacking behavior of all wrappers.



## Input streams decoration
There is a ***practical example*** in Rust’s standard library for input/output operations.

A buffered reader decorates a vector reader adding buffered behavior.

```rust
let mut input = BufReader::new(Cursor::new("Input data"));
input.read(&mut buf).ok();
```

### How to Run

```bash
cargo run --example decorator
```

### Output

```
Read from a buffered reader: Input data
```


## Reference

[Decorator in Rust](https://refactoring.guru/design-patterns/decorator/rust/example)