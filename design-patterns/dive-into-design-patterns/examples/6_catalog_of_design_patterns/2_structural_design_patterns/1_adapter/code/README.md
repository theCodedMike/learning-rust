# Adapter

**Adapter** is a structural design pattern, which allows incompatible objects to collaborate.

The Adapter acts as a wrapper between two objects. It catches calls for one object and transforms them to format and 
interface recognizable by the second object.


## Adapter in Rust
In this example, the `trait SpecificTarget` is incompatible with a `call` function which accepts `trait Target` only.

```rust
fn call(target: impl Target);
```

The adapter helps to pass the incompatible interface to the `call` function.

```rust
let target = TargetAdapter::new(specific_target);
call(target);
```


### How to Run

```bash
cargo run --example adapter
```


### Output

```
A compatible target can be directly called: 'Ordinary request.'
Adaptee is incompatible with client: '.tseuqer cificepS'
But with adapter client can call its method: 'Specific request.'
```


## Reference

[Adapter in Rust](https://refactoring.guru/design-patterns/adapter/rust/example)