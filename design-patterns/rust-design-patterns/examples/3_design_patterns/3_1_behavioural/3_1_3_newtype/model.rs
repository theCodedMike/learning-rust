// Some type, not necessarily in the same module or even crate.
struct Foo {
    //..
}

impl Foo {
    // These functions are not present on Bar.
    //..
}

// The newtype.
pub struct Bar(Foo);

impl Bar {
    // Constructor.
    pub fn new() -> Bar {
        Bar(Foo)
    }

    //..
}
