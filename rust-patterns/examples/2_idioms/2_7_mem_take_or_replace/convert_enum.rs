use std::mem;

#[derive(Debug)]
pub enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}
pub fn a_to_b(e: &mut MyEnum) -> Option<MyEnum> {
    if let MyEnum::A { name, x: _ } = e {
        // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will
        // be assigned to `*e`).
        Some(MyEnum::B {
            name: mem::take(name),
        })
    } else {
        None
    }
}

#[derive(Debug)]
pub enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}
pub fn swizzle(e: &mut MultiVariateEnum) -> MultiVariateEnum {
    use MultiVariateEnum::*;
    match e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { name } => B {
            name: mem::take(name),
        },
        B { name } => A {
            name: mem::take(name),
        },
        C => D,
        D => C,
    }
}
