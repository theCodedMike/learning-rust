# Bridge

**Bridge** is a structural design pattern that divides business logic or huge class into separate class hierarchies that 
can be developed independently.

One of these hierarchies (often called the Abstraction) will get a reference to an object of the second hierarchy 
(Implementation). The abstraction will be able to delegate some (sometimes, most) of its calls to the implementations 
object. Since all implementations will have a common interface, they'd be interchangeable inside the abstraction.



## Devices and Remotes
This example illustrates how the Bridge pattern can help divide the monolithic code of an app that manages devices and 
their remote controls. The Device classes act as the implementation, whereas the Remotes act as the abstraction.

### How to Run

```bash
cargo run --example bridge
```

### Output

```
Tests with basic remote.
Remote: power toggle
------------------------------------
| I'm TV set.
| I'm enabled
| Current volume is 30%
| Current channel is 1
------------------------------------

Tests with advanced remote.
Remote: power toggle
Remote: mute
------------------------------------
| I'm TV set.
| I'm enabled
| Current volume is 0%
| Current channel is 1
------------------------------------

Tests with basic remote.
Remote: power toggle
------------------------------------
| I'm radio.
| I'm enabled
| Current volume is 30%
| Current channel is 1
------------------------------------

Tests with advanced remote.
Remote: power toggle
Remote: mute
------------------------------------
| I'm radio.
| I'm enabled
| Current volume is 0%
| Current channel is 1
------------------------------------
```


## Reference

[Bridge in Rust](https://refactoring.guru/design-patterns/bridge/rust/example)