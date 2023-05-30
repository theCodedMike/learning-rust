# 智能指针
## 1、Drop + Deref

## 2、Box

## 3、Rc VS. Arc

## 4、Weak

## 5、Cell VS. RefCell
| 属性\结构体   | Cell                                                                              | RefCell                                                                                                        |
|----------|-----------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| 定义       | A mutable memory location.                                                        | A mutable memory location with dynamically checked borrow rules                                                |
| 原理       | Cell<T> implements interior mutability by moving values in and out of the Cell<T> | To use references instead of values, one must use the RefCell<T> type, acquiring a write lock before mutating. |
| 适用环境     | 单线程                                                                               | 单线程                                                                                                            |
| 是否实现Send | 是                                                                                 | 是                                                                                                              |
| 是否实现Sync | 否                                                                                 | 否                                                                                                              |
| 方法       | 1、                                                                                | 1、                                                                                                             |


## 6、Send + Sync
| 属性\特征  | Send                                                    | Sync                                                                                                                                                  |
|--------|---------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
| 定义     | Types that can be transferred across thread boundaries. | Types for which it is safe to share references between threads. A type T is Sync if and only if &T is Send                                            |
| 标记特征?  | 是                                                       | 是                                                                                                                                                     |
| 未实现该特征 | *const T、*mut T、NonNull、Rc等等                            | Cell、RefCell、Rc                                                                                                                                       |
| 实现该特征  | &T(T: Sync)、Arc、Cell、RefCell、Mutex、RwLock等等             | primitive types like u8 and f64 are all Sync, and so are simple aggregate types containing them, like tuples, structs and enums.<br/>Arc、Mutex、RwLock |


## 7、Pin



