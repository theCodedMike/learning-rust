# Abstract Factory

**Abstract Factory** is a creational design pattern, which solves the problem of creating entire product families 
without specifying their concrete classes.

Abstract Factory defines an interface for creating all distinct products but leaves the actual product creation to 
concrete factory classes. Each factory type corresponds to a certain product variety.

The client code calls the creation methods of a factory object instead of creating products directly with a constructor 
call (`new` operator). Since a factory corresponds to a single product variant, all its products will be compatible.

Client code works with factories and products only through their abstract interfaces. This lets the client code work 
with any product variants, created by the factory object. You just create a new concrete factory class and pass it to 
the client code.

> If you can't figure out the difference between various factory patterns and concepts, then read our [**Factory Comparison**](https://refactoring.guru/design-patterns/factory-comparison).


## GUI Elements Factory

This example illustrates how a GUI framework can organize its classes into independent libraries:
1. The `gui` library defines interfaces for all the components. It has no external dependencies.
2. The `windows-gui` library provides Windows implementation of the base GUI. Depends on `gui`.
3. The `macos-gui` library provides Mac OS implementation of the base GUI. Depends on `gui`.

The `app` is a client application that can use several implementations of the GUI framework, depending on the current 
environment or configuration. However, most of the `app` code *doesn't depend on specific types of GUI elements*. All 
the client code works with GUI elements through abstract interfaces (traits) defined by the `gui` lib.

There are two approaches to implementing abstract factories in Rust:
- using generics (*static dispatch*)
- using dynamic allocation (*dynamic dispatch*)

When you're given a choice between static and dynamic dispatch, there is rarely a clear-cut correct answer. You'll want 
to use static dispatch in your libraries and dynamic dispatch in your binaries. In a library, you want to allow your 
users to decide what kind of dispatch is best for them since you don't know what their needs are. If you use dynamic 
dispatch, they're forced to do the same, whereas if you use static dispatch, they can choose whether to use dynamic 
dispatch or not.


## `app/main.rs`
Here, the abstract factory is implemented via generics which lets the compiler create a code that does NOT require 
dynamic dispatch in runtime.

```rust
pub trait GuiFactory {
    type B: Button;
    type C: Checkbox;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
}
```

### How to Run

```bash
cargo run --example abstract-factory-app
```

### Output

```
Windows button has pressed
Windows button has pressed
Windows checkbox has switched
Windows checkbox has switched
```


## `app-dyn/main.rs`

If a concrete type of abstract factory is not known at the compilation time,
then is should be implemented using `Box` pointers.

```rust
pub trait GuiFactoryDynamic {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}
```

### How to Run

```bash
cargo run --example abstract-factory-app-dyn
```

### Output

```
MacOS button has pressed
MacOS button has pressed
MacOS button has pressed
MacOS checkbox has switched
MacOS checkbox has switched
```

## Reference

[Abstract Factory in Rust](https://refactoring.guru/design-patterns/abstract-factory/rust/example)
