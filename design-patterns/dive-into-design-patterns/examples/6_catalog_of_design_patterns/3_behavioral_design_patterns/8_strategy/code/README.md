# Strategy
**Strategy** is a behavioral design pattern that turns a set of behaviors into objects and makes them interchangeable 
inside original context object.

The original object, called context, holds a reference to a strategy object. The context delegates executing the 
behavior to the linked strategy object. In order to change the way the context performs its work, other objects may 
replace the currently linked strategy object with another one.


## Conceptual Example
A conceptual Strategy example via traits.

### How to run

```bash
cargo run --example strategy
```

### Output

```
Walking route from Home to Club: 4 km, 30 min
Walking route from Club to Work: 4 km, 30 min
Public transport route from Home to Club: 3 km, 5 min
Public transport route from Club to Work: 3 km, 5 min
```


## Functional approach
Functions and closures simplify Strategy implementation as you can inject behavior right into the object without 
complex interface definition.

It seems that Strategy is often implicitly and widely used in the modern development with Rust, e.g. it's just like 
iterators work:

```rust
let a = [0i32, 1, 2];

let mut iter = a.iter().filter(|x| x.is_positive());
```
### How to run
```bash
cargo run --example strategy-func
```
### Output

```
Walking route from Home to Club: 4 km, 30 min
Walking route from Club to Work: 4 km, 30 min
Public transport route from Home to Club: 3 km, 5 min
Public transport route from Club to Work: 3 km, 5 min
Specific route from Home to Club
Specific route from Club to Work
```


## Reference

[Strategy in Rust](https://refactoring.guru/design-patterns/strategy/rust/example)
