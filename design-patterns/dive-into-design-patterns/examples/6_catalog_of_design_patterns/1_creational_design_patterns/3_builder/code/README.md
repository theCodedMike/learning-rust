# Builder

**Builder** is a creational design pattern, which allows constructing of complex objects step by step.

Unlike other creational patterns, Builder doesn't require products to have a common interface. That makes it possible 
to produce different products using the same construction process.


## Car & car manual builders
This slightly synthetic example illustrates how you can use the Builder pattern to construct totally different products 
using the same building process. For example, the trait `Builder` declares steps for assembling a car. However, 
depending on the builder implementation, a constructed object can be something different, for example, a car manual. 
The resulting manual will contain instructions from each building step, making it accurate and up-to-date.

The **Builder** design pattern is not the same as the **Fluent Interface** idiom (that relies on *method chaining*), 
although Rust developers sometimes use those terms interchangeably.

1. **Fluent Interface** is a way to chain methods for constructing or modifying an object using the following approach:
   `let car = Car::default().places(5).gas(30)`.
   It's pretty elegant way to construct an object. Still, such a code may not be an instance of the Builder pattern.
2. While the **Builder** pattern also suggests constructing object step by step, it also lets you build different types 
   of products using the same construction process.


### How to Run

```bash
cargo run --example builder
```

### Output

```
Car built: SportsCar

Car manual built:
Type of car: CityCar
Count of seats: 2
Engine: volume - 1.2; mileage - 0
Transmission: Automatic
GPS Navigator: Functional
```

## Reference

[Builder in Rust](https://refactoring.guru/design-patterns/builder/rust/example)
