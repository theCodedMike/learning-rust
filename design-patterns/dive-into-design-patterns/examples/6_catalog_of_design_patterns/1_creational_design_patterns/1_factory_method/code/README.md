# Factory Method

**Factory method** is a creational design pattern which solves the problem of creating product objects without 
specifying their concrete classes.

The Factory Method defines a method, which should be used for creating objects instead of using a direct constructor 
call (`new` operator). Subclasses can override this method to change the class of objects that will be created.

> If you can't figure out the difference between various factory patterns and concepts, then read our [**Factory Comparison**](https://refactoring.guru/design-patterns/factory-comparison).



## Dialog Rendering
This example illustrates how to organize a GUI framework into independent modules using **dynamic dispatch**:
1. The `gui` module defines interfaces for all the components. It has no external dependencies.
2. The `html_gui` module provides HTML implementation of the base GUI. Depends on `gui`.
3. The `windows_gui` module provides Windows implementation of the base GUI. Depends on `gui`.

The `app` is a client application that can use several implementations of the GUI framework, depending on the current 
environment or configuration. However, most of the app code doesn’t depend on specific types of GUI elements. All 
client code works with GUI elements through abstract interfaces defined by the `gui` module.

The [**Abstract Factory**](https://refactoring.guru/design-patterns/abstract-factory/rust/example) example demonstrates 
an even greater separation of a factory interface and its implementations.

### How to Run

```bash
cargo run --example factory-method-render-dialog
```

### Output

```
-- No OS detected, creating the HTML GUI --
<button>Test Button</button>
Click! Button says - 'Hello World!'
Dialog - Refresh
```



## Maze Game
This example illustrates how to implement the Factory Method pattern using **static dispatch** (generics).

*Inspired by the Factory Method [**example from the GoF book**](https://en.wikipedia.org/wiki/Factory_method_pattern).*

### How to Run

```bash
cargo run --example factory-method-maze-game
```

### Output

```
Loading resources...
Starting the game...
Magic Room: Infinite Room
Magic Room: Red Room
Loading resources...
Starting the game...
Ordinary Room: #2
Ordinary Room: #1
```


## Reference

[Factory Method in Rust](https://refactoring.guru/design-patterns/factory-method/rust/example)
