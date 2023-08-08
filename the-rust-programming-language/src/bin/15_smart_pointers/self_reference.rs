#![allow(dead_code)]
#![allow(unused)]

use ouroboros::self_referencing;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

/// 补充: 自引用
///
/// cargo r --bin self-ref
///
/// ## 目录
/// ### 什么是自引用
///
/// ### 解决方式: 使用Option
///
/// ### 解决方式: 使用Unsafe
///
/// ### 解决方式: 使用!Unpin(Pin), 即 PhantomPinned
///   Unpin trait is automatically implemented for almost every type.
/// ```rust
///     impl !Unpin for PhantomPinned {}
/// ```
///   Pin: Types that pin data to its location in memory.
///   
///   At a high level, a Pin<P> ensures that the pointee of any pointer type P has a stable location in memory,
///   meaning it cannot be moved elsewhere and its memory cannot be deallocated until it gets dropped.
///
///   By default, all types in Rust are movable.
///
/// ### 解决方式: 使用三方库
///
fn main() {
    /* 什么是自引用 */
    #[derive(Debug)]
    struct Person<'a> {
        name: String,
        nickname: &'a str, // 该引用指向上面的name
    }
    let s = "Birds sing".to_string();
    /*
    // 显然以下这种方式是不行的
    let bird = Person {
        name: s,
        nickname: &s, // borrow of moved value: `s`
    };
    */
    println!("{:?}", s); // "Birds sing"
    println!();

    /* 使用Option */
    let mut jet = PersonOption {
        name: "JetBrains".to_string(),
        nickname: None,
    };
    jet.nickname = Some(&jet.name);
    println!("{:?}", jet); // PersonOption { name: "JetBrains", nickname: Some("JetBrains") }
    println!();

    /* 使用Unsafe */
    let mut self_ref = PersonUnsafe::new("use unsafe".to_string());
    self_ref.init();
    println!("{:?}", self_ref); // PersonUnsafe { name: "use unsafe", nickname: 0x6edd3ff3a0 }
    println!("{}", self_ref.get_value()); // use unsafe
    println!("{}", self_ref.get_nickname()); // use unsafe
    println!();

    /* 使用Pin */
    let mut a = String::from("a");
    let mut b = String::from("b");
    println!("{}, address: {:p}", a, a.as_ptr()); // a: 0x1b95224ebb0  变量a -> 值"a"
    println!("{}, address: {:p}", b, b.as_ptr()); // b: 0x1b95224ebd0  变量b -> 值"b"
    std::mem::swap(&mut a, &mut b);
    println!("{}, address: {:p}", a, a.as_ptr()); // b: 0x1b95224ebd0  变量a -> 值"b"
    println!("{}, address: {:p}", b, b.as_ptr()); // a: 0x1b95224ebb0  变量b -> 值"a"  说明a、b在内存中的地址改变了

    let mut one = PersonPin::new("fake_1".to_string(), "1".to_string());
    // one: PersonPin { name: "1", nickname: 0x25430487ed0, _pin: PhantomPinned }, address: 0x25430487ed0
    println!("one, {:?}, address: {:p}", one, one);
    println!("name_address: {:?}", NonNull::from(&one.name)); // 0x25430487ed0
    println!("nickname: {}", one.get_nickname()); // 1
    let mut two = PersonPin::new("fake_2".to_string(), "2".to_string());

    // Since our type doesn't implement Unpin, this will fail to compile:
    //std::mem::swap(&mut *one, &mut *two); // cannot borrow as mutable
    //*one.get_name_mut() = "100".to_string(); // cannot borrow data in dereference of `Pin<Box<PersonPin>>` as mutable
    //one.name = "100".to_string(); // cannot assign to data in dereference of `Pin<Box<PersonPin>>`
    //std::mem::swap(one.get_name_mut(), one.get_fake_name_mut()); // cannot borrow data in dereference of `Pin<Box<PersonPin>>` as mutable
    // 总之被Pin包裹后, 无法拿到&mut T，所以其地址不会被移动
    println!();

    /* 使用ouroboros 三方库: rental owning-ref */
    let mut my_value = MyStructBuilder {
        int_data: 42,
        float_data: 3.14,
        int_reference_builder: |int_data: &i32| int_data,
        float_reference_builder: |float_data: &mut f32| float_data,
    }
    .build();

    println!("int_data: {:?}", my_value.borrow_int_data()); // 42
    println!("int_reference: {}", my_value.borrow_int_reference()); // 42
                                                                    //no method named `borrow_float_data`
                                                                    //println!("float_data: {}", my_value.borrow_float_data());
    println!("float_reference: {:?}", my_value.borrow_float_reference()); // 3.14
                                                                          // Sets the value of float_data to 84.0
    my_value.with_mut(|fields| {
        **fields.float_reference = (**fields.int_reference as f32) * 2.0;
    });

    // We can hold on to this reference...
    println!("int_data: {:?}", my_value.borrow_int_data()); // 42
    println!("int_reference: {}", my_value.borrow_int_reference()); // 42
    println!("float_reference: {:?}", my_value.borrow_float_reference()); // 84.0

    // As long as the struct is still alive.
    drop(my_value);
    // This will cause an error!
    //println!("{:?}", my_value.borrow_int_data());
}

#[derive(Debug)]
struct PersonOption<'a> {
    name: String,
    nickname: Option<&'a str>,
}
impl<'a> PersonOption<'a> {
    fn new(name: String) -> Self {
        PersonOption {
            name,
            nickname: None,
        }
    }

    fn set_nickname<'b: 'a>(&'b mut self) {
        self.nickname = Some(&self.name);
    }
}

#[derive(Debug)]
struct PersonUnsafe {
    // 直接存储裸指针,不再受到Rust借用规则和生命周期的限制
    name: String,
    nickname: *const String,
}
impl PersonUnsafe {
    fn new(val: String) -> Self {
        PersonUnsafe {
            name: val,
            nickname: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let p: *const String = &self.name;
        self.nickname = p;
    }

    fn get_value(&self) -> &String {
        &self.name
    }

    fn get_nickname(&self) -> &String {
        assert_eq!(false, self.nickname.is_null());
        unsafe { &*(self.nickname) }
    }
}

/// NonNull: *mut T but non-zero and covariant
///
/// PhantomPinned: 未实现Unpin，即Pin
///
#[derive(Debug)]
struct PersonPin {
    fake_name: String,
    name: String,
    nickname: NonNull<String>,
    _pin: PhantomPinned,
}
impl PersonPin {
    // To ensure the data doesn't move when the function returns,
    // we place it in the heap where it will stay for the lifetime of the object,
    // and the only way to access it would be through a pointer to it.
    fn new(fake_name: String, name: String) -> Pin<Box<Self>> {
        let person = PersonPin {
            fake_name,
            name,
            // we only create the pointer once the data is in place
            // otherwise it will have already moved before we even started
            nickname: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(person);
        let nickname = NonNull::from(&boxed.name);
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).nickname = nickname;
        }
        boxed
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_nickname(&self) -> &String {
        unsafe { &*self.nickname.as_ptr() }
    }

    fn get_name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn get_fake_name_mut(&mut self) -> &mut String {
        &mut self.fake_name
    }
}

#[self_referencing]
struct MyStruct {
    int_data: i32,
    float_data: f32,
    #[borrows(int_data)]
    int_reference: &'this i32,
    #[borrows(mut float_data)]
    float_reference: &'this mut f32,
}
