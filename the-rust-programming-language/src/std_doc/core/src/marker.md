#### `macro marker_impls`
#### `pub unsafe auto trait Send`
#### `impl<T: ?Sized> !Send for *const T`
#### `impl<T: ?Sized> !Send for *mut T`
#### `unsafe impl<T: Sync + ?Sized> Send for &T`
#### `pub trait Sized`
#### `pub trait Unsize<T: ?Sized>`
#### `pub trait StructuralPartialEq`
#### `pub trait StructuralEq`
#### `pub trait Copy: Clone`
#### `impl Copy for !`
#### `impl<T: ?Sized> Copy for &T`
#### `pub unsafe auto trait Sync`
#### `impl<T: ?Sized> !Sync for *const T`
#### `impl<T: ?Sized> !Sync for *mut T`
#### `pub struct PhantomData<T: ?Sized>`
#### `impl<T: ?Sized> Hash for PhantomData<T>`
#### `impl<T: ?Sized> cmp::PartialEq for PhantomData<T>`
#### `impl<T: ?Sized> cmp::Eq for PhantomData<T>`
#### `impl<T: ?Sized> cmp::PartialOrd for PhantomData<T>`
#### `impl<T: ?Sized> cmp::Ord for PhantomData<T>`
#### `impl<T: ?Sized> Copy for PhantomData<T>`
#### `impl<T: ?Sized> Clone for PhantomData<T>`
#### `impl<T: ?Sized> Default for PhantomData<T>`
#### `impl<T: ?Sized> StructuralPartialEq for PhantomData<T>`
#### `impl<T: ?Sized> StructuralEq for PhantomData<T>`
#### `pub trait DiscriminantKind`
#### `pub(crate) unsafe auto trait Freeze`
#### `impl<T: ?Sized> !Freeze for UnsafeCell<T>`
#### `pub auto trait Unpin`
#### `pub struct PhantomPinned`
#### `impl !Unpin for PhantomPinned`
#### `pub trait Destruct`
#### `pub trait Tuple`
#### `pub trait PointerLike`
#### `pub trait ConstParamTy: StructuralEq`
#### `pub macro ConstParamTy`
#### `pub trait FnPtr: Copy + Clone`



