# Visitor

**Visitor** is a behavioral design pattern that allows adding new behaviors to existing class hierarchy without 
altering any existing code.

> Read why Visitors can’t be simply replaced with method overloading in our article [**Visitor and Double Dispatch**](../../10_visitor).


## Deserialization
A real-world example of the Visitor pattern is [**serde serialization framework**](https://serde.rs) and its 
deserialization model (see [**Serde data model**](https://serde.rs/data-model.html)).

1. `Visitor` should be implemented for a deserializable type.
2. `Visitor` is passed to a `Deserializer` (an "Element" in terms of the Visitor Pattern), which accepts and drives 
   the `Visitor` in order to construct a desired type.

Let's reproduce this deserializing model in our example.

### How to Run

```bash
cargo run --example visitor
```

### Output

```
Ok(TwoValuesStruct { a: 123, b: 456 })
Ok(TwoValuesStruct { a: 123, b: 456 })
Ok(TwoValuesArray { ab: [123, 456] })
Error: parse_str unimplemented
```



## Reference

[Visitor in Rust](https://refactoring.guru/design-patterns/visitor/rust/example)