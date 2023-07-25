# Observer

**Observer** is a behavioral design pattern that allows some objects to notify other objects about changes in their state.

The Observer pattern provides a way to subscribe and unsubscribe to and from these events for any object that 
implements a subscriber interface.


## Conceptual example

In Rust, a convenient way to define a subscriber is to have a function as a callable object with complex logic passing
it to a event publisher.

In this Observer example, Subscribers are either *a lambda function* or *an explicit function* subscribed to the 
event. Explicit function objects could be also unsubscribed (although, there could be limitations for some function 
types).

### How to Run

```bash
cargo run --example observer
```
### Output

```
Save log to /path/to/log/file.txt: Load file test1.txt
Save log to /path/to/log/file.txt: Load file test2.txt
Email to admin@example.com: Save file test2.txt
```


## Reference

[Observer in Rust](https://refactoring.guru/design-patterns/observer/rust/example)
