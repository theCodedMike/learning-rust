# Memento

**Memento** is a behavioral design pattern that allows making snapshots of an object's state and restoring it in the future.

The Memento doesn't compromise the internal structure of the object it works with, as well as data kept inside the snapshots.



## Conceptual example
This is a conceptual example of Memento pattern.

### How to Run

```bash
cargo run --example memento
```
### Output

```
Originator backup: '1'
Originator backup: '2'
Restored to state: 2
Restored to state: 1
```


## Serde serialization framework
A common way to make a structure serializable is to derive `Serialize` and `Deserialize` traits from the 
[**serde serialization framework**][serde]. Then an object of serializable type can be converted to many different 
formats, e.g. JSON with [**serde_json**][serde_json] crate.

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Originator {
    state: u32,
}
```

### How to Run

```bash
cargo run --example memento-serde
```

### Output

```
{"state":1}
{"state":2}
Restored to state: 2
Restored to state: 1
```


## Reference
[Memento in Rust](https://refactoring.guru/design-patterns/memento/rust/example)


[serde]:https://crates.io/crates/serde
[serde_json]:https://crates.io/crates/serde_json