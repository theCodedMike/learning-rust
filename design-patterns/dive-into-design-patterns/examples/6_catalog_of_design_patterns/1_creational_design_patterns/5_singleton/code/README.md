# Singleton

**Singleton** is a creational design pattern, which ensures that only one object of its kind exists and provides a 
single point of access to it for any other code.

Singleton has almost the same pros and cons as global variables. Although they're super-handy, they break the 
modularity of your code.

You can't just use a class that depends on a Singleton in some other context, without carrying over the Singleton to 
the other context. Most of the time, this limitation comes up during the creation of unit tests.


## Rust specifics
By definition, Singleton is a *global mutable object*. In Rust this is a `static mut` item. Thus, to avoid all sorts of 
concurrency issues, the function or block that is either reading or writing to a mutable static variable should be 
marked as an [**unsafe block**](https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics).

For this reason, the Singleton pattern can be perceived as unsafe. However,
the pattern is still widely used in practice. A good read-world example of
Singleton is a `log` crate that introduces `log!`, `debug!` and other logging
macros, which you can use throughout your code after setting up a concrete
logger instance, such as [**env_logger**](https://crates.io/crates/env_logger).
As we can see, `env_logger` uses
[**log::set_boxed_logger**](https://docs.rs/log/latest/log/fn.set_boxed_logger.html)
under the hood, which has an `unsafe` block to set up a global logger object.

- In order to provide safe and usable access to a singleton object,
  introduce an API hiding `unsafe` blocks under the hood.
- See the thread about a mutable Singleton on [**Stackoverflow**](https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton) for more information.


Starting with Rust 1.63, `Mutex::new` is const, you can use global static `Mutex` locks without needing lazy 
initialization. See the *Singleton using Mutex* example below.



## Safe Singleton
A pure safe way to implement Singleton in Rust is using no global variables at all and passing everything around 
through function arguments. The oldest living variable is an object created at the start of the `main()`.

### How to Run

```bash
cargo run --example singleton-safe
```

### Output

```
Final state: 1
```


## Lazy Singleton
This is a singleton implementation via `lazy_static!`, which allows declaring a static variable with lazy initialization 
at first access. It is actually implemented via `unsafe` with `static mut` manipulation, however, it keeps your code 
clear of `unsafe` blocks.

### How to Run

```bash
cargo run --example singleton-lazy
```

### Output

```
Called 3
```


## Singleton using Mutex
Starting with `Rust 1.63`, it can be easier to work with global mutable singletons, although it's still preferable to 
avoid global variables in mostcases.

Now that `Mutex::new` is `const`, you can use global static `Mutex` locks without needing lazy initialization.

### How to Run

```bash
cargo run --example singleton-mutex
```

### Output

```
Called 3 times
```

## Reference

[Singleton in Rust](https://refactoring.guru/design-patterns/singleton/rust/example)