```rust
//! Primitive traits and types representing basic properties of types.
//! 译: 表示类型的基本性质的原始特征（trait）和类型。
//! 
//! Rust types can be classified in various useful ways according to
//! their intrinsic properties. These classifications are represented
//! as traits.
//! Rust类型可以根据其固有特性以各种有用的方式进行分类。而这些分类标准表现为特征（trait）。

#![stable(feature = "rust1", since = "1.0.0")]

use crate::cell::UnsafeCell;
use crate::cmp;
use crate::fmt::Debug;
use crate::hash::Hash;
use crate::hash::Hasher;

/// Implements a given marker trait for multiple types at the same time.
///
/// The basic syntax looks like this:
/// ```ignore private macro
/// marker_impls! { MarkerTrait for u8, i8 }
/// ```
/// You can also implement `unsafe` traits
/// ```ignore private macro
/// marker_impls! { unsafe MarkerTrait for u8, i8 }
/// ```
/// Add attributes to all impls:
/// ```ignore private macro
/// marker_impls! {
///     #[allow(lint)]
///     #[unstable(feature = "marker_trait", issue = "none")]
///     MarkerTrait for u8, i8
/// }
/// ```
/// And use generics:
/// ```ignore private macro
/// marker_impls! {
///     MarkerTrait for
///         u8, i8,
///         {T: ?Sized} *const T,
///         {T: ?Sized} *mut T,
///         {T: MarkerTrait} PhantomData<T>,
///         u32,
/// }
/// ```
#[unstable(feature = "internal_impls_macro", issue = "none")]
macro marker_impls {
( $(#[$($meta:tt)*])* $Trait:ident for $({$($bounds:tt)*})? $T:ty $(, $($rest:tt)*)? ) => {
        $(#[$($meta)*])* impl< $($($bounds)*)? > $Trait for $T {}
        marker_impls! { $(#[$($meta)*])* $Trait for $($($rest)*)? }
    },
( $(#[$($meta:tt)*])* $Trait:ident for ) => {},

( $(#[$($meta:tt)*])* unsafe $Trait:ident for $({$($bounds:tt)*})? $T:ty $(, $($rest:tt)*)? ) => {
        $(#[$($meta)*])* unsafe impl< $($($bounds)*)? > $Trait for $T {}
        marker_impls! { $(#[$($meta)*])* unsafe $Trait for $($($rest)*)? }
    },
( $(#[$($meta:tt)*])* unsafe $Trait:ident for ) => {},
}


/// Types that can be transferred across thread boundaries.  
/// 译: 可以跨线程传递的类型
///
/// This trait is automatically implemented when the compiler determines it's
/// appropriate.  
/// 译: 当编译器认为合适时，该特征就会被自动实现。
///
/// An example of a non-`Send` type is the reference-counting pointer
/// [`rc::Rc`][`Rc`]. If two threads attempt to clone [`Rc`]s that point to the same
/// reference-counted value, they might try to update the reference count at the
/// same time, which is [undefined behavior][ub] because [`Rc`] doesn't use atomic
/// operations. Its cousin [`sync::Arc`][arc] does use atomic operations (incurring
/// some overhead) and thus is `Send`.  
/// 译: 一个非 `Send` 类型的例子就是引用计数指针[`rc::Rc`][`Rc`]。
/// 如果两个线程试图克隆指向同一引用计数值的[`Rc`]，那它们就有可能会同时更新引用计数，而这是[未定义的行为][ub]，因为[`Rc`]并未使用原子操作。
/// 它的表兄弟[`sync::Arc`][arc]使用了原子操作（会产生一些开销），因此是 `Send` 类型。
///
/// See [the Nomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html) and the [`Sync`] trait for more details.  
/// 译: 更多详情，请参阅[the Nomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html)和[`Sync`]特征。
///
/// [`Rc`]: ../../std/rc/struct.Rc.html
/// [arc]: ../../std/sync/struct.Arc.html
/// [ub]: ../../reference/behavior-considered-undefined.html
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "Send")]
#[rustc_on_unimplemented(
    message = "`{Self}` cannot be sent between threads safely",
    label = "`{Self}` cannot be sent between threads safely"
)]
pub unsafe auto trait Send {
    // empty.
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Send for *const T {}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Send for *mut T {}


// Most instances arise automatically, but this instance is needed to link up `T: Sync` with
// `&T: Send` (and it also removes the unsound default instance `T Send` -> `&T: Send` that would
// otherwise exist).
// 译: 大多数实例都是自动产生的，但这个实例需要将 `T: Sync` 与 `&T: Send` 连接起来
// （它还删除了不可靠的 default 实例 `T Send` -> `&T: Send`，否则它就会存在）。
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Sync + ?Sized> Send for &T {}


/// Types with a constant size known at compile time.  
/// 译: 在编译时就能确定占用内存大小的类型。
///
/// All type parameters have an implicit bound of `Sized`. The special syntax
/// `?Sized` can be used to remove this bound if it's not appropriate.  
/// 译: 所有的类型参数都有一个隐式的特征约束 `Sized` 。
/// 如果不满足，就使用特殊的语法`?Sized`来绕开这一约束。
///
/// ```
/// # #![allow(dead_code)]
/// struct Foo<T>(T);
/// struct Bar<T: ?Sized>(T);
///
/// // struct FooUse(Foo<[i32]>); // error: Sized is not implemented for [i32]
/// struct BarUse(Bar<[i32]>); // OK
/// ```
///
/// The one exception is the implicit `Self` type of a trait. A trait does not
/// have an implicit `Sized` bound as this is incompatible with [trait object]s
/// where, by definition, the trait needs to work with all possible implementors,
/// and thus could be any size.  
/// 译: 唯一的例外是特征（trait）的隐式 `Self` 类型。特征并没有隐式的 `Sized` 约束，因为这与[特征对象][trait object]不兼容。
/// 根据特征对象的定义，特征需要与它所有可能的实现者协同工作，因此它可以是任意大小的。
///
/// Although Rust will let you bind `Sized` to a trait, you won't
/// be able to use it to form a trait object later:  
/// 译: 尽管Rust允许你将 `Sized` 绑定到某个特征，但是你不能使用它来形成特征对象：
///
/// ```
/// # #![allow(unused_variables)]
/// trait Foo { }
/// trait Bar: Sized { }
///
/// struct Impl;
/// impl Foo for Impl { }
/// impl Bar for Impl { }
///
/// let x: &dyn Foo = &Impl;    // OK
/// // let y: &dyn Bar = &Impl; // error: the trait `Bar` cannot
///                             // be made into an object
/// ```
///
/// [trait object]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
#[doc(alias = "?", alias = "?Sized")]
#[stable(feature = "rust1", since = "1.0.0")]
#[lang = "sized"]
#[rustc_on_unimplemented(
    message = "the size for values of type `{Self}` cannot be known at compilation time",
    label = "doesn't have a size known at compile-time"
)]
#[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
#[rustc_specialization_trait]
#[rustc_deny_explicit_impl]
#[rustc_coinductive]
pub trait Sized {
    // Empty.
}


/// Types that can be "unsized" to a dynamically-sized type.  
/// 译: 可作为动态大小类型（DST）的类型。
///
/// For example, the sized array type `[i8; 2]` implements `Unsize<[i8]>` and
/// `Unsize<dyn fmt::Debug>`.  
/// 译: 比如，作为确定大小的（sized）数组类型 `[i8; 2]` 实现了 `Unsize<[i8]>` 和 `Unsize<dyn fmt::Debug>` 。
///
/// All implementations of `Unsize` are provided automatically by the compiler.
/// Those implementations are:  
/// 译: 所有 `Unsize` 的实现，编译器都会自动提供。这些实现有：
///
/// - Arrays `[T; N]` implement `Unsize<[T]>`.
/// - Types implementing a trait `Trait` also implement `Unsize<dyn Trait>`.
/// - Structs `Foo<..., T, ...>` implement `Unsize<Foo<..., U, ...>>` if all of these conditions
///   are met:
///   - `T: Unsize<U>`.
///   - Only the last field of `Foo` has a type involving `T`.
///   - `Bar<T>: Unsize<Bar<U>>`, where `Bar<T>` stands for the actual type of that last field.
///
/// `Unsize` is used along with [`ops::CoerceUnsized`] to allow
/// "user-defined" containers such as [`Rc`] to contain dynamically-sized
/// types. See the [DST coercion RFC][RFC982] and [the nomicon entry on coercion][nomicon-coerce]
/// for more details.  
/// 译: `Unsize` 要和 [`ops::CoerceUnsized`] 搭配使用，允许 "用户定义" 的容器（如[`Rc`]）包含动态大小的类型。
/// 更多详情，请参阅[DST coercion RFC][RFC982]和[the nomicon entry on coercion][nomicon-coerce]。
///
/// [`ops::CoerceUnsized`]: crate::ops::CoerceUnsized
/// [`Rc`]: ../../std/rc/struct.Rc.html
/// [RFC982]: https://github.com/rust-lang/rfcs/blob/master/text/0982-dst-coercion.md
/// [nomicon-coerce]: https://doc.rust-lang.org/nomicon/coercions.html
#[unstable(feature = "unsize", issue = "18598")]
#[lang = "unsize"]
#[rustc_deny_explicit_impl]
pub trait Unsize<T: ?Sized> {
    // Empty.
}


/// Required trait for constants used in pattern matches.  
/// 译: 模式匹配中使用的常量的必备特征。
///
/// Any type that derives `PartialEq` automatically implements this trait,
/// *regardless* of whether its type-parameters implement `Eq`.  
/// 译: 任何派生了 `PartialEq` 的类型都会自动实现这一特征（trait），无论其类型参数是否实现了 `Eq`。
///
/// If a `const` item contains some type that does not implement this trait,
/// then that type either (1.) does not implement `PartialEq` (which means the
/// constant will not provide that comparison method, which code generation
/// assumes is available), or (2.) it implements *its own* version of
/// `PartialEq` (which we assume does not conform to a structural-equality
/// comparison).  
/// 译: 如果一个 `const` 项包含了某个未实现该特征的类型，那么该类型要么 
/// (1.) 没有实现 `PartialEq` （这意味着常量将不会提供代码生成所假定的比较方法），要么
/// (2.) 它实现了自己版本的 `PartialEq`（我们假定它不符合结构相等比较）。
///
/// In either of the two scenarios above, we reject usage of such a constant in
/// a pattern match.  
/// 译: 对于上述这两种情况，我们拒绝在模式匹配中使用这样的常量。
///
/// See also the [structural match RFC][RFC1445], and [issue 63438] which
/// motivated migrating from attribute-based design to this trait.  
/// 译: 另请参阅 [structural match RFC][RFC1445] 和 [issue 63438] ，
/// 它们促使我们从基于属性的设计迁移到这一特征。
///
/// [RFC1445]: https://github.com/rust-lang/rfcs/blob/master/text/1445-restrict-constants-in-patterns.md
/// [issue 63438]: https://github.com/rust-lang/rust/issues/63438
#[unstable(feature = "structural_match", issue = "31434")]
#[rustc_on_unimplemented(message = "the type `{Self}` does not `#[derive(PartialEq)]`")]
#[lang = "structural_peq"]
pub trait StructuralPartialEq {
    // Empty.
}


/// Required trait for constants used in pattern matches.  
/// 译: 模式匹配中使用的常量的必备特征。
///
/// Any type that derives `Eq` automatically implements this trait, *regardless*
/// of whether its type parameters implement `Eq`.  
/// 译: 任何派生了 `Eq` 的类型都会自动实现这一特征（trait），无论其类型参数是否实现了 `Eq`。
///
/// This is a hack to work around a limitation in our type system.  
/// 译: 这是一个能绕开类型系统限制的黑客程序。
///
/// # Background 背景
///
/// We want to require that types of consts used in pattern matches
/// have the attribute `#[derive(PartialEq, Eq)]`.  
/// 译: 我们希望模式匹配中使用的常量类型具有属性 `#[derive(PartialEq, Eq)]`。
///
/// In a more ideal world, we could check that requirement by just checking that
/// the given type implements both the `StructuralPartialEq` trait *and*
/// the `Eq` trait. However, you can have ADTs that *do* `derive(PartialEq, Eq)`,
/// and be a case that we want the compiler to accept, and yet the constant's
/// type fails to implement `Eq`.  
/// 译: 更理想的情况下，我们可以通过检查给定类型是否同时实现了 `StructuralPartialEq` 和 `Eq` 来检查这一要求。
/// 然而，你可能会有一些 ADT（抽象数据类型，确实派生了`derive(PartialEq, Eq)`），并且是我们希望编译器能接受的情况，
/// 但常量类型还是没有实现 `Eq`。
///
/// Namely, a case like this:  
/// 译: 即这样的示例：
///
/// ```rust
/// #[derive(PartialEq, Eq)]
/// struct Wrap<X>(X);
///
/// fn higher_order(_: &()) { }
///
/// const CFN: Wrap<fn(&())> = Wrap(higher_order);
///
/// fn main() {
///     match CFN {
///         CFN => {}
///         _ => {}
///     }
/// }
/// ```
///
/// (The problem in the above code is that `Wrap<fn(&())>` does not implement
/// `PartialEq`, nor `Eq`, because `for<'a> fn(&'a _)` does not implement those
/// traits.)  
/// 译: （上述代码的问题在于 `Wrap<fn(&())>` 没有实现 `PartialEq` 或 `Eq`，因为 `for<'a> fn(&'a _)` 没有实现这些特征。）
///
/// Therefore, we cannot rely on naive check for `StructuralPartialEq` and
/// mere `Eq`.  
/// 译: 因此，我们不能仅依赖于检查类型是否实现了 `StructuralPartialEq` 和 `Eq`。
///
/// As a hack to work around this, we use two separate traits injected by each
/// of the two derives (`#[derive(PartialEq)]` and `#[derive(Eq)]`) and check
/// that both of them are present as part of structural-match checking.  
/// 译: 为了解决这个问题，我们在两个派生（`#[derive(PartialEq)]` 和 `#[derive(Eq)]`）中分别使用两个独立的特征，
/// 并在结构匹配（structural-match）检查中检查这两个特征是否都存在。
#[unstable(feature = "structural_match", issue = "31434")]
#[rustc_on_unimplemented(message = "the type `{Self}` does not `#[derive(Eq)]`")]
#[lang = "structural_teq"]
pub trait StructuralEq {
    // Empty.
}


// FIXME: Remove special cases of these types from the compiler pattern checking code and always check `T: StructuralEq` instead
marker_impls! {
    #[unstable(feature = "structural_match", issue = "31434")]
    StructuralEq for
        usize, u8, u16, u32, u64, u128,
        isize, i8, i16, i32, i64, i128,
        bool,
        char,
        str /* Technically requires `[u8]: StructuralEq` */,
        {T, const N: usize} [T; N],
        {T} [T],
        {T: ?Sized} &T,
}


/// Types whose values can be duplicated simply by copying bits.  
/// 译: 只需复制比特就能复制其值的类型。
///
/// By default, variable bindings have 'move semantics.' In other
/// words:  
/// 译: 默认情况下，变量绑定具有 "移动语义"。换言之：
///
/// ```
/// #[derive(Debug)]
/// struct Foo;
///
/// let x = Foo;
///
/// let y = x;
///
/// // `x` has moved into `y`, and so cannot be used
///
/// // println!("{x:?}"); // error: use of moved value
/// ```
///
/// However, if a type implements `Copy`, it instead has 'copy semantics':  
/// 译: 然而，如果一个类型实现了 `Copy` 特征，那么它就拥有了 "复制语义"：
///
/// ```
/// // We can derive a `Copy` implementation. `Clone` is also required, as it's
/// // a supertrait of `Copy`.
/// #[derive(Debug, Copy, Clone)]
/// struct Foo;
///
/// let x = Foo;
///
/// let y = x;
///
/// // `y` is a copy of `x`
///
/// println!("{x:?}"); // A-OK!
/// ```
///
/// It's important to note that in these two examples, the only difference is whether you
/// are allowed to access `x` after the assignment. Under the hood, both a copy and a move
/// can result in bits being copied in memory, although this is sometimes optimized away.  
/// 译: 值得注意的是，在以上这两个示例中，唯一的区别在于是否允许在赋值后访问 `x`。
/// 在底层，复制（copy）和移动（move）都会导致比特位在内存中被复制，尽管这种情况有时会被优化掉。
///
/// ## How can I implement `Copy`?  如何实现 `Copy` 特征？
///
/// There are two ways to implement `Copy` on your type. The simplest is to use `derive`:  
/// 译: 有两种方式可以给你的类型实现 `Copy` 特征。最简单的方式是使用 `derive`：
///
/// ```
/// #[derive(Copy, Clone)]
/// struct MyStruct;
/// ```
///
/// You can also implement `Copy` and `Clone` manually:  
/// 译: 你也可以手动实现 `Copy` 和 `Clone`：
///
/// ```
/// struct MyStruct;
///
/// impl Copy for MyStruct { }
///
/// impl Clone for MyStruct {
///     fn clone(&self) -> MyStruct {
///         *self
///     }
/// }
/// ```
///
/// There is a small difference between the two: the `derive` strategy will also place a `Copy`
/// bound on type parameters, which isn't always desired.  
/// 译: 以上两种方式之间有一个小区别：`derive` 策略同时会对类型参数进行 `Copy` 约束，但这并不总是我们所希望的。
///
/// ## What's the difference between `Copy` and `Clone`? `Copy` 和 `Clone` 之间有什么区别呢？
///
/// Copies happen implicitly, for example as part of an assignment `y = x`. The behavior of
/// `Copy` is not overloadable; it is always a simple bit-wise copy.  
/// 译: 复制是隐式的，例如作为赋值 `y = x` 的一部分。`Copy` 这一行为不可重载；它总是一个简单的按位复制。
///
/// Cloning is an explicit action, `x.clone()`. The implementation of [`Clone`] can
/// provide any type-specific behavior necessary to duplicate values safely. For example,
/// the implementation of [`Clone`] for [`String`] needs to copy the pointed-to string
/// buffer in the heap. A simple bitwise copy of [`String`] values would merely copy the
/// pointer, leading to a double free down the line. For this reason, [`String`] is [`Clone`]
/// but not `Copy`.  
/// 译: 克隆是一个显式操作，即 `x.clone()`。[`Clone`] 的实现可以提供安全复制值所需的任何特定类型行为。例如，[`String`] 的 [`Clone`]实现需要在堆中复制指向的字符串缓冲区。对 [``String``] 值进行简单的按位复制只会复制
/// 指针，从而导致重复释放。因此，[`String``] 是 [`Clone`]，而不是 `Copy`。
///
/// [`Clone`] is a supertrait of `Copy`, so everything which is `Copy` must also implement
/// [`Clone`]. If a type is `Copy` then its [`Clone`] implementation only needs to return `*self`
/// (see the example above).
///
/// ## When can my type be `Copy`?
///
/// A type can implement `Copy` if all of its components implement `Copy`. For example, this
/// struct can be `Copy`:
///
/// ```
/// # #[allow(dead_code)]
/// #[derive(Copy, Clone)]
/// struct Point {
///    x: i32,
///    y: i32,
/// }
/// ```
///
/// A struct can be `Copy`, and [`i32`] is `Copy`, therefore `Point` is eligible to be `Copy`.
/// By contrast, consider
///
/// ```
/// # #![allow(dead_code)]
/// # struct Point;
/// struct PointList {
///     points: Vec<Point>,
/// }
/// ```
///
/// The struct `PointList` cannot implement `Copy`, because [`Vec<T>`] is not `Copy`. If we
/// attempt to derive a `Copy` implementation, we'll get an error:
///
/// ```text
/// the trait `Copy` cannot be implemented for this type; field `points` does not implement `Copy`
/// ```
///
/// Shared references (`&T`) are also `Copy`, so a type can be `Copy`, even when it holds
/// shared references of types `T` that are *not* `Copy`. Consider the following struct,
/// which can implement `Copy`, because it only holds a *shared reference* to our non-`Copy`
/// type `PointList` from above:
///
/// ```
/// # #![allow(dead_code)]
/// # struct PointList;
/// #[derive(Copy, Clone)]
/// struct PointListWrapper<'a> {
///     point_list_ref: &'a PointList,
/// }
/// ```
///
/// ## When *can't* my type be `Copy`?
///
/// Some types can't be copied safely. For example, copying `&mut T` would create an aliased
/// mutable reference. Copying [`String`] would duplicate responsibility for managing the
/// [`String`]'s buffer, leading to a double free.
///
/// Generalizing the latter case, any type implementing [`Drop`] can't be `Copy`, because it's
/// managing some resource besides its own [`size_of::<T>`] bytes.
///
/// If you try to implement `Copy` on a struct or enum containing non-`Copy` data, you will get
/// the error [E0204].
///
/// [E0204]: ../../error_codes/E0204.html
///
/// ## When *should* my type be `Copy`?
///
/// Generally speaking, if your type _can_ implement `Copy`, it should. Keep in mind, though,
/// that implementing `Copy` is part of the public API of your type. If the type might become
/// non-`Copy` in the future, it could be prudent to omit the `Copy` implementation now, to
/// avoid a breaking API change.
///
/// ## Additional implementors
///
/// In addition to the [implementors listed below][impls],
/// the following types also implement `Copy`:
///
/// * Function item types (i.e., the distinct types defined for each function)
/// * Function pointer types (e.g., `fn() -> i32`)
/// * Closure types, if they capture no value from the environment
///   or if all such captured values implement `Copy` themselves.
///   Note that variables captured by shared reference always implement `Copy`
///   (even if the referent doesn't),
///   while variables captured by mutable reference never implement `Copy`.
///
/// [`Vec<T>`]: ../../std/vec/struct.Vec.html
/// [`String`]: ../../std/string/struct.String.html
/// [`size_of::<T>`]: crate::mem::size_of
/// [impls]: #implementors
#[stable(feature = "rust1", since = "1.0.0")]
#[lang = "copy"]
// FIXME(matthewjasper) This allows copying a type that doesn't implement
// `Copy` because of unsatisfied lifetime bounds (copying `A<'_>` when only
// `A<'static>: Copy` and `A<'_>: Clone`).
// We have this attribute here for now only because there are quite a few
// existing specializations on `Copy` that already exist in the standard
// library, and there's no way to safely have this behavior right now.
#[rustc_unsafe_specialization_marker]
#[rustc_diagnostic_item = "Copy"]
pub trait Copy: Clone {
    // Empty.
}


/// Derive macro generating an impl of the trait `Copy`.
#[rustc_builtin_macro]
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow_internal_unstable(core_intrinsics, derive_clone_copy)]
pub macro Copy($item:item) {
/* compiler built-in */
}


// Implementations of `Copy` for primitive types.
//
// Implementations that cannot be described in Rust
// are implemented in `traits::SelectionContext::copy_clone_conditions()`
// in `rustc_trait_selection`.
marker_impls! {
    #[stable(feature = "rust1", since = "1.0.0")]
    Copy for
        usize, u8, u16, u32, u64, u128,
        isize, i8, i16, i32, i64, i128,
        f32, f64,
        bool, char,
        {T: ?Sized} *const T,
        {T: ?Sized} *mut T,

}


#[unstable(feature = "never_type", issue = "35121")]
impl Copy for ! {}


/// Shared references can be copied, but mutable references *cannot*!
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Copy for &T {}


/// Types for which it is safe to share references between threads.
///
/// This trait is automatically implemented when the compiler determines
/// it's appropriate.
///
/// The precise definition is: a type `T` is [`Sync`] if and only if `&T` is
/// [`Send`]. In other words, if there is no possibility of
/// [undefined behavior][ub] (including data races) when passing
/// `&T` references between threads.
///
/// As one would expect, primitive types like [`u8`] and [`f64`]
/// are all [`Sync`], and so are simple aggregate types containing them,
/// like tuples, structs and enums. More examples of basic [`Sync`]
/// types include "immutable" types like `&T`, and those with simple
/// inherited mutability, such as [`Box<T>`][box], [`Vec<T>`][vec] and
/// most other collection types. (Generic parameters need to be [`Sync`]
/// for their container to be [`Sync`].)
///
/// A somewhat surprising consequence of the definition is that `&mut T`
/// is `Sync` (if `T` is `Sync`) even though it seems like that might
/// provide unsynchronized mutation. The trick is that a mutable
/// reference behind a shared reference (that is, `& &mut T`)
/// becomes read-only, as if it were a `& &T`. Hence there is no risk
/// of a data race.
///
/// A shorter overview of how [`Sync`] and [`Send`] relate to referencing:
/// * `&T` is [`Send`] if and only if `T` is [`Sync`]
/// * `&mut T` is [`Send`] if and only if `T` is [`Send`]
/// * `&T` and `&mut T` are [`Sync`] if and only if `T` is [`Sync`]
///
/// Types that are not `Sync` are those that have "interior
/// mutability" in a non-thread-safe form, such as [`Cell`][cell]
/// and [`RefCell`][refcell]. These types allow for mutation of
/// their contents even through an immutable, shared reference. For
/// example the `set` method on [`Cell<T>`][cell] takes `&self`, so it requires
/// only a shared reference [`&Cell<T>`][cell]. The method performs no
/// synchronization, thus [`Cell`][cell] cannot be `Sync`.
///
/// Another example of a non-`Sync` type is the reference-counting
/// pointer [`Rc`][rc]. Given any reference [`&Rc<T>`][rc], you can clone
/// a new [`Rc<T>`][rc], modifying the reference counts in a non-atomic way.
///
/// For cases when one does need thread-safe interior mutability,
/// Rust provides [atomic data types], as well as explicit locking via
/// [`sync::Mutex`][mutex] and [`sync::RwLock`][rwlock]. These types
/// ensure that any mutation cannot cause data races, hence the types
/// are `Sync`. Likewise, [`sync::Arc`][arc] provides a thread-safe
/// analogue of [`Rc`][rc].
///
/// Any types with interior mutability must also use the
/// [`cell::UnsafeCell`][unsafecell] wrapper around the value(s) which
/// can be mutated through a shared reference. Failing to doing this is
/// [undefined behavior][ub]. For example, [`transmute`][transmute]-ing
/// from `&T` to `&mut T` is invalid.
///
/// See [the Nomicon][nomicon-send-and-sync] for more details about `Sync`.
///
/// [box]: ../../std/boxed/struct.Box.html
/// [vec]: ../../std/vec/struct.Vec.html
/// [cell]: crate::cell::Cell
/// [refcell]: crate::cell::RefCell
/// [rc]: ../../std/rc/struct.Rc.html
/// [arc]: ../../std/sync/struct.Arc.html
/// [atomic data types]: crate::sync::atomic
/// [mutex]: ../../std/sync/struct.Mutex.html
/// [rwlock]: ../../std/sync/struct.RwLock.html
/// [unsafecell]: crate::cell::UnsafeCell
/// [ub]: ../../reference/behavior-considered-undefined.html
/// [transmute]: crate::mem::transmute
/// [nomicon-send-and-sync]: ../../nomicon/send-and-sync.html
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "Sync")]
#[lang = "sync"]
#[rustc_on_unimplemented(
    on(
        _Self = "std::cell::OnceCell<T>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::OnceLock` instead"
    ),
    on(
        _Self = "std::cell::Cell<u8>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicU8` instead",
    ),
    on(
        _Self = "std::cell::Cell<u16>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicU16` instead",
    ),
    on(
        _Self = "std::cell::Cell<u32>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicU32` instead",
    ),
    on(
        _Self = "std::cell::Cell<u64>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicU64` instead",
    ),
    on(
        _Self = "std::cell::Cell<usize>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicUsize` instead",
    ),
    on(
        _Self = "std::cell::Cell<i8>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicI8` instead",
    ),
    on(
        _Self = "std::cell::Cell<i16>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicI16` instead",
    ),
    on(
        _Self = "std::cell::Cell<i32>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicI32` instead",
    ),
    on(
        _Self = "std::cell::Cell<i64>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicI64` instead",
    ),
    on(
        _Self = "std::cell::Cell<isize>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicIsize` instead",
    ),
    on(
        _Self = "std::cell::Cell<bool>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicBool` instead",
    ),
    on(
        _Self = "std::cell::Cell<T>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock`",
    ),
    on(
        _Self = "std::cell::RefCell<T>",
        note = "if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead",
    ),
    message = "`{Self}` cannot be shared between threads safely",
    label = "`{Self}` cannot be shared between threads safely"
)]
pub unsafe auto trait Sync {
    // FIXME(estebank): once support to add notes in `rustc_on_unimplemented`
    // lands in beta, and it has been extended to check whether a closure is
    // anywhere in the requirement chain, extend it as such (#48534):
    // ```
    // on(
    //     closure,
    //     note="`{Self}` cannot be shared safely, consider marking the closure `move`"
    // ),
    // ```

    // Empty
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for *const T {}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for *mut T {}


/// Zero-sized type used to mark things that "act like" they own a `T`.
///
/// Adding a `PhantomData<T>` field to your type tells the compiler that your
/// type acts as though it stores a value of type `T`, even though it doesn't
/// really. This information is used when computing certain safety properties.
///
/// For a more in-depth explanation of how to use `PhantomData<T>`, please see
/// [the Nomicon](../../nomicon/phantom-data.html).
///
/// # A ghastly note 👻👻👻
///
/// Though they both have scary names, `PhantomData` and 'phantom types' are
/// related, but not identical. A phantom type parameter is simply a type
/// parameter which is never used. In Rust, this often causes the compiler to
/// complain, and the solution is to add a "dummy" use by way of `PhantomData`.
///
/// # Examples
///
/// ## Unused lifetime parameters
///
/// Perhaps the most common use case for `PhantomData` is a struct that has an
/// unused lifetime parameter, typically as part of some unsafe code. For
/// example, here is a struct `Slice` that has two pointers of type `*const T`,
/// presumably pointing into an array somewhere:
///
/// ```compile_fail,E0392
/// struct Slice<'a, T> {
///     start: *const T,
///     end: *const T,
/// }
/// ```
///
/// The intention is that the underlying data is only valid for the
/// lifetime `'a`, so `Slice` should not outlive `'a`. However, this
/// intent is not expressed in the code, since there are no uses of
/// the lifetime `'a` and hence it is not clear what data it applies
/// to. We can correct this by telling the compiler to act *as if* the
/// `Slice` struct contained a reference `&'a T`:
///
/// ```
/// use std::marker::PhantomData;
///
/// # #[allow(dead_code)]
/// struct Slice<'a, T> {
///     start: *const T,
///     end: *const T,
///     phantom: PhantomData<&'a T>,
/// }
/// ```
///
/// This also in turn infers the lifetime bound `T: 'a`, indicating
/// that any references in `T` are valid over the lifetime `'a`.
///
/// When initializing a `Slice` you simply provide the value
/// `PhantomData` for the field `phantom`:
///
/// ```
/// # #![allow(dead_code)]
/// # use std::marker::PhantomData;
/// # struct Slice<'a, T> {
/// #     start: *const T,
/// #     end: *const T,
/// #     phantom: PhantomData<&'a T>,
/// # }
/// fn borrow_vec<T>(vec: &Vec<T>) -> Slice<'_, T> {
///     let ptr = vec.as_ptr();
///     Slice {
///         start: ptr,
///         end: unsafe { ptr.add(vec.len()) },
///         phantom: PhantomData,
///     }
/// }
/// ```
///
/// ## Unused type parameters
///
/// It sometimes happens that you have unused type parameters which
/// indicate what type of data a struct is "tied" to, even though that
/// data is not actually found in the struct itself. Here is an
/// example where this arises with [FFI]. The foreign interface uses
/// handles of type `*mut ()` to refer to Rust values of different
/// types. We track the Rust type using a phantom type parameter on
/// the struct `ExternalResource` which wraps a handle.
///
/// [FFI]: ../../book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
///
/// ```
/// # #![allow(dead_code)]
/// # trait ResType { }
/// # struct ParamType;
/// # mod foreign_lib {
/// #     pub fn new(_: usize) -> *mut () { 42 as *mut () }
/// #     pub fn do_stuff(_: *mut (), _: usize) {}
/// # }
/// # fn convert_params(_: ParamType) -> usize { 42 }
/// use std::marker::PhantomData;
/// use std::mem;
///
/// struct ExternalResource<R> {
///    resource_handle: *mut (),
///    resource_type: PhantomData<R>,
/// }
///
/// impl<R: ResType> ExternalResource<R> {
///     fn new() -> Self {
///         let size_of_res = mem::size_of::<R>();
///         Self {
///             resource_handle: foreign_lib::new(size_of_res),
///             resource_type: PhantomData,
///         }
///     }
///
///     fn do_stuff(&self, param: ParamType) {
///         let foreign_params = convert_params(param);
///         foreign_lib::do_stuff(self.resource_handle, foreign_params);
///     }
/// }
/// ```
///
/// ## Ownership and the drop check
///
/// The exact interaction of `PhantomData` with drop check **may change in the future**.
///
/// Currently, adding a field of type `PhantomData<T>` indicates that your type *owns* data of type
/// `T` in very rare circumstances. This in turn has effects on the Rust compiler's [drop check]
/// analysis. For the exact rules, see the [drop check] documentation.
///
/// ## Layout
///
/// For all `T`, the following are guaranteed:
/// * `size_of::<PhantomData<T>>() == 0`
/// * `align_of::<PhantomData<T>>() == 1`
///
/// [drop check]: Drop#drop-check
#[lang = "phantom_data"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct PhantomData<T: ?Sized>;


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Hash for PhantomData<T> {
    #[inline]
    fn hash<H: Hasher>(&self, _: &mut H) {}
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> cmp::PartialEq for PhantomData<T> {
    fn eq(&self, _other: &PhantomData<T>) -> bool {
        true
    }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> cmp::Eq for PhantomData<T> {}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> cmp::PartialOrd for PhantomData<T> {
    fn partial_cmp(&self, _other: &PhantomData<T>) -> Option<cmp::Ordering> {
        Option::Some(cmp::Ordering::Equal)
    }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> cmp::Ord for PhantomData<T> {
    fn cmp(&self, _other: &PhantomData<T>) -> cmp::Ordering {
        cmp::Ordering::Equal
    }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Copy for PhantomData<T> {}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Clone for PhantomData<T> {
    fn clone(&self) -> Self {
        Self
    }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Default for PhantomData<T> {
    fn default() -> Self {
        Self
    }
}


#[unstable(feature = "structural_match", issue = "31434")]
impl<T: ?Sized> StructuralPartialEq for PhantomData<T> {}


#[unstable(feature = "structural_match", issue = "31434")]
impl<T: ?Sized> StructuralEq for PhantomData<T> {}


/// Compiler-internal trait used to indicate the type of enum discriminants.
///
/// This trait is automatically implemented for every type and does not add any
/// guarantees to [`mem::Discriminant`]. It is **undefined behavior** to transmute
/// between `DiscriminantKind::Discriminant` and `mem::Discriminant`.
///
/// [`mem::Discriminant`]: crate::mem::Discriminant
#[unstable(
    feature = "discriminant_kind",
    issue = "none",
    reason = "this trait is unlikely to ever be stabilized, use `mem::discriminant` instead"
)]
#[lang = "discriminant_kind"]
#[rustc_deny_explicit_impl]
pub trait DiscriminantKind {
    /// The type of the discriminant, which must satisfy the trait
    /// bounds required by `mem::Discriminant`.
    #[lang = "discriminant_type"]
    type Discriminant: Clone + Copy + Debug + Eq + PartialEq + Hash + Send + Sync + Unpin;
}


/// Compiler-internal trait used to determine whether a type contains
/// any `UnsafeCell` internally, but not through an indirection.
/// This affects, for example, whether a `static` of that type is
/// placed in read-only static memory or writable static memory.
#[lang = "freeze"]
pub(crate) unsafe auto trait Freeze {}


impl<T: ?Sized> !Freeze for UnsafeCell<T> {}


marker_impls! {
    unsafe Freeze for
        {T: ?Sized} PhantomData<T>,
        {T: ?Sized} *const T,
        {T: ?Sized} *mut T,
        {T: ?Sized} &T,
        {T: ?Sized} &mut T,
}


/// Types that can be safely moved after being pinned.
///
/// Rust itself has no notion of immovable types, and considers moves (e.g.,
/// through assignment or [`mem::replace`]) to always be safe.
///
/// The [`Pin`][Pin] type is used instead to prevent moves through the type
/// system. Pointers `P<T>` wrapped in the [`Pin<P<T>>`][Pin] wrapper can't be
/// moved out of. See the [`pin` module] documentation for more information on
/// pinning.
///
/// Implementing the `Unpin` trait for `T` lifts the restrictions of pinning off
/// the type, which then allows moving `T` out of [`Pin<P<T>>`][Pin] with
/// functions such as [`mem::replace`].
///
/// `Unpin` has no consequence at all for non-pinned data. In particular,
/// [`mem::replace`] happily moves `!Unpin` data (it works for any `&mut T`, not
/// just when `T: Unpin`). However, you cannot use [`mem::replace`] on data
/// wrapped inside a [`Pin<P<T>>`][Pin] because you cannot get the `&mut T` you
/// need for that, and *that* is what makes this system work.
///
/// So this, for example, can only be done on types implementing `Unpin`:
///
/// ```rust
/// # #![allow(unused_must_use)]
/// use std::mem;
/// use std::pin::Pin;
///
/// let mut string = "this".to_string();
/// let mut pinned_string = Pin::new(&mut string);
///
/// // We need a mutable reference to call `mem::replace`.
/// // We can obtain such a reference by (implicitly) invoking `Pin::deref_mut`,
/// // but that is only possible because `String` implements `Unpin`.
/// mem::replace(&mut *pinned_string, "other".to_string());
/// ```
///
/// This trait is automatically implemented for almost every type.
///
/// [`mem::replace`]: crate::mem::replace
/// [Pin]: crate::pin::Pin
/// [`pin` module]: crate::pin
#[stable(feature = "pin", since = "1.33.0")]
#[rustc_on_unimplemented(
    note = "consider using the `pin!` macro\nconsider using `Box::pin` if you need to access the pinned value outside of the current scope",
    message = "`{Self}` cannot be unpinned"
)]
#[lang = "unpin"]
pub auto trait Unpin {}


/// A marker type which does not implement `Unpin`.
///
/// If a type contains a `PhantomPinned`, it will not implement `Unpin` by default.
#[stable(feature = "pin", since = "1.33.0")]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhantomPinned;


#[stable(feature = "pin", since = "1.33.0")]
impl !Unpin for PhantomPinned {}


marker_impls! {
    #[stable(feature = "pin", since = "1.33.0")]
    Unpin for
        {T: ?Sized} &T,
        {T: ?Sized} &mut T,
}


marker_impls! {
    #[stable(feature = "pin_raw", since = "1.38.0")]
    Unpin for
        {T: ?Sized} *const T,
        {T: ?Sized} *mut T,
}


/// A marker for types that can be dropped.
///
/// This should be used for `~const` bounds,
/// as non-const bounds will always hold for every type.
#[unstable(feature = "const_trait_impl", issue = "67792")]
#[lang = "destruct"]
#[rustc_on_unimplemented(message = "can't drop `{Self}`", append_const_msg)]
#[rustc_deny_explicit_impl]
#[const_trait]
pub trait Destruct {}


/// A marker for tuple types.
///
/// The implementation of this trait is built-in and cannot be implemented
/// for any user type.
#[unstable(feature = "tuple_trait", issue = "none")]
#[lang = "tuple_trait"]
#[rustc_on_unimplemented(message = "`{Self}` is not a tuple")]
#[rustc_deny_explicit_impl]
pub trait Tuple {}


/// A marker for pointer-like types.
///
/// All types that have the same size and alignment as a `usize` or
/// `*const ()` automatically implement this trait.
#[unstable(feature = "pointer_like_trait", issue = "none")]
#[lang = "pointer_like"]
#[rustc_on_unimplemented(
    message = "`{Self}` needs to have the same ABI as a pointer",
    label = "`{Self}` needs to be a pointer-like type"
)]
pub trait PointerLike {}


/// A marker for types which can be used as types of `const` generic parameters.
#[cfg_attr(not(bootstrap), lang = "const_param_ty")]
#[unstable(feature = "adt_const_params", issue = "95174")]
#[rustc_on_unimplemented(message = "`{Self}` can't be used as a const parameter type")]
pub trait ConstParamTy: StructuralEq {}


/// Derive macro generating an impl of the trait `ConstParamTy`.
#[rustc_builtin_macro]
#[unstable(feature = "adt_const_params", issue = "95174")]
#[cfg(not(bootstrap))]
pub macro ConstParamTy($item:item) {
/* compiler built-in */
}


// FIXME(generic_const_parameter_types): handle `ty::FnDef`/`ty::Closure`
// FIXME(generic_const_parameter_types): handle `ty::Tuple`
marker_impls! {
    #[unstable(feature = "adt_const_params", issue = "95174")]
    ConstParamTy for
        usize, u8, u16, u32, u64, u128,
        isize, i8, i16, i32, i64, i128,
        bool,
        char,
        str /* Technically requires `[u8]: ConstParamTy` */,
        {T: ConstParamTy, const N: usize} [T; N],
        {T: ConstParamTy} [T],
        {T: ?Sized + ConstParamTy} &T,
}


/// A common trait implemented by all function pointers.
#[unstable(
    feature = "fn_ptr_trait",
    issue = "none",
    reason = "internal trait for implementing various traits for all function pointers"
)]
#[lang = "fn_ptr_trait"]
#[rustc_deny_explicit_impl]
pub trait FnPtr: Copy + Clone {
    /// Returns the address of the function pointer.
    #[lang = "fn_ptr_addr"]
    fn addr(self) -> *const ();
}

```