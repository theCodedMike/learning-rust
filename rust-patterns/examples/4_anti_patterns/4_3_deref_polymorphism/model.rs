use std::ops::Deref;

pub struct Foo {}

impl Foo {
    pub fn m(&self) {
        //..
        println!("foo");
    }
}

pub struct Bar {
    pub(crate) f: Foo,
}

impl Deref for Bar {
    type Target = Foo;
    fn deref(&self) -> &Foo {
        &self.f
    }
}
