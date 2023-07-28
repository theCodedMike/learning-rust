# Template Method
Template Method is a behavioral design pattern that allows you to defines a skeleton of an algorithm in a base class 
and let subclasses override the steps without changing the overall algorithm’s structure.


## Conceptual Example

### How to run
```bash
cargo run --example template-method
```

### Output

```
Same client code can work with different concrete implementations:
TemplateMethod says: I am doing the bulk of the work
ConcreteStruct1 says: Implemented Operation1
TemplateMethod says: But I let subclasses override some operations
ConcreteStruct1 says: Implemented Operation2
TemplateMethod says: But I am doing the bulk of the work anyway

Same client code can work with different concrete implementations:
TemplateMethod says: I am doing the bulk of the work
ConcreteStruct2 says: Implemented Operation1
TemplateMethod says: But I let subclasses override some operations
ConcreteStruct2 says: Implemented Operation2
TemplateMethod says: But I am doing the bulk of the work anyway
```


## Reference

[Template Method in Rust](https://refactoring.guru/design-patterns/template-method/rust/example)
