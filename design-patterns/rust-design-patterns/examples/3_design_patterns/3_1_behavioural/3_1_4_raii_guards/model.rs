use std::ops::Deref;

pub struct Foo {}

impl Foo {
    pub fn foo(&self) {
        println!("Foo: foo");
    }
}

pub struct Mutex<T> {
    data: T, // We keep a reference to our data: T here.
             //..
}

pub struct MutexGuard<'a, T: 'a> {
    lock: &'a Mutex<T>,
    //..
}

// Locking the mutex is explicit.
impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Mutex { data }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        // Lock the underlying OS mutex.
        //..

        // MutexGuard keeps a reference to self
        MutexGuard {
            lock: self,
            //..
        }
    }
}

// Destructor for unlocking the mutex.
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // Unlock the underlying OS mutex.
        //..
    }
}

// Implementing Deref means we can treat MutexGuard like a pointer to T.
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.lock
    }
}

impl<T> Deref for Mutex<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn baz(x: Mutex<Foo>) {
    let xx = x.lock();
    xx.foo(); // foo is a method on Foo.
              // The borrow checker ensures we can't store a reference to the underlying
              // Foo which will outlive the guard xx.

    // x is unlocked when we exit this function and xx's destructor is executed.
}
